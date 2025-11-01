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

use bevy::prelude::*;
use bevy::reflect::Reflect;

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
        println!(
            "📡 [interface] réception — canal '{}' intensité {:.2} → taux transmission {:.2}",
            input.channel, input.intensity, link.transmission_rate
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
        println!(
            "💡 [interface] émission — cible '{}' amplitude {:.2}",
            output.target, output.amplitude
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
        println!("🌐 [interface] connexion établie avec PrimaryVoid");
    } else {
        // Log d'état des connexions existantes
        println!(
            "🌐 [interface] liens actifs : {:?} | taux {:.2}",
            link.connected_voids, link.transmission_rate
        );
    }
}

/// Initialise le module `interface`.
///
/// Configure les ressources, enregistre les types et ajoute les systèmes nécessaires.
pub fn init(app: &mut App) {
    // Log de démarrage de l'initialisation
    println!("🔧 [interface] initialisation de la couche de projection...");

    app.insert_resource(InterfaceLink::default())
        .register_type::<InputSignal>()
        .register_type::<OutputProjection>()
        .register_type::<InterfaceLink>()
        .add_systems(Update, (receive_inputs, emit_outputs, sync_links));

    // Log de confirmation de mise en ligne
    println!("✅ [interface] système d’interconnexion en ligne.");

    // Log final de synthèse de l'initialisation
    println!("🧠 [interface] module prêt — communication et visualisation synchronisées.");

    // Log final de la phase d'initialisation
    println!("🪞 [interface] module finalisé — interconnexion fluide établie.");
}

/// Fonction de debug — affiche l’état ou la progression du module.
pub fn debug_info() {
    // Log d'information sur l'état du module
    println!("🧩 [interface] communication et projection actives...");
}