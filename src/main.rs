//! # Entrée principale du Void Engine
//!
//! Ce fichier lance le moteur en initialisant toutes les couches définies dans `lib.rs`.
//! Il crée une instance du moteur, exécute la boucle principale Bevy, et affiche les logs
//! d’initialisation et d’état général.

use tracing::info;
use tracing_subscriber::EnvFilter;
use void_engine::VoidEngine;

fn init_tracing() {
    let default_directives = if cfg!(feature = "verbose") {
        "void_engine=debug,substrate=debug,manifold=debug,wgpu=warn"
    } else {
        "void_engine=info,substrate=info,wgpu=warn"
    };

    let directives = std::env::var("RUST_LOG").unwrap_or_else(|_| default_directives.to_string());

    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(directives))
        .with_target(true)
        .with_level(true)
        .compact();

    let _ = subscriber.try_init();
}

fn main() {
    init_tracing();
    info!(target: "void_engine", "Lancement du Void Engine");

    // Initialisation complète du moteur
    let mut app = VoidEngine::init();

    // Log de debug initial
    VoidEngine::debug();

    // Exécution de la boucle principale Bevy
    app.run();
}
