use serde::{Deserialize, Serialize};

/// Core stats that define a pet's current condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetStats {
    pub hunger: f32,
    pub happiness: f32,
    pub health: f32,
    pub energy: f32,
}

impl Default for PetStats {
    fn default() -> Self {
        Self {
            hunger: 50.0,
            happiness: 50.0,
            health: 100.0,
            energy: 100.0,
        }
    }
}

impl PetStats {
    /// Clamp all stats to [0.0, 100.0].
    pub fn clamp(&mut self) {
        self.hunger = self.hunger.clamp(0.0, 100.0);
        self.happiness = self.happiness.clamp(0.0, 100.0);
        self.health = self.health.clamp(0.0, 100.0);
        self.energy = self.energy.clamp(0.0, 100.0);
    }
}

/// Life stages a pet progresses through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PetStage {
    Egg,
    Baby,
    Teen,
    Adult,
    Elder,
}

impl PetStage {
    /// Age (in seconds) at which the pet evolves to the next stage.
    pub fn evolution_threshold(self) -> Option<u64> {
        match self {
            Self::Egg => Some(30),
            Self::Baby => Some(120),
            Self::Teen => Some(300),
            Self::Adult => Some(600),
            Self::Elder => None,
        }
    }

    pub fn next(self) -> Option<Self> {
        match self {
            Self::Egg => Some(Self::Baby),
            Self::Baby => Some(Self::Teen),
            Self::Teen => Some(Self::Adult),
            Self::Adult => Some(Self::Elder),
            Self::Elder => None,
        }
    }
}

/// Thresholds for the needs-care gate.
pub const NEEDS_CARE_HUNGER: f32 = 90.0;
pub const NEEDS_CARE_HAPPINESS: f32 = 10.0;
pub const NEEDS_CARE_ENERGY: f32 = 10.0;
/// Recovery thresholds — stats must be better than these to clear needs_care.
pub const RECOVER_HUNGER: f32 = 70.0;
pub const RECOVER_HAPPINESS: f32 = 30.0;
pub const RECOVER_ENERGY: f32 = 20.0;

/// A virtual pet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pet {
    pub name: String,
    pub stats: PetStats,
    pub stage: PetStage,
    pub age_seconds: u64,
    pub alive: bool,
    /// When true, the pet stops gaining resources and must be cared for.
    #[serde(default)]
    pub needs_care: bool,
}

impl Pet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            stats: PetStats::default(),
            stage: PetStage::Egg,
            age_seconds: 0,
            alive: true,
            needs_care: false,
        }
    }

    /// Check if stats have crossed critical thresholds.
    pub fn check_needs_care(&self) -> bool {
        self.stats.hunger >= NEEDS_CARE_HUNGER
            || self.stats.happiness <= NEEDS_CARE_HAPPINESS
            || self.stats.energy <= NEEDS_CARE_ENERGY
    }

    /// Check if stats have recovered enough to clear needs_care.
    pub fn check_recovered(&self) -> bool {
        self.stats.hunger < RECOVER_HUNGER
            && self.stats.happiness > RECOVER_HAPPINESS
            && self.stats.energy > RECOVER_ENERGY
    }
}
