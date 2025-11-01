//! Module `structure` — composant fondamental du moteur Void Engine
//!
//! Ce module gère la **structure ECS** (Entity Component System) du moteur,
//! responsable de l'organisation, la simulation et la cohérence des entités dans le monde virtuel.
//!
//! Il relie les couches inférieures (`substrate`, `core`) aux couches supérieures (`function`, `reflection`).

use bevy::prelude::*;

/// Composant de base : identifie une entité dans le moteur.
#[derive(Component)]
pub struct EntityTag {
    pub name: String,
}

/// Composant de transformation (position, rotation, échelle).
#[derive(Component)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }
}

#[allow(dead_code)]
/// Système d’exemple — met à jour les positions des entités en fonction du temps Bevy.
fn update_positions(mut query: Query<(&mut Transform, &EntityTag)>, time: Res<Time>) {
    for (mut transform, tag) in query.iter_mut() {
        transform.position += Vec3::new(0.0, 1.0, 0.0) * time.delta_secs();
        println!(
            "🧱 [structure] Entité '{}' déplacée en {:?}",
            tag.name, transform.position
        );
    }
}

/// Initialise le module `structure` (et la boucle ECS de base).
pub fn init(app: &mut App) {
    println!("🔧 [structure] Initialisation du monde ECS...");

    // ⚙️ Placeholder : systèmes ECS et entités seront enregistrés depuis le noyau
    println!("🧱 [structure] Chargement des systèmes ECS par le noyau...");
    println!("🧱 [structure] Enregistrement des entités de base...");

    app.add_systems(Update, update_positions);

    println!("✅ [structure] Monde ECS configuré (structure statique prête).");
}

/// Fonction de debug — affiche l’état ou la progression du module.
pub fn debug_info() {
    println!("🧩 [structure] ECS actif et connecté au moteur.");
}
