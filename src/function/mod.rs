//! Module `function` — composant fondamental du moteur Void Engine
//!
//! Couche **L3** : rétroaction, cohérence systémique et résonance interne.
//!
//! Ce module gère les **boucles fonctionnelles** du Void Engine :
//! - cycles d’interaction,
//! - résonances internes,
//! - propagation de cohérence entre sous-systèmes (structure, dynamics, reflection).
//!
//! Intégration ECS :
//! - `FeedbackLoop` : ressource reflectable qui enregistre les fluctuations globales.
//! - `Oscillator` : composant reflectable, représentant les entités vibratoires du système.
//! - Systèmes : `update_oscillators`, `regulate_entropy` (stade Update).

use bevy::prelude::*;
use std::f32::consts::PI;

/// Resource representing the global feedback loop state.
/// Tracks entropy, resonance phase, coherence level, and a frame counter for logging.
#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct FeedbackLoop {
    /// Current global entropy level of the system.
    pub global_entropy: f32,
    /// Current phase of the resonance cycle (radians).
    pub resonance_phase: f32,
    /// Coherence level derived from entropy, clamped between 0 and 1.
    pub coherence_level: f32,
    /// Frame counter used for periodic logging.
    pub frame_counter: u32,
}

/// Component representing a systemic vibratory oscillator.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Oscillator {
    /// Frequency of oscillation in Hz.
    pub frequency: f32,
    /// Amplitude of oscillation.
    pub amplitude: f32,
    /// Current phase of the oscillator (radians).
    pub phase: f32,
}

impl Default for Oscillator {
    fn default() -> Self {
        Self {
            frequency: 1.0,
            amplitude: 1.0,
            phase: 0.0,
        }
    }
}

/// System: updates oscillator phases and energy contributions to global entropy.
/// Prints a summary message every 60 frames to avoid console spam.
fn update_oscillators(
    mut query: Query<&mut Oscillator>,
    time: Res<Time>,
    mut feedback: ResMut<FeedbackLoop>,
) {
    // Iterate over all oscillators and update their phase based on frequency and delta time.
    for mut osc in query.iter_mut() {
        // Advance phase: phase += frequency * delta_time * 2π to complete cycles in radians.
        osc.phase += osc.frequency * time.delta_secs() * 2.0 * PI;
        // Calculate instantaneous energy as absolute value of amplitude * sin(phase).
        let energy = (osc.amplitude * osc.phase.sin()).abs();

        // Increment global entropy with a small contribution from this oscillator's energy.
        feedback.global_entropy += energy * 0.0001;
    }
    // Compute coherence level inversely proportional to entropy, clamped between 0 and 1.
    feedback.coherence_level = (1.0 / (1.0 + feedback.global_entropy)).clamp(0.0, 1.0);
    // Increment frame counter with wrapping to avoid overflow.
    feedback.frame_counter = feedback.frame_counter.wrapping_add(1);

    // Log status every 60 frames to reduce console spam.
    if feedback.frame_counter % 60 == 0 {
        println!(
            "🔁 [function] update_oscillators | entropy={:.5} coherence={:.4}",
            feedback.global_entropy, feedback.coherence_level
        );
    }
}

/// System: decays entropy and advances the global resonance phase.
fn regulate_entropy(mut feedback: ResMut<FeedbackLoop>) {
    const DECAY: f32 = 0.95;
    // Apply exponential decay to global entropy to simulate dissipation.
    feedback.global_entropy *= DECAY;
    // Advance resonance phase by a small fixed increment, wrapping around 2π.
    feedback.resonance_phase = (feedback.resonance_phase + 0.01) % (2.0 * PI);
    println!(
        "🌐 [function] regulate_entropy | entropy={:.6} phase={:.2}",
        feedback.global_entropy, feedback.resonance_phase
    );
}

/// System: resets the feedback loop state to default values.
pub fn reset_feedback(mut feedback: ResMut<FeedbackLoop>) {
    // Reset all feedback loop parameters to their initial default states.
    feedback.global_entropy = 0.0;
    feedback.resonance_phase = 0.0;
    feedback.coherence_level = 1.0;
    feedback.frame_counter = 0;
    println!("🔄 [function] reset_feedback | feedback loop state reset to defaults.");
}

/// Initializes the `function` module in Bevy.
pub fn init(app: &mut App) {
    println!("🔧 [function] initializing functional feedback loops...");

    app.insert_resource(FeedbackLoop::default())
        .register_type::<FeedbackLoop>()
        .register_type::<Oscillator>()
        .add_systems(Update, (update_oscillators, regulate_entropy))
        .add_systems(Startup, reset_feedback);

    println!("✅ [function] functional feedback loops online.");
    println!("🧠 [function] module prêt — boucles fonctionnelles stabilisées.");
}

/// Debug function — prints a synthetic status message.
pub fn debug_info() {
    println!("🧩 [function] feedback loops synchronized and operational.");
}
