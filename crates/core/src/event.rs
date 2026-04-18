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
    /// The pet has died (boss fights only in v2).
    Died,
    /// The pet's stats are critical — needs care before exploring.
    NeedsCare,
    /// The pet has recovered from needs-care state.
    Recovered,
    /// The player entered Explore mode.
    EnteredExplore,
    /// The player returned to Camp mode.
    EnteredCamp,
    /// The pet was forced back to Camp (needs care while exploring).
    ForcedCamp,
    /// Won an auto-battle.
    BattleWon { xp_earned: u32, enemy_name: String },
    /// Lost an auto-battle (non-lethal in explore).
    BattleLost { enemy_name: String },
    /// Leveled up!
    LeveledUp { new_level: u32 },
    /// An item was dropped by a defeated enemy.
    ItemDropped { item_name: String, rarity: String },
    /// An item was equipped.
    ItemEquipped { item_name: String, slot: String },
    /// Inventory is full — loot was discarded.
    InventoryFull,
    /// A boss is available to fight (after enough auto-battles).
    BossAvailable,
    /// Defeated a boss.
    BossDefeated { xp_earned: u32, boss_name: String },
    /// Lost to a boss.
    BossDefeat { boss_name: String },
    /// Fled from a boss fight.
    FledFromBoss,
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
