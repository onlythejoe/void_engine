// Module `manifold` — Couche L6 : Champ unifié du Void Engine
//
// Le module `manifold` représente la **fusion totale** de toutes les couches précédentes :
// - `core` : fondements structurels et logiques du moteur
// - `dynamics` : comportements et mouvements internes
// - `function` : interactions et calculs abstraits
// - `reflection` : perception interne et rétroaction
// - `interface` : communication et projection externe
//
// Sa mission : maintenir la **cohérence spatio-temporelle et ontologique** du Void Engine,
// en orchestrant les échanges entre ces couches via un champ global : le **VoidField**.

use bevy::prelude::*;
use bevy::time::TimePlugin;
use crate::{reflection::*, interface::*};

/// Représente le champ unifié du Void — convergence de toutes les sous-couches.
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct VoidField {
    pub energy_flow: f32,
    pub coherence: f32,
    pub entropy: f32,
    pub active_layers: u8,
}

/// Système : agrège les valeurs issues des sous-couches pour maintenir l’équilibre du champ global.
fn unify_field(
    mut field: ResMut<VoidField>,
    reflection: Res<ReflectionField>,
    interface: Res<InterfaceLink>,
) {
    // Calcule et met à jour les propriétés du champ unifié en fonction des sous-couches.
    // Log the current state of the unified field for monitoring energy flow and coherence.
    field.energy_flow = (reflection.coherence + interface.transmission_rate) / 2.0;
    field.coherence = (field.energy_flow * 0.8 + (1.0 - reflection.depth) * 0.2).clamp(0.0, 1.0);
    field.entropy = 1.0 - field.coherence;
    field.active_layers = 6;

    // Monitoring unified field state
    println!(
        "🌌 [manifold] champ unifié → énergie {:.3} | cohérence {:.3} | entropie {:.3}",
        field.energy_flow, field.coherence, field.entropy
    );
}

/// Système : simule la respiration du Void — oscillation naturelle du champ d’énergie.
fn pulse(mut field: ResMut<VoidField>, time: Option<Res<Time>>) {
    // Simule une pulsation naturelle du champ d’énergie basée sur le temps écoulé.
    if let Some(time) = time {
        let wave = (time.elapsed_secs().sin() * 0.5 + 0.5) * field.coherence;
        field.energy_flow = (field.energy_flow * 0.9 + wave * 0.1).clamp(0.0, 1.0);
        // Log the pulse effect on energy flow for dynamic monitoring.
        println!("💫 [manifold] pulsation → flux {:.3}", field.energy_flow);
    } else {
        // Indicate that the Time resource is not yet available for pulse calculation.
        println!("⏳ [manifold] Time resource not yet available, skipping pulse.");
    }
}

/// Initialise la couche `manifold` et connecte toutes les sous-couches du moteur.
pub fn init(app: &mut App) {
    // Configure et lance le module manifold avec ses systèmes et ressources.
    // Signal the start of the manifold initialization process.
    println!("🔧 [manifold] Initialisation du champ global du Void...");

    app.add_plugins(TimePlugin);

    app.insert_resource(VoidField::default())
        .register_type::<VoidField>()
        .add_systems(Update, (unify_field, pulse));

    // Confirm that the unified field system is operational.
    println!("✅ [manifold] Champ unifié opérationnel, Void Engine cohérent.");
    // Summary log indicating the module is ready and stabilized.
    println!("🧠 [manifold] module prêt — cohérence universelle stabilisée.");
}

/// Fonction de debug — affiche l’état global du champ du Void.
pub fn debug_info() {
    // Affiche des informations de débogage sur la cohérence et le champ d’unification.
    // Inform that the debug state of the unified field is active.
    println!("🌀 [manifold] Cohérence et champ d’unification actifs...");
}