//! Module `core` — composant fondamental du moteur Void Engine
//!
//! Responsabilités principales :
//! - TODO: préciser les sous-systèmes et le rôle du module dans la stack globale
//!
//! Architecture :
//! Chaque module du Void Engine est autonome, mais interconnecté via le système ECS (Bevy).
//! L'objectif est de permettre une orchestration fluide entre simulation, rendu et réflexion.

/// Initialise le module `core`.
pub fn init() {
    println!("🔧 [core] module initialisé.");
}

/// Fonction de debug — affiche l’état ou la progression du module.
pub fn debug_info() {
    println!("🧩 [core] en cours de développement...");
}
