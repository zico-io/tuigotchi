use crate::pet::PetStage;

/// Game events emitted by the core systems.
///
/// V2 will extend this with combat, loot, and skill events.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum GameEvent {
    /// A stat is critically low/high.
    StatWarning(&'static str),
    /// The pet evolved to a new stage.
    Evolved { from: PetStage, to: PetStage },
    /// The pet has died.
    Died,
}

/// Simple event collector. Gather events during a tick, drain after processing.
#[derive(Debug, Default)]
pub struct EventBus {
    events: Vec<GameEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
