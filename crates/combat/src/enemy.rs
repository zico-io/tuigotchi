use rand::Rng;
use serde::{Deserialize, Serialize};

/// An enemy the pet can encounter while exploring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub xp_reward: u32,
}

struct EnemyTemplate {
    name: &'static str,
    base_hp: f32,
    base_attack: f32,
    base_defense: f32,
    base_xp: u32,
}

const TEMPLATES: &[EnemyTemplate] = &[
    EnemyTemplate {
        name: "Slime",
        base_hp: 20.0,
        base_attack: 5.0,
        base_defense: 2.0,
        base_xp: 10,
    },
    EnemyTemplate {
        name: "Goblin",
        base_hp: 30.0,
        base_attack: 8.0,
        base_defense: 3.0,
        base_xp: 15,
    },
    EnemyTemplate {
        name: "Wolf",
        base_hp: 25.0,
        base_attack: 10.0,
        base_defense: 4.0,
        base_xp: 18,
    },
    EnemyTemplate {
        name: "Bat",
        base_hp: 15.0,
        base_attack: 6.0,
        base_defense: 1.0,
        base_xp: 8,
    },
    EnemyTemplate {
        name: "Skeleton",
        base_hp: 35.0,
        base_attack: 9.0,
        base_defense: 5.0,
        base_xp: 20,
    },
];

/// Generate a random enemy scaled to the pet's level.
pub fn generate_enemy(pet_level: u32, rng: &mut impl Rng) -> Enemy {
    let idx = rng.gen_range(0..TEMPLATES.len());
    let t = &TEMPLATES[idx];
    let scale = 1.0 + pet_level as f32 * 0.1;

    Enemy {
        name: t.name.to_string(),
        hp: t.base_hp * scale,
        attack: t.base_attack * scale,
        defense: t.base_defense * scale,
        xp_reward: t.base_xp + t.base_xp * pet_level / 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn enemy_generation_produces_valid_stats() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let enemy = generate_enemy(1, &mut rng);

        assert!(!enemy.name.is_empty());
        assert!(enemy.hp > 0.0);
        assert!(enemy.attack > 0.0);
        assert!(enemy.defense > 0.0);
        assert!(enemy.xp_reward > 0);
    }

    #[test]
    fn stats_scale_with_level() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let enemy_low = generate_enemy(1, &mut rng);

        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let enemy_high = generate_enemy(10, &mut rng);

        // Same template (same seed), but higher level should have higher stats
        assert!(enemy_high.hp > enemy_low.hp);
        assert!(enemy_high.attack > enemy_low.attack);
        assert!(enemy_high.xp_reward > enemy_low.xp_reward);
    }
}
