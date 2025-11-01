//! Module `substrate` — couche technique du Void Engine
//!
//! Gère les interfaces bas-niveau : rendu graphique, GPU, device context, et configuration système.
//! Ce module constitue la base matérielle sur laquelle reposent la simulation, la physique, et les couches supérieures.

use std::sync::Arc;
use bevy::prelude::*;
use wgpu::{Instance, Adapter, Device, Queue, InstanceDescriptor, Backends, RequestAdapterOptions};

/// Structure représentant le contexte GPU global du Void Engine.
#[derive(Resource, Debug, Default)]
pub struct GpuContext {
    pub instance: Option<Instance>,
    pub adapter: Option<Adapter>,
    pub device: Option<Arc<Device>>,
    pub queue: Option<Arc<Queue>>,
}

impl GpuContext {
    /// Initialise le contexte GPU (backend auto-détecté).
    ///
    /// Cette fonction crée une instance GPU, sélectionne un adaptateur compatible,
    /// puis initialise le device et la queue associés.
    pub async fn initialize() -> Self {
        // Log d'initialisation du contexte GPU
        println!("🎮 [substrate] Initialisation du GPU context...");

        // Configuration de l’instance GPU (multi-backend compatible)
        let instance_desc = InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        };
        let instance = Instance::new(&instance_desc);

        // Sélection de l’adaptateur GPU disponible
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .expect("❌ [substrate] Aucun adaptateur GPU trouvé !");

        // Création du périphérique et de la file de commandes
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("❌ [substrate] Impossible de créer le device GPU.");

        // Log de succès avec le nom de l'adaptateur GPU sélectionné
        println!(
            "✅ [substrate] GPU initialisé avec succès : {}",
            adapter.get_info().name
        );

        Self {
            instance: Some(instance),
            adapter: Some(adapter),
            device: Some(Arc::new(device)),
            queue: Some(Arc::new(queue)),
        }
    }
}

/// Initialise le module `substrate`.
///
/// Cette fonction insère un contexte GPU par défaut dans Bevy,
/// puis lance une initialisation asynchrone du GPU.
pub fn init(app: &mut App) {
    // Log de démarrage de l'initialisation du module GPU
    println!("🔧 [substrate] Initialisation du module GPU...");

    // Insertion d’un contexte vide dans Bevy
    app.insert_resource(GpuContext::default());

    // Initialisation asynchrone du GPU
    bevy::tasks::IoTaskPool::get()
        .spawn(async {
            let context = GpuContext::initialize().await;

            // Log indiquant la fin de l'initialisation asynchrone avec état de l'adaptateur
            println!(
                "⚙️ [substrate] GPU async setup terminé : adaptateur disponible = {}",
                context.adapter.is_some()
            );
        })
        .detach();

    // Log indiquant que le contexte GPU est en cours d'initialisation asynchrone
    println!("✅ [substrate] Contexte GPU en cours d’initialisation (asynchrone).");

    // Log final de confirmation que le module est prêt
    println!("🧠 [substrate] module prêt — fondation matérielle stabilisée.");
}

/// Fonction de debug — affiche l’état du module `substrate`.
///
/// Cette fonction sert à vérifier l'état actuel du contexte GPU,
/// qui peut être en attente ou déjà initialisé.
pub fn debug_info() {
    // Log d'information sur la configuration GPU actuelle
    println!("🧩 [substrate] Configuration GPU : en attente ou initialisée selon l’état asynchrone.");
}
