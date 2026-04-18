use tuigotchi_combat::{
    battle::{self, BattleResult},
    combat_profile::CombatProfile,
    enemy,
    explore_state::ExploreState,
};
use tuigotchi_items::{inventory::Inventory, item::StatModifier, loot};
use tuigotchi_skills::tree::AggregatedEffects;

use rand::Rng;

use crate::{
    event::{EventBus, GameEvent},
    game_state::GameMode,
    pet::Pet,
};

/// Per-second stat decay rates (tuned for ~8-hour idle window).
const HUNGER_RATE: f32 = 0.0035;
const HAPPINESS_RATE: f32 = 0.0021;
const ENERGY_RATE: f32 = 0.0014;

/// Bundles mutable references to combat state for the tick function.
pub struct CombatContext<'a> {
    pub profile: &'a mut CombatProfile,
    pub explore_state: &'a mut ExploreState,
    pub inventory: &'a mut Inventory,
    pub equipment_modifiers: Vec<StatModifier>,
}

/// Advance the pet's state by `elapsed` seconds.
/// Returns any game events that occurred during the tick.
pub fn tick(
    pet: &mut Pet,
    elapsed: u64,
    events: &mut EventBus,
    game_mode: GameMode,
    combat: Option<&mut CombatContext<'_>>,
    skill_effects: &AggregatedEffects,
) {
    if !pet.alive {
        return;
    }

    let dt = elapsed as f32;

    // Stat decay (applies in all modes), modified by skill effects
    pet.stats.hunger += HUNGER_RATE * dt * skill_effects.hunger_rate_multiplier;
    pet.stats.happiness -= HAPPINESS_RATE * dt * skill_effects.happiness_rate_multiplier;
    pet.stats.energy -= ENERGY_RATE * dt * skill_effects.energy_rate_multiplier;

    pet.stats.clamp();

    // Needs-care check (replaces death-from-starvation in v2)
    if pet.check_needs_care() && !pet.needs_care {
        pet.needs_care = true;
        events.push(GameEvent::NeedsCare);

        // If exploring, force back to camp
        if game_mode == GameMode::Explore {
            events.push(GameEvent::ForcedCamp);
        }
    } else if pet.needs_care && pet.check_recovered() {
        pet.needs_care = false;
        events.push(GameEvent::Recovered);
    }

    // Mode-specific tick logic
    #[allow(unreachable_patterns)]
    match game_mode {
        GameMode::Camp => {
            // Camp: decay only (already applied above)
        }
        GameMode::Explore => {
            if let Some(ctx) = combat {
                if !pet.needs_care {
                    run_combat_tick(pet, events, ctx, skill_effects);
                }
            }
        }
        _ => {}
    }

    // Stat warnings
    if pet.stats.hunger >= 80.0 {
        events.push(GameEvent::StatWarning("hunger"));
    }
    if pet.stats.happiness <= 20.0 {
        events.push(GameEvent::StatWarning("happiness"));
    }
    if pet.stats.energy <= 10.0 {
        events.push(GameEvent::StatWarning("energy"));
    }

    // Age & evolution
    pet.age_seconds += elapsed;
    if let Some(threshold) = pet.stage.evolution_threshold() {
        if pet.age_seconds >= threshold {
            if let Some(next) = pet.stage.next() {
                let prev = pet.stage;
                pet.stage = next;
                events.push(GameEvent::Evolved {
                    from: prev,
                    to: next,
                });
            }
        }
    }
}

/// Run a single combat encounter during an explore tick.
fn run_combat_tick(
    pet: &Pet,
    events: &mut EventBus,
    ctx: &mut CombatContext<'_>,
    skill_effects: &AggregatedEffects,
) {
    let mut rng = rand::thread_rng();
    let level = ctx.profile.level();
    let foe = enemy::generate_enemy(level, &mut rng);
    let stats = battle::derive_combat_stats(
        pet.stats.happiness,
        pet.stats.energy,
        pet.stats.health,
        level,
        &ctx.equipment_modifiers,
    );

    let result = battle::resolve_auto_battle(&stats, &foe, &mut rng);

    match result {
        BattleResult::Victory {
            xp_earned,
            damage_taken,
        } => {
            // Apply XP multiplier from skills
            let effective_xp = (xp_earned as f32 * skill_effects.xp_multiplier).round() as u32;

            ctx.explore_state.battles_won += 1;
            ctx.explore_state.total_xp_earned += effective_xp;
            ctx.explore_state.battles_since_boss += 1;
            ctx.explore_state.last_battle_log = Some(format!(
                "Defeated {}! +{} XP (took {:.0} damage)",
                foe.name, effective_xp, damage_taken,
            ));

            events.push(GameEvent::BattleWon {
                xp_earned: effective_xp,
                enemy_name: foe.name.clone(),
            });

            if ctx.profile.add_xp(effective_xp) {
                events.push(GameEvent::LeveledUp {
                    new_level: ctx.profile.level(),
                });
            }

            // Attempt loot drop with skill bonus
            let loot_chance = (0.30 + skill_effects.loot_chance_bonus).min(1.0);
            if rng.gen_bool(loot_chance as f64) {
                let item = loot::generate_loot_guaranteed(level, &mut rng);
                let item_name = item.name.clone();
                let rarity = item.rarity.label().to_string();
                if ctx.inventory.add_item(item).is_ok() {
                    events.push(GameEvent::ItemDropped { item_name, rarity });
                } else {
                    events.push(GameEvent::InventoryFull);
                }
            }

            // Check for boss availability
            if ctx.explore_state.battles_since_boss >= 50 {
                ctx.explore_state.battles_since_boss = 0;
                events.push(GameEvent::BossAvailable);
            }
        }
        BattleResult::Defeat { damage_taken } => {
            ctx.explore_state.battles_lost += 1;
            ctx.explore_state.last_battle_log = Some(format!(
                "Lost to {} (took {:.0} damage)",
                foe.name, damage_taken,
            ));

            events.push(GameEvent::BattleLost {
                enemy_name: foe.name,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pet::{Pet, PetStage};
    use tuigotchi_items::inventory::Inventory;

    fn default_effects() -> AggregatedEffects {
        AggregatedEffects::default()
    }

    #[test]
    fn stat_decay_over_time() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Baby;
        let mut events = EventBus::new();

        let hunger_before = pet.stats.hunger;
        tick(
            &mut pet,
            1000,
            &mut events,
            GameMode::Camp,
            None,
            &default_effects(),
        );

        assert!(pet.stats.hunger > hunger_before);
        assert!(pet.stats.happiness < 50.0);
        assert!(pet.stats.energy < 100.0);
    }

    #[test]
    fn starvation_triggers_needs_care_not_death() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        pet.stats.hunger = 89.0;
        let mut events = EventBus::new();

        // Push hunger past 90 threshold
        tick(
            &mut pet,
            1000,
            &mut events,
            GameMode::Camp,
            None,
            &default_effects(),
        );

        assert!(pet.alive); // no death from starvation in v2
        assert!(pet.needs_care);
        assert!(events
            .drain()
            .iter()
            .any(|e| matches!(e, GameEvent::NeedsCare)));
    }

    #[test]
    fn decay_rates_are_idle_scaled() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut events = EventBus::new();

        // After 1 hour (3600s), stats should have changed modestly
        tick(
            &mut pet,
            3600,
            &mut events,
            GameMode::Camp,
            None,
            &default_effects(),
        );

        // Hunger: 50 + 0.0035 * 3600 = 62.6
        assert!((pet.stats.hunger - 62.6).abs() < 0.1);
        // Happiness: 50 - 0.0021 * 3600 = 42.44
        assert!((pet.stats.happiness - 42.44).abs() < 0.1);
        // Energy: 100 - 0.0014 * 3600 = 94.96
        assert!((pet.stats.energy - 94.96).abs() < 0.1);
        // Not starving yet, health unchanged
        assert!((pet.stats.health - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn evolution_triggers_at_threshold() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Egg;
        pet.age_seconds = 29;
        let mut events = EventBus::new();

        tick(
            &mut pet,
            1,
            &mut events,
            GameMode::Camp,
            None,
            &default_effects(),
        );

        assert_eq!(pet.stage, PetStage::Baby);
        assert!(events
            .drain()
            .iter()
            .any(|e| matches!(e, GameEvent::Evolved { .. })));
    }

    #[test]
    fn explore_mode_forces_camp_on_needs_care() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        pet.stats.hunger = 89.0;
        let mut events = EventBus::new();

        // Push hunger past 90 threshold while exploring
        tick(
            &mut pet,
            1000,
            &mut events,
            GameMode::Explore,
            None,
            &default_effects(),
        );

        assert!(pet.needs_care);
        let drained = events.drain();
        assert!(drained.iter().any(|e| matches!(e, GameEvent::NeedsCare)));
        assert!(drained.iter().any(|e| matches!(e, GameEvent::ForcedCamp)));
    }

    #[test]
    fn explore_mode_decays_stats() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut events = EventBus::new();

        let hunger_before = pet.stats.hunger;
        tick(
            &mut pet,
            100,
            &mut events,
            GameMode::Explore,
            None,
            &default_effects(),
        );

        // Decay still applies in Explore mode
        assert!(pet.stats.hunger > hunger_before);
    }

    #[test]
    fn explore_with_combat_produces_battle_events() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut events = EventBus::new();
        let mut profile = CombatProfile::new();
        let mut explore = ExploreState::default();
        let mut inventory = Inventory::new(20);
        let eq_mods = inventory.total_modifiers();

        tick(
            &mut pet,
            1,
            &mut events,
            GameMode::Explore,
            Some(&mut CombatContext {
                profile: &mut profile,
                explore_state: &mut explore,
                inventory: &mut inventory,
                equipment_modifiers: eq_mods,
            }),
            &default_effects(),
        );

        let drained = events.drain();
        // Should have at least one battle event (won or lost)
        let has_battle = drained.iter().any(|e| {
            matches!(
                e,
                GameEvent::BattleWon { .. } | GameEvent::BattleLost { .. }
            )
        });
        assert!(has_battle);
        assert!(explore.battles_won + explore.battles_lost > 0);
        assert!(explore.last_battle_log.is_some());
    }

    #[test]
    fn combat_tick_can_produce_loot() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut inventory = Inventory::new(100);

        // Run many ticks to get at least one loot drop
        for _ in 0..50 {
            let mut events = EventBus::new();
            let mut profile = CombatProfile::new();
            let mut explore = ExploreState::default();
            let eq_mods = inventory.total_modifiers();

            tick(
                &mut pet,
                1,
                &mut events,
                GameMode::Explore,
                Some(&mut CombatContext {
                    profile: &mut profile,
                    explore_state: &mut explore,
                    inventory: &mut inventory,
                    equipment_modifiers: eq_mods,
                }),
                &default_effects(),
            );
        }

        // With 50 battles and 30% drop rate, we should have some items
        assert!(
            !inventory.is_empty(),
            "should have received at least one loot drop in 50 battles"
        );
    }

    #[test]
    fn boss_available_triggers_at_50_battles() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut inventory = Inventory::new(200);

        let mut profile = CombatProfile::new();
        let mut explore = ExploreState::default();

        // Run 50 ticks (each produces one battle)
        let mut boss_available_seen = false;
        for _ in 0..60 {
            let mut events = EventBus::new();
            let eq_mods = inventory.total_modifiers();

            tick(
                &mut pet,
                1,
                &mut events,
                GameMode::Explore,
                Some(&mut CombatContext {
                    profile: &mut profile,
                    explore_state: &mut explore,
                    inventory: &mut inventory,
                    equipment_modifiers: eq_mods,
                }),
                &default_effects(),
            );

            for event in events.drain() {
                if matches!(event, GameEvent::BossAvailable) {
                    boss_available_seen = true;
                }
            }
        }

        // After 50+ battles won, we should see BossAvailable
        // (Some battles may be lost, so we run 60 to be safe)
        assert!(
            boss_available_seen || explore.battles_won < 50,
            "BossAvailable should fire after 50 victories; wins={}, battles_since_boss={}",
            explore.battles_won,
            explore.battles_since_boss,
        );
    }

    #[test]
    fn skill_decay_modifiers_reduce_decay() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Adult;
        pet.age_seconds = 600;
        let mut events = EventBus::new();

        // Half hunger rate
        let effects = AggregatedEffects {
            hunger_rate_multiplier: 0.5,
            ..AggregatedEffects::default()
        };

        tick(&mut pet, 3600, &mut events, GameMode::Camp, None, &effects);

        // Hunger: 50 + 0.0035 * 0.5 * 3600 = 56.3
        assert!((pet.stats.hunger - 56.3).abs() < 0.1);
    }
}
