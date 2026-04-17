use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::enemy::Enemy;

/// Derived combat stats for the pet, based on its care stats and level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatStats {
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub max_hp: f32,
}

/// Derive combat stats from the pet's care stats and combat level.
pub fn derive_combat_stats(happiness: f32, energy: f32, health: f32, level: u32) -> CombatStats {
    let lv = level as f32;
    CombatStats {
        attack: 10.0 + (happiness + energy) * 0.1 + lv * 2.0,
        defense: 5.0 + health * 0.15 + lv * 1.5,
        speed: 5.0 + energy * 0.1 + lv * 1.0,
        max_hp: 50.0 + health * 0.5 + lv * 5.0,
    }
}

/// Outcome of an auto-battle.
#[derive(Debug, Clone)]
pub enum BattleResult {
    /// The pet won.
    Victory { xp_earned: u32, damage_taken: f32 },
    /// The pet lost (non-lethal).
    Defeat { damage_taken: f32 },
}

/// Resolve an auto-battle between the pet and an enemy.
///
/// The system is designed to heavily favor the player: victories are common,
/// defeats are rare and non-lethal.
pub fn resolve_auto_battle(stats: &CombatStats, enemy: &Enemy, rng: &mut impl Rng) -> BattleResult {
    let pet_damage = (stats.attack - enemy.defense * 0.5).max(1.0);
    let enemy_damage = (enemy.attack - stats.defense * 0.5).max(1.0);

    // Slight random variance (+/- 20%)
    let pet_variance = rng.gen_range(0.8..1.2);
    let enemy_variance = rng.gen_range(0.8..1.2);
    let pet_damage = pet_damage * pet_variance;
    let enemy_damage = enemy_damage * enemy_variance;

    let mut pet_hp = stats.max_hp;
    let mut enemy_hp = enemy.hp;
    let mut total_damage_taken = 0.0_f32;

    // Pet attacks first if faster (which is most of the time)
    let pet_first = stats.speed >= 5.0; // effectively always true given the formula

    loop {
        if pet_first {
            enemy_hp -= pet_damage;
            if enemy_hp <= 0.0 {
                return BattleResult::Victory {
                    xp_earned: enemy.xp_reward,
                    damage_taken: total_damage_taken,
                };
            }
            pet_hp -= enemy_damage;
            total_damage_taken += enemy_damage;
            if pet_hp <= 0.0 {
                return BattleResult::Defeat {
                    damage_taken: total_damage_taken,
                };
            }
        } else {
            pet_hp -= enemy_damage;
            total_damage_taken += enemy_damage;
            if pet_hp <= 0.0 {
                return BattleResult::Defeat {
                    damage_taken: total_damage_taken,
                };
            }
            enemy_hp -= pet_damage;
            if enemy_hp <= 0.0 {
                return BattleResult::Victory {
                    xp_earned: enemy.xp_reward,
                    damage_taken: total_damage_taken,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enemy::Enemy;
    use rand::SeedableRng;

    #[test]
    fn derive_combat_stats_produces_sane_values() {
        let stats = derive_combat_stats(50.0, 100.0, 100.0, 1);
        assert!(stats.attack > 10.0);
        assert!(stats.defense > 5.0);
        assert!(stats.speed > 5.0);
        assert!(stats.max_hp > 50.0);
    }

    #[test]
    fn auto_battle_against_weak_enemy_is_victory() {
        let stats = derive_combat_stats(50.0, 100.0, 100.0, 5);
        let weak_enemy = Enemy {
            name: "Slime".into(),
            hp: 10.0,
            attack: 2.0,
            defense: 1.0,
            xp_reward: 5,
        };
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let result = resolve_auto_battle(&stats, &weak_enemy, &mut rng);
        assert!(matches!(result, BattleResult::Victory { .. }));
    }

    #[test]
    fn combat_stats_scale_with_level() {
        let low = derive_combat_stats(50.0, 50.0, 50.0, 1);
        let high = derive_combat_stats(50.0, 50.0, 50.0, 10);
        assert!(high.attack > low.attack);
        assert!(high.defense > low.defense);
        assert!(high.max_hp > low.max_hp);
    }
}
