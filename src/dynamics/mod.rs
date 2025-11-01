//! Module `dynamics` — couche de simulation du Void Engine
//!
//! Ce module gère les **interactions physiques et énergétiques** entre entités :
//! - calcul des forces, vitesses et accélérations,
//! - propagation d’énergie (cinétique, potentielle, vibratoire),
//! - intégration dans la boucle ECS pour influencer la structure du monde.
//!
//! Il constitue la **couche L1 (dynamique quantique et systémique)** du moteur.

use bevy::prelude::*;
use std::f32::consts::PI;
use tracing::{debug, info};

/// Composant représentant la vélocité d'une entité (en unités/s).
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: Vec3,
}

/// Composant représentant la masse et l’inertie d’une entité.
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Mass {
    pub value: f32,
}

/// Composant représentant une force appliquée à une entité.
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Force {
    pub direction: Vec3,
    pub magnitude: f32,
}

impl Force {
    /// Calcule le vecteur de force résultant.
    pub fn vector(&self) -> Vec3 {
        self.direction.normalize_or_zero() * self.magnitude
    }
}

/// Système : applique les forces aux entités pour mettre à jour leurs vitesses.
pub fn apply_forces(mut query: Query<(&mut Velocity, &Force, &Mass)>, time: Res<Time>) {
    // Applique l'accélération issue des forces sur la vélocité linéaire.
    for (mut velocity, force, mass) in query.iter_mut() {
        // Calcul de l'accélération : force / masse (avec protection contre division par zéro)
        let acceleration = force.vector() / mass.value.max(1e-6);
        // Intégration de l'accélération dans la vitesse linéaire (changement de vitesse)
        velocity.linear += acceleration * time.delta_secs();
        debug!(
            target: "dynamics",
            ?acceleration,
            linear = ?velocity.linear,
            "accélération appliquée"
        );
    }
}

/// Système : met à jour les positions à partir des vitesses.
pub fn integrate_positions(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    // Intègre la vélocité linéaire dans la position et la vélocité angulaire dans la rotation.
    for (mut transform, velocity) in query.iter_mut() {
        // Mise à jour de la position par déplacement linéaire
        transform.translation += velocity.linear * time.delta_secs();
        // Mise à jour de la rotation autour de l'axe Y (en radians)
        transform.rotation = transform.rotation
            * Quat::from_rotation_y(velocity.angular.y * time.delta_secs() * PI / 180.0);
    }
}

/// Initialise le module `dynamics` — enregistre les composants et systèmes physiques.
pub fn init(app: &mut App) {
    info!(target: "dynamics", "initialisation des systèmes physiques");

    app.register_type::<Velocity>()
        .register_type::<Mass>()
        .register_type::<Force>()
        .add_systems(Update, (apply_forces, integrate_positions));

    info!(
        target: "dynamics",
        "systèmes physiques enregistrés et actifs"
    );
    debug!(
        target: "dynamics",
        "module prêt — dynamique systémique stabilisée"
    );
}

/// Fonction de debug — affiche un état symbolique du module.
pub fn debug_info() {
    debug!(target: "dynamics", "simulation physique en cours d’intégration");
}
