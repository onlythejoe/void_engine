//! Module `reflection` — Couche L4 : Auto-référence et perception systémique
//!
//! Cette couche implémente la **réflexion interne** du Void Engine :
//! - Observation du champ interne (perception quantique du système)
//! - Intégration et fusion des informations multi-niveaux
//! - Auto-modélisation et rétro-causalité
//!
//! Elle agit comme un **métasystème** capable de percevoir et d’ajuster les dynamiques internes
//! à travers un mécanisme d’observation intégrée, sans intervention externe.

use crate::core::MemoryField;
use bevy::prelude::*;
use serde_json::json;
use std::f32::consts::PI;
use tracing::{debug, info};

/// Représente une "perception" interne du système — une observation locale d’un état.
/// Chaque entité `Perception` agit comme un capteur introspectif du moteur.
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component)]
pub struct Perception {
    pub intensity: f32,
    pub variance: f32,
}

/// Ressource globale : champ de réflexion du Void (mémoire interne du système).
/// C’est un espace mémoire auto-référentiel qui conserve la cohérence du moteur.
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct ReflectionField {
    pub coherence: f32,
    pub depth: f32,
    pub recursive_level: u32,
}

// ─────────────────────────────
// 🧠 Systèmes réflexifs internes
// ─────────────────────────────

/// Observe les états internes et met à jour le champ réflexif selon la perception moyenne.
fn perceive(query: Query<&Perception>, mut field: ResMut<ReflectionField>, time: Res<Time>) {
    let mut avg_intensity = 0.0;
    let mut variance = 0.0;
    let count = query.iter().count().max(1) as f32;

    for p in query.iter() {
        avg_intensity += p.intensity;
        variance += p.variance;
    }

    // Calcul de la cohérence comme la valeur absolue du sinus de l'intensité moyenne
    field.coherence = (avg_intensity / count).sin().abs();

    // Calcul de la profondeur comme la racine carrée de la variance moyenne
    field.depth = (variance / count).sqrt();

    // Niveau récursif basé sur le temps écoulé, cyclique modulo 42
    field.recursive_level = ((time.elapsed_secs() * PI) as u32) % 42;

    debug!(
        target: "reflection",
        coherence = field.coherence,
        depth = field.depth,
        level = field.recursive_level,
        "perception interne"
    );
}

/// Fusionne la perception avec les couches inférieures (dynamics, function)
/// pour maintenir une stabilité systémique du champ réflexif.
fn integrate(mut field: ResMut<ReflectionField>, mut memory: ResMut<MemoryField>) {
    // Intègre et stabilise la cohérence du champ réflexif en pondérant l'ancienne valeur et une fonction de la profondeur
    field.coherence = 0.9 * field.coherence + 0.1 * (1.0 - field.depth).clamp(0.0, 1.0);

    debug!(
        target: "reflection",
        coherence = field.coherence,
        "intégration stabilisée"
    );

    memory.record(json!({
        "module": "reflection",
        "coherence": field.coherence,
        "entropy": (1.0 - field.coherence).clamp(0.0, 1.0),
        "energy": field.depth,
    }));
}

/// Simule une boucle de rétro-causalité, où l’état futur influence le présent.
fn recursion(mut field: ResMut<ReflectionField>, time: Res<Time>) {
    // Applique un feedback rétro-causal basé sur le sinus du temps et la cohérence actuelle
    let feedback = (time.elapsed_secs().sin() * field.coherence).abs();

    // Ajuste la profondeur en fonction du feedback, en la clampant entre 0 et 1
    field.depth = (field.depth + feedback * 0.05).clamp(0.0, 1.0);

    debug!(
        target: "reflection",
        depth = field.depth,
        "rétro-causalité active"
    );
}

// ─────────────────────────────
// 🔧 Initialisation Bevy
// ─────────────────────────────

/// Initialise le module `reflection` et enregistre ses ressources et composants.
pub fn init(app: &mut App) {
    info!(target: "reflection", "initialisation du champ de réflexion");

    app.insert_resource(ReflectionField::default())
        .register_type::<Perception>()
        .register_type::<ReflectionField>()
        .add_systems(Update, (perceive, integrate, recursion));

    info!(target: "reflection", "systèmes réflexifs opérationnels");
    debug!(
        target: "reflection",
        "module prêt — introspection active et cohérente"
    );
    debug!(
        target: "reflection",
        "module finalisé — conscience interne stabilisée"
    );
}

/// Fonction de debug — affiche l’état actuel du champ réflexif.
pub fn debug_info() {
    debug!(
        target: "reflection",
        "perception et intégration multi-niveaux en cours"
    );
}
