use bevy::prelude::*;
use tracing::{debug, info};

pub mod memory;

pub use memory::MemoryField;

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

impl Engine {
    /// Crée une nouvelle instance du moteur.
    pub fn new() -> Self {
        info!(target: "core", "Void Engine initialisé");
        Self { is_running: true }
    }

    /// Lance la boucle principale du moteur.
    pub fn run(&mut self) {
        info!(target: "core", "Boucle principale démarrée");
        while self.is_running {
            self.update();
            break; // temporaire : évite la boucle infinie pour les tests initiaux
        }
        info!(target: "core", "Boucle principale terminée");
    }

    /// Met à jour le moteur (appelée à chaque tick).
    fn update(&self) {
        debug!(target: "core", "Tick moteur");
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
    info!(target: "core", "Initialisation du noyau Void Engine");

    app.insert_resource(Engine::new())
        .insert_resource(MemoryField::new(10))
        .add_systems(Startup, run_engine);

    info!(target: "core", "Noyau enregistré et prêt à fonctionner");
}

/// Fonction de debug — affiche des informations sur l’état interne du moteur.
pub fn debug_info() {
    debug!(target: "core", "moteur en cours de développement");
}
