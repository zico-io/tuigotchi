use tuigotchi_combat::{combat_profile::CombatProfile, explore_state::ExploreState};
use tuigotchi_items::inventory::Inventory;

use crate::{
    event::{EventBus, GameEvent},
    game_state::GameMode,
    pet::Pet,
};

/// Per-second stat decay rates — must match tick.rs.
const HUNGER_RATE: f32 = 0.0035;
const HAPPINESS_RATE: f32 = 0.0021;
const ENERGY_RATE: f32 = 0.0014;

/// Optional combat state for offline simulation.
pub struct OfflineCombatContext<'a> {
    pub profile: &'a mut CombatProfile,
    pub explore_state: &'a mut ExploreState,
    pub inventory: &'a mut Inventory,
    pub game_mode: GameMode,
}

/// Simulate elapsed offline time analytically (no second-by-second loop).
///
/// Applies stat decay in batch, steps through evolution thresholds,
/// and checks needs-care status. Returns a summary of what happened.
pub fn simulate_offline(
    pet: &mut Pet,
    elapsed_seconds: u64,
    events: &mut EventBus,
    combat: Option<&mut OfflineCombatContext<'_>>,
) -> OfflineSummary {
    if !pet.alive || elapsed_seconds == 0 {
        return OfflineSummary::default();
    }

    let dt = elapsed_seconds as f32;
    let mut summary = OfflineSummary {
        elapsed_seconds,
        ..Default::default()
    };

    // Batch stat decay
    let hunger_before = pet.stats.hunger;
    let happiness_before = pet.stats.happiness;
    let energy_before = pet.stats.energy;

    pet.stats.hunger += HUNGER_RATE * dt;
    pet.stats.happiness -= HAPPINESS_RATE * dt;
    pet.stats.energy -= ENERGY_RATE * dt;
    pet.stats.clamp();

    summary.hunger_change = pet.stats.hunger - hunger_before;
    summary.happiness_change = pet.stats.happiness - happiness_before;
    summary.energy_change = pet.stats.energy - energy_before;

    // Step through evolution thresholds
    let remaining_age = pet.age_seconds + elapsed_seconds;
    while let Some(threshold) = pet.stage.evolution_threshold() {
        if remaining_age >= threshold {
            if let Some(next) = pet.stage.next() {
                let prev = pet.stage;
                pet.stage = next;
                summary.evolutions += 1;
                events.push(GameEvent::Evolved {
                    from: prev,
                    to: next,
                });
            } else {
                break;
            }
        } else {
            break;
        }
    }
    pet.age_seconds = remaining_age;

    // Check needs-care
    if pet.check_needs_care() && !pet.needs_care {
        pet.needs_care = true;
        events.push(GameEvent::NeedsCare);
    }

    // Offline combat simulation
    if let Some(ctx) = combat {
        if ctx.game_mode == GameMode::Explore {
            let level = ctx.profile.level();
            let avg_xp = 10 + level * 2;
            // 1 battle per second offline, cap at a reasonable amount
            let battles = elapsed_seconds.min(28800) as u32; // cap at 8 hours
            let total_xp = battles * avg_xp;

            ctx.explore_state.battles_won += battles;
            ctx.explore_state.total_xp_earned += total_xp;

            let level_before = ctx.profile.level();
            ctx.profile.add_xp(total_xp);
            let level_after = ctx.profile.level();

            summary.battles_won = battles;
            summary.xp_earned = total_xp;
            summary.levels_gained = level_after - level_before;

            if summary.levels_gained > 0 {
                events.push(GameEvent::LeveledUp {
                    new_level: level_after,
                });
            }

            // Simplified offline loot: ~30% of battles produce items
            let estimated_drops = (battles as f32 * 0.30) as u32;
            let mut rng = rand::thread_rng();
            let mut items_found = 0u32;
            for _ in 0..estimated_drops {
                if ctx.inventory.is_full() {
                    break;
                }
                if let Some(item) = tuigotchi_items::loot::generate_loot(level, &mut rng) {
                    if ctx.inventory.add_item(item).is_ok() {
                        items_found += 1;
                    }
                }
            }
            summary.items_found = items_found;

            // Check if enough battles for a boss (skip actual boss encounter offline)
            if ctx.explore_state.battles_since_boss + battles >= 50 {
                summary.boss_available = true;
                // Reset counter as if the boss was skipped
                ctx.explore_state.battles_since_boss =
                    (ctx.explore_state.battles_since_boss + battles) % 50;
            } else {
                ctx.explore_state.battles_since_boss += battles;
            }
        }
    }

    summary
}

/// Summary of what happened during offline simulation.
#[derive(Debug, Clone, Default)]
pub struct OfflineSummary {
    pub elapsed_seconds: u64,
    pub hunger_change: f32,
    pub happiness_change: f32,
    pub energy_change: f32,
    pub evolutions: u32,
    pub battles_won: u32,
    pub xp_earned: u32,
    pub levels_gained: u32,
    pub items_found: u32,
    /// Whether a boss encounter was available during offline time.
    pub boss_available: bool,
}

impl OfflineSummary {
    /// Format as a human-readable welcome-back message.
    pub fn message(&self) -> String {
        if self.elapsed_seconds == 0 {
            return String::new();
        }

        let hours = self.elapsed_seconds / 3600;
        let minutes = (self.elapsed_seconds % 3600) / 60;

        let time_str = if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        };

        let mut parts = vec![format!("Welcome back! You were away for {time_str}.")];

        if self.evolutions > 0 {
            parts.push(format!(
                "Your pet evolved {} time{}!",
                self.evolutions,
                if self.evolutions > 1 { "s" } else { "" }
            ));
        }

        if self.battles_won > 0 {
            parts.push(format!(
                "Won {} battles, earned {} XP.",
                self.battles_won, self.xp_earned,
            ));
        }

        if self.levels_gained > 0 {
            parts.push(format!(
                "Gained {} level{}!",
                self.levels_gained,
                if self.levels_gained > 1 { "s" } else { "" }
            ));
        }

        if self.items_found > 0 {
            parts.push(format!(
                "Found {} item{}!",
                self.items_found,
                if self.items_found > 1 { "s" } else { "" }
            ));
        }

        if self.boss_available {
            parts.push("A boss appeared while you were away!".into());
        }

        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pet::{Pet, PetStage};
    use tuigotchi_items::inventory::Inventory;

    #[test]
    fn offline_one_hour_decay() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut events = EventBus::new();

        let summary = simulate_offline(&mut pet, 3600, &mut events, None);

        // Hunger: 50 + 0.0035 * 3600 = 62.6
        assert!((pet.stats.hunger - 62.6).abs() < 0.1);
        assert!(summary.hunger_change > 0.0);
        assert!(summary.happiness_change < 0.0);
        assert!(!pet.needs_care);
    }

    #[test]
    fn offline_triggers_needs_care() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        pet.stats.hunger = 85.0;
        let mut events = EventBus::new();

        // 2000s of hunger at 0.0035/s = +7.0 → 92.0 >= 90 threshold
        simulate_offline(&mut pet, 2000, &mut events, None);

        assert!(pet.needs_care);
        assert!(events
            .drain()
            .iter()
            .any(|e| matches!(e, GameEvent::NeedsCare)));
    }

    #[test]
    fn offline_evolution_step_through() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Egg;
        pet.age_seconds = 0;
        let mut events = EventBus::new();

        // 500 seconds should evolve Egg→Baby (30s) and Baby→Teen (120s)
        // and Teen→Adult (300s)
        let summary = simulate_offline(&mut pet, 500, &mut events, None);

        assert_eq!(pet.stage, PetStage::Adult);
        assert_eq!(summary.evolutions, 3);
    }

    #[test]
    fn offline_zero_elapsed_is_noop() {
        let mut pet = Pet::new("Test");
        let hunger_before = pet.stats.hunger;
        let mut events = EventBus::new();

        simulate_offline(&mut pet, 0, &mut events, None);

        assert!((pet.stats.hunger - hunger_before).abs() < f32::EPSILON);
        assert!(events.is_empty());
    }

    #[test]
    fn pet_survives_eight_hours_offline() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut events = EventBus::new();

        simulate_offline(&mut pet, 28800, &mut events, None);

        // Pet is alive (no death from starvation in v2)
        assert!(pet.alive);
        // But needs care
        assert!(pet.needs_care);
    }

    #[test]
    fn summary_message_formatting() {
        let summary = OfflineSummary {
            elapsed_seconds: 7200,
            hunger_change: 25.2,
            happiness_change: -15.12,
            energy_change: -10.08,
            evolutions: 1,
            battles_won: 0,
            xp_earned: 0,
            levels_gained: 0,
            items_found: 0,
            boss_available: false,
        };

        let msg = summary.message();
        assert!(msg.contains("2h 0m"));
        assert!(msg.contains("evolved 1 time"));
    }

    #[test]
    fn offline_with_combat_produces_xp() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut events = EventBus::new();
        let mut profile = CombatProfile::new();
        let mut explore = ExploreState::default();
        let mut inventory = Inventory::new(100);

        let summary = simulate_offline(
            &mut pet,
            100,
            &mut events,
            Some(&mut OfflineCombatContext {
                profile: &mut profile,
                explore_state: &mut explore,
                inventory: &mut inventory,
                game_mode: GameMode::Explore,
            }),
        );

        // 100 battles at avg_xp = 10 + 1*2 = 12 each = 1200 XP total
        assert_eq!(summary.battles_won, 100);
        assert_eq!(summary.xp_earned, 1200);
        assert!(summary.levels_gained > 0);
        assert!(explore.battles_won == 100);
        assert!(explore.total_xp_earned == 1200);
    }

    #[test]
    fn offline_combat_in_camp_mode_does_nothing() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut events = EventBus::new();
        let mut profile = CombatProfile::new();
        let mut explore = ExploreState::default();
        let mut inventory = Inventory::new(20);

        let summary = simulate_offline(
            &mut pet,
            100,
            &mut events,
            Some(&mut OfflineCombatContext {
                profile: &mut profile,
                explore_state: &mut explore,
                inventory: &mut inventory,
                game_mode: GameMode::Camp,
            }),
        );

        assert_eq!(summary.battles_won, 0);
        assert_eq!(summary.xp_earned, 0);
    }

    #[test]
    fn offline_combat_generates_loot() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut events = EventBus::new();
        let mut profile = CombatProfile::new();
        let mut explore = ExploreState::default();
        let mut inventory = Inventory::new(100);

        let summary = simulate_offline(
            &mut pet,
            1000,
            &mut events,
            Some(&mut OfflineCombatContext {
                profile: &mut profile,
                explore_state: &mut explore,
                inventory: &mut inventory,
                game_mode: GameMode::Explore,
            }),
        );

        assert!(
            summary.items_found > 0,
            "1000 battles should produce some loot"
        );
        assert_eq!(inventory.len(), summary.items_found as usize);
    }

    #[test]
    fn summary_message_includes_combat() {
        let summary = OfflineSummary {
            elapsed_seconds: 3600,
            hunger_change: 12.6,
            happiness_change: -7.56,
            energy_change: -5.04,
            evolutions: 0,
            battles_won: 3600,
            xp_earned: 43200,
            levels_gained: 5,
            items_found: 0,
            boss_available: false,
        };

        let msg = summary.message();
        assert!(msg.contains("Won 3600 battles"));
        assert!(msg.contains("43200 XP"));
        assert!(msg.contains("Gained 5 levels"));
    }
}
