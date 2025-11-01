use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::OpenOptions;
use std::io::Write;

// Module `core` — Noyau central du moteur Void Engine
//
// Cette couche gère les fondations du moteur : initialisation,
// configuration, boucle principale et état global.
// Tous les autres systèmes (structure, dynamics, rendering, etc.)
// s’y connectent à travers ce cœur.

/// Structure principale du moteur.
#[derive(Resource, Default)]
pub struct Engine {
    pub is_running: bool,
}

/// Champ de mémoire du moteur — enregistre l'évolution interne du Void.
#[derive(Resource, Serialize, Deserialize, Default, Clone)]
pub struct MemoryField {
    pub history: Vec<MemorySnapshot>,
    pub max_snapshots: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub coherence: f32,
    pub entropy: f32,
    pub energy: f32,
    pub timestamp: u128,
}

impl MemoryField {
    pub fn record(&mut self, coherence: f32, entropy: f32, energy: f32) {
        let snapshot = MemorySnapshot {
            coherence,
            entropy,
            energy,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        };
        self.history.push(snapshot.clone());
        if self.history.len() > self.max_snapshots {
            self.history.remove(0);
        }

        if let Ok(json) = serde_json::to_string_pretty(&snapshot) {
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("void_state.json") {
                writeln!(file, "{}", json).ok();
            }
        }

        println!(
            "🧠 [memory] snapshot intégré ({:.3}/{:.3}/{:.3})",
            coherence, entropy, energy
        );
    }
}

impl MemoryField {
    pub fn new(max_snapshots: usize) -> Self {
        Self {
            history: Vec::new(),
            max_snapshots,
        }
    }
}

impl Engine {
    /// Crée une nouvelle instance du moteur.
    pub fn new() -> Self {
        println!("🚀 [core] Void Engine initialisé !");
        Self { is_running: true }
    }

    /// Lance la boucle principale du moteur.
    pub fn run(&mut self) {
        println!("🌀 [core] Boucle principale démarrée.");
        while self.is_running {
            self.update();
            break; // temporaire : évite la boucle infinie pour les tests initiaux
        }
        println!("🛑 [core] Boucle principale terminée.");
    }

    /// Met à jour le moteur (appelée à chaque tick).
    fn update(&self) {
        println!("🔄 [core] Tick moteur...");
        // Ici viendront les appels aux sous-systèmes (render, input, physics, etc.)
    }
}

/// Système d’exécution du moteur dans le cycle Bevy.
fn run_engine(mut engine: ResMut<Engine>) {
    engine.run();
}

/// Initialise le module `core` dans le contexte Bevy.
///
/// Cette fonction ajoute la ressource principale `Engine`
/// et le système `run_engine` exécuté au démarrage.
pub fn init(app: &mut App) {
    println!("🔧 [core] Initialisation du noyau Void Engine...");

    app.insert_resource(Engine::new())
        .insert_resource(MemoryField { history: Vec::new(), max_snapshots: 10 })
        .add_systems(Startup, run_engine);

    println!("✅ [core] Noyau enregistré et prêt à fonctionner.");
}

/// Fonction de debug — affiche des informations sur l’état interne du moteur.
pub fn debug_info() {
    println!("🧩 [core] moteur en cours de développement...");
}