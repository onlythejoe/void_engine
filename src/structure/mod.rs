//! Module `structure` — composant fondamental du moteur Void Engine
//!
//! Ce module gère la **structure ECS** (Entity Component System) du moteur,
//! responsable de l'organisation, la simulation et la cohérence des entités dans le monde virtuel.
//!
//! Il relie les couches inférieures (`substrate`, `core`) aux couches supérieures (`function`, `reflection`).

use bevy::prelude::*;
use tracing::{debug, info};

/// Composant de base : identifie une entité dans le moteur.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EntityTag {
    pub name: String,
}

#[allow(dead_code)]
/// Système d’exemple — met à jour les positions des entités en fonction du temps Bevy.
fn update_positions(mut query: Query<(&mut Transform, &EntityTag)>, time: Res<Time>) {
    for (mut transform, tag) in query.iter_mut() {
        transform.translation += Vec3::new(0.0, 1.0, 0.0) * time.delta_secs();
        debug!(
            target: "structure",
            entity = %tag.name,
            position = ?transform.translation,
            "entité déplacée"
        );
    }
}

/// Initialise le module `structure` (et la boucle ECS de base).
pub fn init(app: &mut App) {
    info!(target: "structure", "Initialisation du monde ECS");

    // ⚙️ Placeholder : systèmes ECS et entités seront enregistrés depuis le noyau
    debug!(target: "structure", "Chargement des systèmes ECS par le noyau");
    debug!(target: "structure", "Enregistrement des entités de base");

    app.register_type::<EntityTag>()
        .add_systems(Update, update_positions);

    info!(
        target: "structure",
        "Monde ECS configuré (structure statique prête)"
    );
}

/// Fonction de debug — affiche l’état ou la progression du module.
pub fn debug_info() {
    debug!(target: "structure", "ECS actif et connecté au moteur");
}
