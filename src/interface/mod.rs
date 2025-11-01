// Module `interface` — composant fondamental du moteur Void Engine
//
// Responsabilités principales :
// - TODO: préciser les sous-systèmes et le rôle du module dans la stack globale
//
// Architecture :
// Chaque module du Void Engine est autonome, mais interconnecté via le système ECS (Bevy).
// L'objectif est de permettre une orchestration fluide entre simulation, rendu et réflexion.

// Module `interface` — Couche L5 : Projection externe et interconnexion du Void
//
// Ce module représente la **membrane du système** :
// - Il relie le moteur interne (`core`, `dynamics`, `function`, `reflection`) au monde extérieur.
// - Il gère les **flux entrants/sortants**, la **visualisation** et les **interfaces inter-Void**.
//
// En termes systémiques, `interface` agit comme un **pont d’observation** :
// il traduit les dynamiques internes en signaux observables et capte les stimuli externes pour les
// réinjecter dans les couches inférieures du moteur.

use crate::core::MemoryField;
use bevy::prelude::*;
use bevy::reflect::Reflect;
use tracing::{debug, info};

#[derive(Component)]
struct InterfaceDiagnostic;

/// Composant représentant une entrée externe (capteur, signal, événement utilisateur...).
#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub struct InputSignal {
    /// Intensité du signal reçu.
    pub intensity: f32,
    /// Canal d'émission du signal.
    pub channel: String,
}

/// Composant représentant une sortie observable (affichage, visualisation, export...).
#[derive(Reflect, Component, Default, Debug)]
#[reflect(Component)]
pub struct OutputProjection {
    /// Amplitude de la projection émise.
    pub amplitude: f32,
    /// Cible de la projection.
    pub target: String,
}

/// Ressource gérant les liens entre Voids (communication inter-systèmes).
#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct InterfaceLink {
    /// Liste des identifiants des Voids connectés.
    pub connected_voids: Vec<String>,
    /// Taux de transmission des données entre Voids.
    pub transmission_rate: f32,
}

/// Système : réception des signaux entrants.
///
/// Traite et atténue les intensités des signaux externes,
/// met à jour le taux de transmission en fonction de la force du signal.
fn receive_inputs(mut query: Query<&mut InputSignal>, mut link: ResMut<InterfaceLink>) {
    for mut input in query.iter_mut() {
        // Applique une dissipation naturelle sur l'intensité du signal reçu.
        input.intensity *= 0.95;

        // Calcule le taux de transmission normalisé à partir de l'intensité.
        link.transmission_rate = (input.intensity / 10.0).clamp(0.0, 1.0);

        // Log de réception des signaux entrants
        debug!(
            target: "interface",
            channel = %input.channel,
            intensity = input.intensity,
            transmission = link.transmission_rate,
            "réception signal"
        );
    }
}

/// Système : émission des projections vers l’extérieur.
///
/// Modifie les amplitudes des projections selon le taux de transmission,
/// reflétant la qualité du lien inter-Void.
fn emit_outputs(mut query: Query<&mut OutputProjection>, link: Res<InterfaceLink>) {
    for mut output in query.iter_mut() {
        // Ajuste l'amplitude de sortie en fonction du taux de transmission actuel.
        output.amplitude *= link.transmission_rate;

        // Log d'émission des projections externes
        debug!(
            target: "interface",
            target = %output.target,
            amplitude = output.amplitude,
            "émission signal"
        );
    }
}

/// Système : synchronise les connexions entre différents Voids.
///
/// Établit des connexions initiales si aucune n'existe,
/// ou affiche l'état actuel des liens actifs.
fn sync_links(mut link: ResMut<InterfaceLink>) {
    if link.connected_voids.is_empty() {
        link.connected_voids.push("PrimaryVoid".into());

        // Log de création de connexion initiale
        info!(target: "interface", "connexion établie avec PrimaryVoid");
    } else {
        // Log d'état des connexions existantes
        debug!(
            target: "interface",
            links = ?link.connected_voids,
            rate = link.transmission_rate,
            "liens actifs"
        );
    }
}

fn setup_visualization(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.8),
                custom_size: Some(Vec2::splat(220.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        InterfaceDiagnostic,
    ));
}

fn update_visualization(
    memory: Res<MemoryField>,
    mut query: Query<&mut Sprite, With<InterfaceDiagnostic>>,
) {
    if let Ok(mut sprite) = query.get_single_mut() {
        let coherence = memory.average("coherence", 60).unwrap_or(0.5);
        let entropy = memory.average("entropy", 60).unwrap_or(0.5);
        let intensity = (1.0 - entropy).clamp(0.0, 1.0);

        sprite.color = Color::rgb(
            coherence.clamp(0.0, 1.0),
            intensity,
            (1.0 - coherence).clamp(0.0, 1.0),
        );
    }
}

/// Initialise le module `interface`.
///
/// Configure les ressources, enregistre les types et ajoute les systèmes nécessaires.
pub fn init(app: &mut App) {
    // Log de démarrage de l'initialisation
    info!(target: "interface", "initialisation de la couche de projection");

    app.insert_resource(InterfaceLink::default())
        .register_type::<InputSignal>()
        .register_type::<OutputProjection>()
        .register_type::<InterfaceLink>()
        .add_systems(Startup, setup_visualization)
        .add_systems(
            Update,
            (
                receive_inputs,
                emit_outputs,
                sync_links,
                update_visualization,
            ),
        );

    // Log de confirmation de mise en ligne
    info!(target: "interface", "système d’interconnexion en ligne");

    // Log final de synthèse de l'initialisation
    debug!(
        target: "interface",
        "module prêt — communication et visualisation synchronisées"
    );

    // Log final de la phase d'initialisation
    debug!(
        target: "interface",
        "module finalisé — interconnexion fluide établie"
    );
}

/// Fonction de debug — affiche l’état ou la progression du module.
pub fn debug_info() {
    // Log d'information sur l'état du module
    debug!(target: "interface", "communication et projection actives");
}
