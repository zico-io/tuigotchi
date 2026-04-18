use std::path::PathBuf;

use tuigotchi_core::{
    action::{self, Action, ALL_ACTIONS},
    combat::{
        battle, boss,
        combat_profile::CombatProfile,
        explore_state::ExploreState,
        manual_combat::{BossEncounterState, TurnResult, ALL_COMBAT_ACTIONS},
    },
    event::{EventBus, GameEvent},
    game_state::GameMode,
    items::{inventory::Inventory, item::Rarity, loot},
    offline::{self, OfflineCombatContext},
    pet::Pet,
    save::{self, SaveData},
    tick::{self, CombatContext},
};

/// Which screen the TUI is currently displaying.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Main,
    Inventory,
    BossFight,
}

pub struct App {
    pub pet: Pet,
    pub events: EventBus,
    pub selected_action: usize,
    pub status_message: Option<String>,
    pub running: bool,
    pub save_path: PathBuf,
    pub game_mode: GameMode,
    pub combat_profile: CombatProfile,
    pub explore_state: ExploreState,
    pub inventory: Inventory,
    pub screen: Screen,
    pub inventory_cursor: usize,
    pub boss_encounter: Option<BossEncounterState>,
    pub boss_action_cursor: usize,
    pub explore_tick_count: u32,
}

impl App {
    pub fn new(pet_name: impl Into<String>, save_path: PathBuf) -> Self {
        Self {
            pet: Pet::new(pet_name),
            events: EventBus::new(),
            selected_action: 0,
            status_message: None,
            running: true,
            save_path,
            game_mode: GameMode::default(),
            combat_profile: CombatProfile::new(),
            explore_state: ExploreState::default(),
            inventory: Inventory::default(),
            screen: Screen::Main,
            inventory_cursor: 0,
            boss_encounter: None,
            boss_action_cursor: 0,
            explore_tick_count: 0,
        }
    }

    /// Restore from save data, simulating offline time. Returns the app with a welcome-back message.
    pub fn from_save(data: SaveData, save_path: PathBuf) -> Self {
        let mut pet = data.pet;
        let mut events = EventBus::new();
        let game_mode = data.game_mode;
        let mut combat_profile = data.combat_profile.unwrap_or_default();
        let mut explore_state = data.explore_state.unwrap_or_default();
        let mut inventory = data.inventory.unwrap_or_default();
        let boss_encounter = data.boss_encounter;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let elapsed = now.saturating_sub(data.last_saved_at);
        let summary = offline::simulate_offline(
            &mut pet,
            elapsed,
            &mut events,
            Some(&mut OfflineCombatContext {
                profile: &mut combat_profile,
                explore_state: &mut explore_state,
                inventory: &mut inventory,
                game_mode,
            }),
        );
        let status_message = if elapsed > 60 {
            Some(summary.message())
        } else {
            None
        };

        // Restore boss fight screen if we were in one
        let screen = if boss_encounter.is_some() {
            Screen::BossFight
        } else {
            Screen::Main
        };

        Self {
            pet,
            events,
            selected_action: 0,
            status_message,
            running: true,
            save_path,
            game_mode,
            combat_profile,
            explore_state,
            inventory,
            screen,
            inventory_cursor: 0,
            boss_encounter,
            boss_action_cursor: 0,
            explore_tick_count: 0,
        }
    }

    /// Save current state to disk.
    pub fn save(&self) -> Result<(), save::SaveError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let data = SaveData::new(
            self.pet.clone(),
            now,
            self.game_mode,
            Some(self.combat_profile.clone()),
            Some(self.explore_state.clone()),
            Some(self.inventory.clone()),
            self.boss_encounter.clone(),
        );
        save::save(&data, &self.save_path)
    }

    /// Whether the pet is healthy enough to explore.
    pub fn can_explore(&self) -> bool {
        !self.pet.needs_care
    }

    /// Toggle between Camp and Explore modes.
    pub fn toggle_mode(&mut self) {
        match self.game_mode {
            GameMode::Camp => {
                if self.can_explore() {
                    self.game_mode = GameMode::Explore;
                    self.status_message = Some("Heading out to explore!".into());
                } else {
                    self.status_message = Some("Your pet needs care before exploring!".into());
                }
            }
            GameMode::Explore => {
                self.game_mode = GameMode::Camp;
                self.status_message = Some("Returned to camp.".into());
            }
            _ => {}
        }
    }

    /// Toggle the inventory screen (only available in Camp mode).
    pub fn toggle_inventory(&mut self) {
        match self.screen {
            Screen::Main => {
                if self.game_mode == GameMode::Camp {
                    self.screen = Screen::Inventory;
                    self.inventory_cursor = 0;
                } else {
                    self.status_message = Some("Return to camp to manage inventory.".into());
                }
            }
            Screen::Inventory => {
                self.screen = Screen::Main;
            }
            Screen::BossFight => {
                self.status_message = Some("Can't manage inventory during a boss fight!".into());
            }
        }
    }

    /// Move inventory cursor down.
    pub fn inventory_next(&mut self) {
        let len = self.inventory.len();
        if len > 0 {
            self.inventory_cursor = (self.inventory_cursor + 1) % len;
        }
    }

    /// Move inventory cursor up.
    pub fn inventory_prev(&mut self) {
        let len = self.inventory.len();
        if len > 0 {
            self.inventory_cursor = if self.inventory_cursor == 0 {
                len - 1
            } else {
                self.inventory_cursor - 1
            };
        }
    }

    /// Equip the currently selected item.
    pub fn inventory_equip(&mut self) {
        if self.inventory_cursor < self.inventory.len() {
            let item_name = self.inventory.items()[self.inventory_cursor].name.clone();
            let slot = self.inventory.items()[self.inventory_cursor]
                .slot
                .label()
                .to_string();
            if self.inventory.equip(self.inventory_cursor).is_ok() {
                self.status_message = Some(format!("Equipped {item_name}!"));
                self.events
                    .push(GameEvent::ItemEquipped { item_name, slot });
            }
        }
    }

    /// Unequip the currently selected item's slot.
    pub fn inventory_unequip(&mut self) {
        if self.inventory_cursor < self.inventory.len() {
            let slot = self.inventory.items()[self.inventory_cursor].slot;
            self.inventory.unequip(slot);
            self.status_message = Some("Unequipped item.".into());
        }
    }

    /// Discard the currently selected item.
    pub fn inventory_discard(&mut self) {
        if self.inventory_cursor < self.inventory.len() {
            let item = self.inventory.remove_item(self.inventory_cursor);
            if let Some(item) = item {
                self.status_message = Some(format!("Discarded {}.", item.name));
            }
            // Adjust cursor if it's now out of bounds
            if !self.inventory.is_empty() && self.inventory_cursor >= self.inventory.len() {
                self.inventory_cursor = self.inventory.len() - 1;
            }
        }
    }

    pub fn current_action(&self) -> Action {
        ALL_ACTIONS[self.selected_action]
    }

    pub fn next_action(&mut self) {
        self.selected_action = (self.selected_action + 1) % ALL_ACTIONS.len();
    }

    pub fn prev_action(&mut self) {
        self.selected_action = if self.selected_action == 0 {
            ALL_ACTIONS.len() - 1
        } else {
            self.selected_action - 1
        };
    }

    pub fn perform_action(&mut self) {
        let action = self.current_action();
        action::apply_action(&mut self.pet, action);
        self.status_message = Some(format!("You {}!", action_past_tense(action)));
    }

    pub fn tick(&mut self, elapsed_secs: u64) {
        if self.game_mode == GameMode::Explore {
            self.explore_tick_count = self.explore_tick_count.wrapping_add(1);
        }

        let eq_mods = self.inventory.total_modifiers();
        let combat_ctx = if self.game_mode == GameMode::Explore {
            Some(CombatContext {
                profile: &mut self.combat_profile,
                explore_state: &mut self.explore_state,
                inventory: &mut self.inventory,
                equipment_modifiers: eq_mods,
            })
        } else {
            None
        };

        // We need to reborrow to satisfy the borrow checker with the mutable ref
        // to self.events while combat_ctx borrows other fields.
        if let Some(mut ctx) = combat_ctx {
            tick::tick(
                &mut self.pet,
                elapsed_secs,
                &mut self.events,
                self.game_mode,
                Some(&mut ctx),
            );
        } else {
            tick::tick(
                &mut self.pet,
                elapsed_secs,
                &mut self.events,
                self.game_mode,
                None,
            );
        }

        self.process_events();
    }

    /// Navigate to next boss action.
    pub fn boss_next_action(&mut self) {
        self.boss_action_cursor = (self.boss_action_cursor + 1) % ALL_COMBAT_ACTIONS.len();
    }

    /// Navigate to previous boss action.
    pub fn boss_prev_action(&mut self) {
        self.boss_action_cursor = if self.boss_action_cursor == 0 {
            ALL_COMBAT_ACTIONS.len() - 1
        } else {
            self.boss_action_cursor - 1
        };
    }

    /// Perform the currently selected boss action.
    pub fn boss_perform_action(&mut self) {
        let action = ALL_COMBAT_ACTIONS[self.boss_action_cursor];

        let result = if let Some(ref mut encounter) = self.boss_encounter {
            let mut rng = rand::thread_rng();
            Some(encounter.process_turn(action, &mut rng))
        } else {
            None
        };

        if let Some(result) = result {
            match result {
                TurnResult::Continue => {}
                TurnResult::Victory { xp_earned } => {
                    let boss_name = self
                        .boss_encounter
                        .as_ref()
                        .map(|e| e.boss.enemy.name.clone())
                        .unwrap_or_default();

                    // Add XP
                    if self.combat_profile.add_xp(xp_earned) {
                        self.events.push(GameEvent::LeveledUp {
                            new_level: self.combat_profile.level(),
                        });
                    }

                    // Generate guaranteed Rare+ loot
                    let mut rng = rand::thread_rng();
                    let level = self.combat_profile.level();
                    let item = generate_boss_loot(level, &mut rng);
                    let item_name = item.name.clone();
                    let rarity = item.rarity.label().to_string();
                    if self.inventory.add_item(item).is_ok() {
                        self.events
                            .push(GameEvent::ItemDropped { item_name, rarity });
                    } else {
                        self.events.push(GameEvent::InventoryFull);
                    }

                    self.events.push(GameEvent::BossDefeated {
                        xp_earned,
                        boss_name: boss_name.clone(),
                    });

                    self.status_message = Some(format!("Defeated {boss_name}! +{xp_earned} XP"));
                    self.boss_encounter = None;
                    self.game_mode = GameMode::Explore;
                    self.screen = Screen::Main;
                }
                TurnResult::Defeat { .. } => {
                    let boss_name = self
                        .boss_encounter
                        .as_ref()
                        .map(|e| e.boss.enemy.name.clone())
                        .unwrap_or_default();

                    // Lose 10% XP
                    self.combat_profile.lose_xp_percent(0.10);

                    // Force needs_care
                    self.pet.needs_care = true;

                    self.events.push(GameEvent::BossDefeat {
                        boss_name: boss_name.clone(),
                    });

                    self.status_message =
                        Some(format!("{} was defeated by {boss_name}...", self.pet.name,));
                    self.boss_encounter = None;
                    self.game_mode = GameMode::Camp;
                    self.screen = Screen::Main;
                }
                TurnResult::Fled => {
                    self.events.push(GameEvent::FledFromBoss);

                    let boss_name = self
                        .boss_encounter
                        .as_ref()
                        .map(|e| e.boss.enemy.name.clone())
                        .unwrap_or_default();
                    self.status_message = Some(format!("Escaped from {boss_name}!"));
                    self.boss_encounter = None;
                    self.game_mode = GameMode::Explore;
                    self.screen = Screen::Main;
                }
            }
        }
    }

    fn start_boss_encounter(&mut self) {
        let mut rng = rand::thread_rng();
        let level = self.combat_profile.level();
        let boss_data = boss::generate_boss(level, &mut rng);
        let eq_mods = self.inventory.total_modifiers();
        let pet_stats = battle::derive_combat_stats(
            self.pet.stats.happiness,
            self.pet.stats.energy,
            self.pet.stats.health,
            level,
            &eq_mods,
        );
        let encounter = BossEncounterState::new(boss_data, pet_stats);
        self.boss_encounter = Some(encounter);
        self.game_mode = GameMode::BossFight;
        self.screen = Screen::BossFight;
        self.boss_action_cursor = 0;
        self.status_message = Some("A powerful foe appears!".into());
    }

    fn process_events(&mut self) {
        let events = self.events.drain();
        // Check if any event is BossAvailable before processing
        let has_boss_available = events.iter().any(|e| matches!(e, GameEvent::BossAvailable));

        for event in events {
            match event {
                GameEvent::StatWarning(stat) => {
                    self.status_message = Some(format!("Warning: {} is critical!", stat));
                }
                GameEvent::Evolved { from: _, to } => {
                    self.status_message = Some(format!("{} evolved to {:?}!", self.pet.name, to));
                }
                GameEvent::Died => {
                    self.status_message = Some(format!("{} has died...", self.pet.name));
                }
                GameEvent::NeedsCare => {
                    self.status_message =
                        Some(format!("{} needs care! Stats are critical.", self.pet.name));
                }
                GameEvent::Recovered => {
                    self.status_message = Some(format!("{} is feeling better!", self.pet.name));
                }
                GameEvent::ForcedCamp => {
                    self.game_mode = GameMode::Camp;
                    self.status_message =
                        Some(format!("{} was forced back to camp!", self.pet.name));
                }
                GameEvent::BattleWon {
                    xp_earned,
                    enemy_name,
                } => {
                    self.status_message = Some(format!("Defeated {enemy_name}! +{xp_earned} XP"));
                }
                GameEvent::BattleLost { enemy_name } => {
                    self.status_message = Some(format!("Lost to {enemy_name}... but survived!"));
                }
                GameEvent::LeveledUp { new_level } => {
                    self.status_message = Some(format!("Level up! Now level {new_level}!"));
                }
                GameEvent::ItemDropped { item_name, rarity } => {
                    self.status_message = Some(format!("Loot: [{rarity}] {item_name}!"));
                }
                GameEvent::ItemEquipped { item_name, slot } => {
                    self.status_message = Some(format!("Equipped {item_name} in {slot} slot."));
                }
                GameEvent::InventoryFull => {
                    self.status_message =
                        Some("Inventory full! Discard items to pick up more loot.".into());
                }
                GameEvent::BossDefeated {
                    xp_earned,
                    boss_name,
                } => {
                    self.status_message = Some(format!("Defeated {boss_name}! +{xp_earned} XP"));
                }
                GameEvent::BossDefeat { boss_name } => {
                    self.status_message =
                        Some(format!("{} was defeated by {boss_name}...", self.pet.name));
                }
                GameEvent::FledFromBoss => {
                    // Already handled in boss_perform_action
                }
                GameEvent::BossAvailable => {
                    // Handled below
                }
                GameEvent::EnteredExplore | GameEvent::EnteredCamp => {}
                _ => {}
            }
        }

        // Start boss encounter after processing all events
        if has_boss_available && self.boss_encounter.is_none() {
            self.start_boss_encounter();
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

fn action_past_tense(action: Action) -> &'static str {
    match action {
        Action::Feed => "fed your pet",
        Action::Play => "played with your pet",
        Action::Clean => "cleaned your pet",
        Action::Sleep => "put your pet to sleep",
        _ => "did something",
    }
}

/// Generate a guaranteed Rare+ loot drop for a boss kill.
fn generate_boss_loot(level: u32, rng: &mut impl rand::Rng) -> tuigotchi_core::items::item::Item {
    use tuigotchi_core::items::item::{EquipmentSlot, Item, StatModifier, StatType};

    // Try to roll a Rare item naturally
    for _ in 0..500 {
        if let Some(item) = loot::generate_loot(level, rng) {
            if item.rarity == Rarity::Rare {
                return item;
            }
        }
    }

    // Fallback: construct a guaranteed rare item
    Item {
        name: "Boss Trophy".into(),
        rarity: Rarity::Rare,
        slot: EquipmentSlot::Accessory,
        modifiers: vec![
            StatModifier {
                stat: StatType::Attack,
                value: 2.0 + level as f32 * 1.25,
            },
            StatModifier {
                stat: StatType::Defense,
                value: 2.0 + level as f32 * 1.25,
            },
        ],
    }
}
