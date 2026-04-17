use crate::{
    event::{EventBus, GameEvent},
    game_state::GameMode,
    pet::Pet,
};

/// Per-second stat decay rates (tuned for ~8-hour idle window).
const HUNGER_RATE: f32 = 0.0035;
const HAPPINESS_RATE: f32 = 0.0021;
const ENERGY_RATE: f32 = 0.0014;

/// Advance the pet's state by `elapsed` seconds.
/// Returns any game events that occurred during the tick.
pub fn tick(pet: &mut Pet, elapsed: u64, events: &mut EventBus, game_mode: GameMode) {
    if !pet.alive {
        return;
    }

    let dt = elapsed as f32;

    // Stat decay (applies in all modes)
    pet.stats.hunger += HUNGER_RATE * dt;
    pet.stats.happiness -= HAPPINESS_RATE * dt;
    pet.stats.energy -= ENERGY_RATE * dt;

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
            // TODO: Phase 3 — combat tick, random encounters
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pet::{Pet, PetStage};

    #[test]
    fn stat_decay_over_time() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Baby;
        let mut events = EventBus::new();

        let hunger_before = pet.stats.hunger;
        tick(&mut pet, 1000, &mut events, GameMode::Camp);

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
        tick(&mut pet, 1000, &mut events, GameMode::Camp);

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
        tick(&mut pet, 3600, &mut events, GameMode::Camp);

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

        tick(&mut pet, 1, &mut events, GameMode::Camp);

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
        tick(&mut pet, 1000, &mut events, GameMode::Explore);

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
        tick(&mut pet, 100, &mut events, GameMode::Explore);

        // Decay still applies in Explore mode
        assert!(pet.stats.hunger > hunger_before);
    }
}
