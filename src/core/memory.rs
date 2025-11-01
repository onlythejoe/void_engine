use bevy::prelude::*;

/// Ressource centrale de mémoire pour le Void Engine.
/// Enregistre les états successifs de cohérence, d’entropie et d’énergie
/// à chaque cycle des modules supérieurs.
#[derive(Resource, Default, Debug)]
pub struct MemoryField {
    pub history: Vec<MemorySnapshot>,
    pub max_size: usize,
}

/// Structure représentant une "trame" mémorielle du moteur.
#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub coherence: f32,
    pub entropy: f32,
    pub energy: f32,
}

impl MemoryField {
    /// Crée une nouvelle mémoire avec une taille maximale donnée.
    pub fn new(max_size: usize) -> Self {
        Self {
            history: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Ajoute un enregistrement dans la mémoire, en respectant la taille max.
    pub fn record(&mut self, coherence: f32, entropy: f32, energy: f32) {
        if self.history.len() >= self.max_size {
            self.history.remove(0);
        }
        self.history.push(MemorySnapshot {
            coherence,
            entropy,
            energy,
        });
    }

    /// Retourne la dernière trame de mémoire enregistrée.
    pub fn last(&self) -> Option<&MemorySnapshot> {
        self.history.last()
    }

    /// Vide complètement la mémoire.
    pub fn clear(&mut self) {
        self.history.clear();
    }
}