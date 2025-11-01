//! Module `substrate` — couche technique du Void Engine
//!
//! Gère les interfaces bas-niveau : rendu graphique, GPU, device context, et configuration système.
//! Ce module constitue la base matérielle sur laquelle reposent la simulation, la physique, et les couches supérieures.

use std::sync::Arc;

use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use tokio::sync::oneshot;
use tracing::{debug, error, info, warn};
use wgpu::{
    Backends, Device, Instance, InstanceDescriptor, InstanceFlags, PowerPreference,
    Queue, RequestAdapterOptions, Surface,
    TextureFormat,
};

/// Structure représentant le contexte GPU global du Void Engine.
#[derive(Resource, Clone)]
pub struct GpuContext {
    pub adapter_name: String,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub surface_format: Option<TextureFormat>,
    pub surface: Option<Arc<Surface<'static>>>,
}

#[derive(Debug)]
pub enum GpuInitError {
    NoAdapter,
    RequestDevice(String),
}

impl std::fmt::Display for GpuInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoAdapter => write!(f, "no compatible GPU adapter available"),
            Self::RequestDevice(err) => write!(f, "failed to request device: {err}"),
        }
    }
}

impl std::error::Error for GpuInitError {}

impl From<wgpu::RequestDeviceError> for GpuInitError {
    fn from(value: wgpu::RequestDeviceError) -> Self {
        Self::RequestDevice(value.to_string())
    }
}

impl GpuContext {
    /// Initialise le contexte GPU (backend auto-détecté).
    pub async fn initialize() -> Result<Self, GpuInitError> {
        info!(target: "substrate", "starting GPU initialization");

        let instance = Instance::new(&InstanceDescriptor {
            flags: InstanceFlags::default(),
            backends: Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .map_err(|_| GpuInitError::NoAdapter)?;

        let adapter_info = adapter.get_info();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;

        let surface: Option<Arc<Surface<'static>>> = None;

        let surface_format = None;

        if surface.is_some() && surface_format.is_some() {
            debug!(target: "substrate", "headless surface configured");
        } else {
            warn!(target: "substrate", "headless surface unavailable; continuing without surface");
        }

        info!(target: "substrate", adapter = %adapter_info.name, "GPU initialization complete");

        Ok(Self {
            adapter_name: adapter_info.name,
            device: Arc::new(device),
            queue: Arc::new(queue),
            surface_format,
            surface,
        })
    }
}

#[allow(dead_code)]
fn configure_surface(_surface: &Surface<'static>, _device: &Device, _format: TextureFormat) {
    // No longer used due to removed surface creation
}

#[derive(Resource)]
struct PendingGpuInit(oneshot::Receiver<Result<GpuContext, GpuInitError>>);

fn start_gpu_initialization(mut commands: Commands) {
    let (sender, receiver) = oneshot::channel();

    IoTaskPool::get().spawn(async move {
        let result = GpuContext::initialize().await;
        let _ = sender.send(result);
    }).detach();

    commands.insert_resource(PendingGpuInit(receiver));
    info!(target: "substrate", "spawned asynchronous GPU task");
}

fn poll_gpu_initialization(mut commands: Commands, pending: Option<ResMut<PendingGpuInit>>) {
    let Some(mut pending) = pending else {
        return;
    };

    match pending.0.try_recv() {
        Ok(result) => match result {
            Ok(context) => {
                info!(target: "substrate", adapter = %context.adapter_name, "GPU context ready");
                commands.insert_resource(context);
                commands.remove_resource::<PendingGpuInit>();
            }
            Err(err) => {
                error!(target: "substrate", ?err, "failed to initialize GPU context");
                commands.remove_resource::<PendingGpuInit>();
            }
        },
        Err(err) => {
            error!(target: "substrate", ?err, "GPU initialization channel closed unexpectedly");
            commands.remove_resource::<PendingGpuInit>();
        }
    }
}

/// Initialise le module `substrate`.
pub fn init(app: &mut App) {
    info!(target: "substrate", "initializing GPU substrate module");

    app.add_systems(Startup, start_gpu_initialization)
        .add_systems(Update, poll_gpu_initialization);
}

/// Fonction de debug — affiche l’état du module `substrate`.
pub fn debug_info() {
    info!(target: "substrate", "GPU substrate active");
}
