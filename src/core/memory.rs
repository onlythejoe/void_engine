use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, error, info};

const DEFAULT_MEMORY_PATH: &str = "void_state.json";
const MEMORY_TARGET: &str = "core::memory";

/// Persistent memory buffer shared across Void Engine subsystems.
#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct MemoryField {
    history: Vec<Value>,
    max_snapshots: usize,
    #[serde(default = "default_path")]
    base_path: PathBuf,
    #[serde(skip)]
    writes_since_rotation: usize,
}

fn default_path() -> PathBuf {
    PathBuf::from(DEFAULT_MEMORY_PATH)
}

impl Default for MemoryField {
    fn default() -> Self {
        Self::new(512)
    }
}

impl MemoryField {
    /// Create a new memory field capped by `max_snapshots`.
    pub fn new(max_snapshots: usize) -> Self {
        Self {
            history: Vec::with_capacity(max_snapshots),
            max_snapshots: max_snapshots.max(1),
            base_path: default_path(),
            writes_since_rotation: 0,
        }
    }

    /// Record a JSON snapshot and persist it to disk.
    pub fn record(&mut self, snapshot: Value) {
        self.history.push(snapshot.clone());
        if self.history.len() > self.max_snapshots {
            self.history.remove(0);
        }

        if let Err(err) = self.append_snapshot(&snapshot) {
            error!(target: MEMORY_TARGET, ?err, "failed to append memory snapshot");
        }

        self.writes_since_rotation += 1;
        if self.writes_since_rotation >= self.max_snapshots {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let archive_name = format!(
                "{}-{}.json",
                self.base_path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("void_state"),
                timestamp
            );
            self.rotate(&archive_name);
        }
    }

    /// Flush the in-memory buffer to disk, replacing existing content.
    pub fn flush(&mut self) {
        if let Err(err) = self.write_all() {
            error!(target: MEMORY_TARGET, ?err, "failed to flush memory buffer");
        } else {
            debug!(target: MEMORY_TARGET, "memory buffer flushed to disk");
        }
    }

    /// Rotate the current memory file into `path`, clearing buffered snapshots.
    pub fn rotate(&mut self, path: &str) {
        if let Err(err) = self.rotate_internal(path) {
            error!(target: MEMORY_TARGET, ?err, "failed to rotate memory log");
        } else {
            info!(target: MEMORY_TARGET, archive = %path, "memory log rotated");
            self.writes_since_rotation = 0;
        }
    }

    /// Load an existing memory field from `path`.
    pub fn from_file(path: &str) -> Option<Self> {
        let file = File::open(path).ok()?;
        let reader = BufReader::new(file);
        let mut field = Self {
            history: Vec::new(),
            max_snapshots: 512,
            base_path: PathBuf::from(path),
            writes_since_rotation: 0,
        };

        for line in reader.lines() {
            match line {
                Ok(line) if !line.trim().is_empty() => match serde_json::from_str::<Value>(&line) {
                    Ok(value) => field.history.push(value),
                    Err(err) => {
                        error!(target: MEMORY_TARGET, ?err, "failed to parse snapshot from file")
                    }
                },
                Ok(_) => {}
                Err(err) => {
                    error!(target: MEMORY_TARGET, ?err, "failed to read snapshot line");
                    return None;
                }
            }
        }

        field.writes_since_rotation = field.history.len().min(field.max_snapshots);
        Some(field)
    }

    /// Returns the total number of cached snapshots.
    pub fn len(&self) -> usize {
        self.history.len()
    }

    /// Returns the most recent snapshot.
    pub fn latest(&self) -> Option<&Value> {
        self.history.last()
    }

    /// Computes the average of a numeric field over the last `window` snapshots.
    pub fn average(&self, key: &str, window: usize) -> Option<f32> {
        let window = window.max(1);
        let start = self.history.len().saturating_sub(window);
        let mut sum = 0.0f32;
        let mut count = 0f32;

        for snapshot in self.history.iter().skip(start) {
            if let Some(value) = snapshot.get(key) {
                if let Some(num) = value.as_f64() {
                    sum += num as f32;
                    count += 1.0;
                }
            }
        }

        if count > 0.0 {
            Some(sum / count)
        } else {
            None
        }
    }

    /// Estimates the linear trend (difference) for a numeric field across the last `window` snapshots.
    pub fn trend(&self, key: &str, window: usize) -> Option<f32> {
        if window < 2 || self.history.len() < 2 {
            return None;
        }

        let window = window.min(self.history.len());
        let start = self.history.len() - window;
        let first = self.history.get(start)?.get(key)?.as_f64()? as f32;
        let last = self.history.last()?.get(key)?.as_f64()? as f32;
        Some(last - first)
    }

    fn append_snapshot(&self, snapshot: &Value) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.base_path)?;
        let json = serde_json::to_string(snapshot)?;
        writeln!(file, "{}", json)?;
        Ok(())
    }

    fn write_all(&self) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.base_path)?;
        for snapshot in &self.history {
            let json = serde_json::to_string(snapshot)?;
            writeln!(file, "{}", json)?;
        }
        Ok(())
    }

    fn rotate_internal(&mut self, path: &str) -> std::io::Result<()> {
        let archive = PathBuf::from(path);
        if let Some(parent) = archive.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        if Path::new(&self.base_path).exists() {
            fs::rename(&self.base_path, &archive)?;
        }

        self.history.clear();

        // Ensure we start fresh by truncating the base file.
        File::create(&self.base_path)?;
        Ok(())
    }
}
