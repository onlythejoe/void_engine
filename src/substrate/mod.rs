//! Module `substrate` — couche technique du Void Engine
//!
//! Gère les interfaces bas-niveau : rendu graphique, GPU, device context, et configuration système.
//! Ce module constitue la base matérielle sur laquelle reposent la simulation, la physique, et les couches supérieures.

use std::sync::Arc;

use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use tokio::sync::oneshot;
use tracing::{debug, error, info, warn};
use wgpu::{
    Backends, Device, HeadlessSurfaceDescriptor, Instance, InstanceDescriptor, PowerPreference,
    PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceTarget,
    TextureFormat, TextureUsages,
};

/// Structure représentant le contexte GPU global du Void Engine.
#[derive(Resource, Clone)]
pub struct GpuContext {
    pub adapter_name: String,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub surface_format: Option<TextureFormat>,
    pub surface: Option<Arc<Surface<'static>>>,
}

#[derive(Debug)]
pub enum GpuInitError {
    NoAdapter,
    RequestDevice(String),
}

impl std::fmt::Display for GpuInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoAdapter => write!(f, "no compatible GPU adapter available"),
            Self::RequestDevice(err) => write!(f, "failed to request device: {err}"),
        }
    }
}

impl std::error::Error for GpuInitError {}

impl From<wgpu::RequestDeviceError> for GpuInitError {
    fn from(value: wgpu::RequestDeviceError) -> Self {
        Self::RequestDevice(value.to_string())
    }
}

impl GpuContext {
    /// Initialise le contexte GPU (backend auto-détecté).
    pub async fn initialize() -> Result<Self, GpuInitError> {
        info!(target: "substrate", "starting GPU initialization");

        let instance = Instance::new(&InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .ok_or(GpuInitError::NoAdapter)?;

        let adapter_info = adapter.get_info();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await?;

        let surface = instance
            .create_surface(SurfaceTarget::Headless(HeadlessSurfaceDescriptor {
                width: 4,
                height: 4,
            }))
            .ok();

        let surface_format = surface
            .as_ref()
            .and_then(|surface| surface.get_capabilities(&adapter).formats.first().copied());

        if let (Some(surface), Some(format)) = (surface.as_ref(), surface_format) {
            configure_surface(surface, &device, format);
            debug!(target: "substrate", "headless surface configured");
        } else {
            warn!(target: "substrate", "headless surface unavailable; continuing without surface");
        }

        info!(target: "substrate", adapter = %adapter_info.name, "GPU initialization complete");

        Ok(Self {
            adapter_name: adapter_info.name,
            device: Arc::new(device),
            queue: Arc::new(queue),
            surface_format,
            surface: surface.map(Arc::new),
        })
    }
}

fn configure_surface(surface: &Surface<'static>, device: &Device, format: TextureFormat) {
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format,
        width: 4,
        height: 4,
        present_mode: PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Opaque,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(device, &config);
}

#[derive(Resource)]
struct PendingGpuInit(oneshot::Receiver<Result<GpuContext, GpuInitError>>);

fn start_gpu_initialization(mut commands: Commands) {
    let (sender, receiver) = oneshot::channel();

    IoTaskPool::get().spawn(async move {
        let result = GpuContext::initialize().await;
        let _ = sender.send(result);
    });

    commands.insert_resource(PendingGpuInit(receiver));
    info!(target: "substrate", "spawned asynchronous GPU task");
}

fn poll_gpu_initialization(mut commands: Commands, mut pending: Option<ResMut<PendingGpuInit>>) {
    let Some(mut pending) = pending else {
        return;
    };

    match pending.0.try_recv() {
        Ok(Some(Ok(context))) => {
            info!(target: "substrate", adapter = %context.adapter_name, "GPU context ready");
            commands.insert_resource(context);
            commands.remove_resource::<PendingGpuInit>();
        }
        Ok(Some(Err(err))) => {
            error!(target: "substrate", ?err, "failed to initialize GPU context");
            commands.remove_resource::<PendingGpuInit>();
        }
        Ok(None) => {}
        Err(err) => {
            error!(target: "substrate", ?err, "GPU initialization channel closed unexpectedly");
            commands.remove_resource::<PendingGpuInit>();
        }
    }
}

/// Initialise le module `substrate`.
pub fn init(app: &mut App) {
    info!(target: "substrate", "initializing GPU substrate module");

    app.add_systems(Startup, start_gpu_initialization)
        .add_systems(Update, poll_gpu_initialization);
}

/// Fonction de debug — affiche l’état du module `substrate`.
pub fn debug_info() {
    info!(target: "substrate", "GPU substrate active");
}
