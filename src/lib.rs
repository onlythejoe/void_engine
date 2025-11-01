//! # Void Engine
//!
//! Moteur expérimental modulaire basé sur **Bevy + WGPU**
//!
//! ## Architecture des couches :
//! - **Core** — fondations primitives (types, temps, mémoire, logique fondamentale)
//! - **Substrate** — plan matériel et énergétique (base d’existence)
//! - **Dynamics** — forces, mouvements, cycles internes
//! - **Structure** — formes, hiérarchies, topologies
//! - **Function** — comportements, opérations, processus
//! - **Reflection** — perception interne, rétroaction, conscience du système
//! - **Interface** — projection externe, communication, visualisation
//! - **Manifold** — champ unifié du moteur (synchronisation totale)
//!
//! Le `VoidEngine` est conçu comme un **organisme évolutif**, où chaque module agit comme une
//! strate de complexité interconnectée.

use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, TaskPool};
use tracing::{debug, info};

pub mod core;
pub mod dynamics;
pub mod function;
pub mod interface;
pub mod manifold;
pub mod reflection;
pub mod structure;
pub mod substrate;

/// Structure centrale du moteur — point d’entrée de tout le système.
pub struct VoidEngine;

impl VoidEngine {
    /// Initialise le moteur complet.
    ///
    /// Hiérarchie d’initialisation :
    /// - 🧱 Phase 1 : couches fondamentales
    /// - ⚙️ Phase 2 : couches dynamiques et structurelles
    /// - 🌌 Phase 3 : couches réflexives et globales
    pub fn init() -> App {
        // ⚡ Initialisation explicite du pool de tâches Bevy
        IoTaskPool::get_or_init(|| TaskPool::new());
        info!(target: "void_engine", "Initialisation du moteur Void");
        let mut app = App::new();

        // 🧱 Phase 1 : couches fondamentales
        core::init(&mut app);
        substrate::init(&mut app);

        // ⚙️ Phase 2 : couches dynamiques et structurelles
        dynamics::init(&mut app);
        structure::init(&mut app);
        function::init(&mut app);

        // 🌌 Phase 3 : couches réflexives et globales
        reflection::init(&mut app);
        interface::init(&mut app);
        manifold::init(&mut app);

        app.insert_resource(core::MemoryField::new(512)); // mémoire circulaire de 512 trames

        info!(
            target: "void_engine",
            "Toutes les couches du moteur sont opérationnelles"
        );
        app
    }

    /// Fonction de debug globale
    pub fn debug() {
        debug!(
            target: "void_engine",
            "État général du moteur : synchronisé, stable, conscient"
        );
        reflection::debug_info();
        interface::debug_info();
        manifold::debug_info();
    }
}
