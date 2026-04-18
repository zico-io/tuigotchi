use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{battle::CombatStats, boss::Boss};

/// Actions the player can take during a boss fight.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatAction {
    Attack,
    Defend,
    Flee,
}

impl CombatAction {
    /// Human-readable label.
    #[allow(unreachable_patterns)]
    pub fn label(self) -> &'static str {
        match self {
            Self::Attack => "Attack",
            Self::Defend => "Defend",
            Self::Flee => "Flee",
            _ => "???",
        }
    }
}

/// All available combat actions in order.
pub const ALL_COMBAT_ACTIONS: &[CombatAction] = &[
    CombatAction::Attack,
    CombatAction::Defend,
    CombatAction::Flee,
];

/// Tracks the state of a boss encounter (turn-based manual combat).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossEncounterState {
    pub boss: Boss,
    pub pet_hp: f32,
    pub pet_max_hp: f32,
    pub boss_hp: f32,
    pub boss_max_hp: f32,
    pub pet_stats: CombatStats,
    pub turn: u32,
    pub log: Vec<String>,
    pub defending: bool,
}

/// Result of processing one turn of combat.
#[derive(Debug, Clone)]
pub enum TurnResult {
    /// Combat continues.
    Continue,
    /// The pet won! Includes XP earned.
    Victory { xp_earned: u32 },
    /// The pet lost. Includes total damage taken.
    Defeat { damage_taken: f32 },
    /// The pet fled successfully.
    Fled,
}

impl BossEncounterState {
    /// Create a new boss encounter from a boss and the pet's combat stats.
    pub fn new(boss: Boss, pet_stats: CombatStats) -> Self {
        let boss_max_hp = boss.enemy.hp;
        let pet_max_hp = pet_stats.max_hp;

        Self {
            boss,
            pet_hp: pet_max_hp,
            pet_max_hp,
            boss_hp: boss_max_hp,
            boss_max_hp,
            pet_stats,
            turn: 0,
            log: Vec::new(),
            defending: false,
        }
    }

    /// Process one turn of combat given the player's action.
    #[allow(unreachable_patterns)]
    pub fn process_turn(&mut self, action: CombatAction, rng: &mut impl Rng) -> TurnResult {
        self.turn += 1;
        self.defending = false;

        match action {
            CombatAction::Attack => self.process_attack(rng),
            CombatAction::Defend => self.process_defend(rng),
            CombatAction::Flee => self.process_flee(rng),
            _ => TurnResult::Continue,
        }
    }

    fn process_attack(&mut self, rng: &mut impl Rng) -> TurnResult {
        // Pet attacks boss
        let variance = rng.gen_range(0.8..1.2_f32);
        let pet_damage =
            (self.pet_stats.attack - self.boss.enemy.defense * 0.3).max(1.0) * variance;
        self.boss_hp -= pet_damage;

        self.log
            .push(format!("You attack for {:.0} damage!", pet_damage));

        if self.boss_hp <= 0.0 {
            self.boss_hp = 0.0;
            self.log
                .push(format!("{} has been defeated!", self.boss.enemy.name));
            return TurnResult::Victory {
                xp_earned: self.boss.enemy.xp_reward,
            };
        }

        // Boss counter-attacks
        self.boss_attacks(rng)
    }

    fn process_defend(&mut self, rng: &mut impl Rng) -> TurnResult {
        self.defending = true;
        self.log.push("You brace for impact!".into());

        // Boss attacks (damage halved due to defending)
        self.boss_attacks(rng)
    }

    fn process_flee(&mut self, rng: &mut impl Rng) -> TurnResult {
        if rng.gen_bool(0.50) {
            self.log.push("You escaped!".into());
            TurnResult::Fled
        } else {
            self.log.push("Couldn't escape!".into());
            // Boss gets a free attack
            self.boss_attacks(rng)
        }
    }

    fn boss_attacks(&mut self, rng: &mut impl Rng) -> TurnResult {
        let variance = rng.gen_range(0.8..1.2_f32);
        let mut boss_damage =
            (self.boss.enemy.attack - self.pet_stats.defense * 0.3).max(1.0) * variance;

        if self.defending {
            boss_damage *= 0.5;
        }

        self.pet_hp -= boss_damage;

        let defend_note = if self.defending { " (blocked!)" } else { "" };
        self.log.push(format!(
            "{} attacks for {:.0} damage!{}",
            self.boss.enemy.name, boss_damage, defend_note,
        ));

        if self.pet_hp <= 0.0 {
            self.pet_hp = 0.0;
            let total_damage = self.pet_max_hp; // took enough to go down
            self.log.push("You have been defeated...".into());
            return TurnResult::Defeat {
                damage_taken: total_damage,
            };
        }

        TurnResult::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boss::generate_boss;
    use rand::SeedableRng;

    fn test_stats() -> CombatStats {
        CombatStats {
            attack: 50.0,
            defense: 20.0,
            speed: 15.0,
            max_hp: 200.0,
        }
    }

    #[test]
    fn attack_reduces_boss_hp() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let boss = generate_boss(1, &mut rng);
        let initial_boss_hp = boss.enemy.hp;
        let stats = test_stats();
        let mut encounter = BossEncounterState::new(boss, stats);

        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        encounter.process_turn(CombatAction::Attack, &mut rng);

        assert!(
            encounter.boss_hp < initial_boss_hp,
            "Boss HP should decrease after attack"
        );
    }

    #[test]
    fn defend_reduces_damage_taken() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let boss = generate_boss(1, &mut rng);
        let stats = test_stats();

        // Attack turn
        let mut enc_attack = BossEncounterState::new(boss.clone(), stats.clone());
        let mut rng1 = rand::rngs::StdRng::seed_from_u64(100);
        enc_attack.process_turn(CombatAction::Attack, &mut rng1);
        let hp_after_attack = enc_attack.pet_hp;

        // Defend turn
        let mut enc_defend = BossEncounterState::new(boss, stats);
        let mut rng2 = rand::rngs::StdRng::seed_from_u64(100);
        enc_defend.process_turn(CombatAction::Defend, &mut rng2);
        let hp_after_defend = enc_defend.pet_hp;

        // Defend should result in more HP remaining (less damage taken)
        assert!(
            hp_after_defend > hp_after_attack,
            "Defending should reduce damage: defend HP {} > attack HP {}",
            hp_after_defend,
            hp_after_attack,
        );
    }

    #[test]
    fn victory_when_boss_hp_reaches_zero() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let boss = generate_boss(1, &mut rng);
        let stats = CombatStats {
            attack: 5000.0, // massive damage
            defense: 9999.0,
            speed: 15.0,
            max_hp: 9999.0,
        };
        let mut encounter = BossEncounterState::new(boss, stats);

        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let result = encounter.process_turn(CombatAction::Attack, &mut rng);
        assert!(
            matches!(result, TurnResult::Victory { .. }),
            "Should be victory with massive attack"
        );
    }

    #[test]
    fn defeat_when_pet_hp_reaches_zero() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let boss = generate_boss(1, &mut rng);
        let stats = CombatStats {
            attack: 1.0,
            defense: 0.0,
            speed: 1.0,
            max_hp: 1.0, // very low HP
        };
        let mut encounter = BossEncounterState::new(boss, stats);

        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let result = encounter.process_turn(CombatAction::Attack, &mut rng);
        assert!(
            matches!(result, TurnResult::Defeat { .. }),
            "Should be defeat with 1 HP"
        );
    }

    #[test]
    fn flee_has_roughly_50_percent_success() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let boss = generate_boss(1, &mut rng);
        let stats = test_stats();

        let mut fled = 0;
        let trials = 1000;
        for seed in 0..trials {
            let mut enc = BossEncounterState::new(boss.clone(), stats.clone());
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let result = enc.process_turn(CombatAction::Flee, &mut rng);
            if matches!(result, TurnResult::Fled) {
                fled += 1;
            }
        }

        let rate = fled as f64 / trials as f64;
        assert!(
            (0.35..=0.65).contains(&rate),
            "Flee rate should be ~50%, got {:.2}",
            rate,
        );
    }

    #[test]
    fn log_accumulates_entries() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let boss = generate_boss(1, &mut rng);
        let stats = test_stats();
        let mut encounter = BossEncounterState::new(boss, stats);

        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        encounter.process_turn(CombatAction::Attack, &mut rng);
        encounter.process_turn(CombatAction::Defend, &mut rng);

        assert!(
            encounter.log.len() >= 4,
            "Should have log entries for attack + boss attack + defend + boss attack"
        );
    }
}
