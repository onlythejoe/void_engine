//! # Entrée principale du Void Engine
//! 
//! Ce fichier lance le moteur en initialisant toutes les couches définies dans `lib.rs`.
//! Il crée une instance du moteur, exécute la boucle principale Bevy, et affiche les logs
//! d’initialisation et d’état général.

use void_engine::VoidEngine;

fn main() {
    println!("🌌 Lancement du Void Engine...");
    
    // Initialisation complète du moteur
    let mut app = VoidEngine::init();

    // Log de debug initial
    VoidEngine::debug();

    // Exécution de la boucle principale Bevy
    app.run();
}
