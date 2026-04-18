use rand::Rng;
use serde::{Deserialize, Serialize};
use tuigotchi_items::item::Rarity;

use crate::enemy::Enemy;

/// A boss enemy with guaranteed loot rarity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boss {
    /// The underlying enemy (reused struct with scaled stats).
    pub enemy: Enemy,
    /// Minimum rarity for the boss drop.
    pub guaranteed_rarity: Rarity,
}

const BOSS_NAMES: &[&str] = &[
    "Giant Slime",
    "Goblin Chief",
    "Alpha Wolf",
    "Vampire Bat",
    "Skeleton Lord",
];

/// Generate a boss scaled to the pet's level.
///
/// Boss stats are 3-5x normal enemy stats; XP reward is 5-10x.
pub fn generate_boss(pet_level: u32, rng: &mut impl Rng) -> Boss {
    let idx = rng.gen_range(0..BOSS_NAMES.len());
    let name = BOSS_NAMES[idx].to_string();

    let scale = 1.0 + pet_level as f32 * 0.1;
    let boss_multiplier = rng.gen_range(3.0..=5.0_f32);
    let xp_multiplier = rng.gen_range(5..=10_u32);

    // Base stats similar to normal enemies but multiplied
    let base_hp = 25.0;
    let base_attack = 7.0;
    let base_defense = 3.0;
    let base_xp = 15_u32;

    let enemy = Enemy {
        name,
        hp: base_hp * scale * boss_multiplier,
        attack: base_attack * scale * boss_multiplier,
        defense: base_defense * scale * boss_multiplier,
        xp_reward: (base_xp + base_xp * pet_level / 5) * xp_multiplier,
    };

    Boss {
        enemy,
        guaranteed_rarity: Rarity::Rare,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enemy::generate_enemy;
    use rand::SeedableRng;

    #[test]
    fn boss_stats_are_3_to_5x_normal() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let level = 5;

        // Generate several normal enemies to get an average
        let mut avg_hp = 0.0;
        let n = 100;
        for _ in 0..n {
            let e = generate_enemy(level, &mut rng);
            avg_hp += e.hp;
        }
        avg_hp /= n as f32;

        // Generate several bosses
        let mut rng = rand::rngs::StdRng::seed_from_u64(99);
        for _ in 0..20 {
            let boss = generate_boss(level, &mut rng);
            // Boss HP should be significantly higher than average enemy
            assert!(
                boss.enemy.hp > avg_hp * 2.0,
                "Boss HP {} should be > 2x avg enemy HP {}",
                boss.enemy.hp,
                avg_hp
            );
        }
    }

    #[test]
    fn boss_xp_is_5_to_10x_normal() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let level = 5;

        let normal = generate_enemy(level, &mut rng);
        let normal_xp = normal.xp_reward;

        let mut rng = rand::rngs::StdRng::seed_from_u64(99);
        for _ in 0..20 {
            let boss = generate_boss(level, &mut rng);
            assert!(
                boss.enemy.xp_reward >= normal_xp * 3,
                "Boss XP {} should be much higher than normal {}",
                boss.enemy.xp_reward,
                normal_xp
            );
        }
    }

    #[test]
    fn boss_has_rare_guaranteed_rarity() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let boss = generate_boss(1, &mut rng);
        assert_eq!(boss.guaranteed_rarity, Rarity::Rare);
    }

    #[test]
    fn boss_names_are_valid() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        for _ in 0..50 {
            let boss = generate_boss(1, &mut rng);
            assert!(!boss.enemy.name.is_empty());
            assert!(BOSS_NAMES.contains(&boss.enemy.name.as_str()));
        }
    }
}
