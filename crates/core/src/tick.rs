use crate::{
    event::{EventBus, GameEvent},
    pet::Pet,
};

/// Per-second stat decay rates.
const HUNGER_RATE: f32 = 0.5;
const HAPPINESS_RATE: f32 = 0.3;
const ENERGY_RATE: f32 = 0.2;
const HEALTH_DECAY_WHEN_STARVING: f32 = 1.0;

/// Advance the pet's state by `elapsed` seconds.
/// Returns any game events that occurred during the tick.
pub fn tick(pet: &mut Pet, elapsed: u64, events: &mut EventBus) {
    if !pet.alive {
        return;
    }

    let dt = elapsed as f32;

    // Stat decay
    pet.stats.hunger += HUNGER_RATE * dt;
    pet.stats.happiness -= HAPPINESS_RATE * dt;
    pet.stats.energy -= ENERGY_RATE * dt;

    // Starving damages health
    if pet.stats.hunger >= 100.0 {
        pet.stats.health -= HEALTH_DECAY_WHEN_STARVING * dt;
    }

    pet.stats.clamp();

    // Death check
    if pet.stats.health <= 0.0 {
        pet.alive = false;
        events.push(GameEvent::Died);
        return;
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
        tick(&mut pet, 10, &mut events);

        assert!(pet.stats.hunger > hunger_before);
        assert!(pet.stats.happiness < 50.0);
        assert!(pet.stats.energy < 100.0);
    }

    #[test]
    fn pet_dies_when_health_depleted() {
        let mut pet = Pet::new("Test");
        pet.stats.hunger = 100.0;
        pet.stats.health = 5.0;
        let mut events = EventBus::new();

        tick(&mut pet, 10, &mut events);

        assert!(!pet.alive);
        assert!(events.drain().iter().any(|e| matches!(e, GameEvent::Died)));
    }

    #[test]
    fn evolution_triggers_at_threshold() {
        let mut pet = Pet::new("Test");
        pet.stage = PetStage::Egg;
        pet.age_seconds = 29;
        let mut events = EventBus::new();

        tick(&mut pet, 1, &mut events);

        assert_eq!(pet.stage, PetStage::Baby);
        assert!(events
            .drain()
            .iter()
            .any(|e| matches!(e, GameEvent::Evolved { .. })));
    }
}
