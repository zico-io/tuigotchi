use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::skill::{self, SkillEffect, SkillId};

/// Error when allocating a skill point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillError {
    NotEnoughPoints,
    MaxRankReached,
    PrerequisiteNotMet,
    UnknownSkill,
}

impl std::fmt::Display for SkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughPoints => write!(f, "not enough skill points"),
            Self::MaxRankReached => write!(f, "skill is already at max rank"),
            Self::PrerequisiteNotMet => write!(f, "prerequisite skill not learned"),
            Self::UnknownSkill => write!(f, "unknown skill"),
        }
    }
}

/// Aggregated effects from all learned skills, ready to apply to game systems.
#[derive(Debug, Clone)]
pub struct AggregatedEffects {
    pub attack_bonus: f32,
    pub defense_bonus: f32,
    pub speed_bonus: f32,
    pub max_hp_bonus: f32,
    pub hunger_rate_multiplier: f32,
    pub happiness_rate_multiplier: f32,
    pub energy_rate_multiplier: f32,
    pub loot_chance_bonus: f32,
    pub xp_multiplier: f32,
}

impl Default for AggregatedEffects {
    fn default() -> Self {
        Self {
            attack_bonus: 0.0,
            defense_bonus: 0.0,
            speed_bonus: 0.0,
            max_hp_bonus: 0.0,
            hunger_rate_multiplier: 1.0,
            happiness_rate_multiplier: 1.0,
            energy_rate_multiplier: 1.0,
            loot_chance_bonus: 0.0,
            xp_multiplier: 1.0,
        }
    }
}

/// Persistent skill tree tracking allocated ranks and available points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTree {
    ranks: HashMap<SkillId, u32>,
    available_points: u32,
}

impl Default for SkillTree {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillTree {
    /// Create an empty skill tree with no points.
    pub fn new() -> Self {
        Self {
            ranks: HashMap::new(),
            available_points: 0,
        }
    }

    /// Grant one skill point.
    pub fn add_point(&mut self) {
        self.available_points += 1;
    }

    /// Current rank for a skill (0 if not learned).
    pub fn rank(&self, id: SkillId) -> u32 {
        self.ranks.get(&id).copied().unwrap_or(0)
    }

    /// Available unspent skill points.
    pub fn available_points(&self) -> u32 {
        self.available_points
    }

    /// Attempt to allocate one point into the given skill.
    pub fn allocate(&mut self, id: SkillId) -> Result<(), SkillError> {
        let def = skill::skill_def(id).ok_or(SkillError::UnknownSkill)?;

        if self.available_points == 0 {
            return Err(SkillError::NotEnoughPoints);
        }

        let current = self.rank(id);
        if current >= def.max_rank {
            return Err(SkillError::MaxRankReached);
        }

        // Check prerequisites
        for prereq in &def.prerequisites {
            if self.rank(*prereq) == 0 {
                return Err(SkillError::PrerequisiteNotMet);
            }
        }

        self.available_points -= 1;
        *self.ranks.entry(id).or_insert(0) += 1;
        Ok(())
    }

    /// Aggregate all learned skills' effects across all ranks.
    pub fn total_effects(&self) -> AggregatedEffects {
        let mut effects = AggregatedEffects::default();

        for (&id, &rank) in &self.ranks {
            if rank == 0 {
                continue;
            }
            if let Some(def) = skill::skill_def(id) {
                for _ in 0..rank {
                    for effect in &def.effects_per_rank {
                        #[allow(unreachable_patterns)]
                        match effect {
                            SkillEffect::CombatStatBonus { stat, value } => match stat.as_str() {
                                "attack" => effects.attack_bonus += value,
                                "defense" => effects.defense_bonus += value,
                                "speed" => effects.speed_bonus += value,
                                "max_hp" => effects.max_hp_bonus += value,
                                _ => {}
                            },
                            SkillEffect::DecayRateModifier { stat, multiplier } => {
                                match stat.as_str() {
                                    "hunger" => {
                                        effects.hunger_rate_multiplier *= multiplier;
                                    }
                                    "happiness" => {
                                        effects.happiness_rate_multiplier *= multiplier;
                                    }
                                    "energy" => {
                                        effects.energy_rate_multiplier *= multiplier;
                                    }
                                    _ => {}
                                }
                            }
                            SkillEffect::LootChanceBonus { bonus } => {
                                effects.loot_chance_bonus += bonus;
                            }
                            SkillEffect::XpBonus { multiplier } => {
                                effects.xp_multiplier *= multiplier;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        effects
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tree_has_no_points() {
        let tree = SkillTree::new();
        assert_eq!(tree.available_points(), 0);
        assert_eq!(tree.rank(SkillId::Sharpen), 0);
    }

    #[test]
    fn add_point_increments() {
        let mut tree = SkillTree::new();
        tree.add_point();
        tree.add_point();
        assert_eq!(tree.available_points(), 2);
    }

    #[test]
    fn allocate_with_points_succeeds() {
        let mut tree = SkillTree::new();
        tree.add_point();
        assert!(tree.allocate(SkillId::Sharpen).is_ok());
        assert_eq!(tree.rank(SkillId::Sharpen), 1);
        assert_eq!(tree.available_points(), 0);
    }

    #[test]
    fn allocate_without_points_fails() {
        let mut tree = SkillTree::new();
        assert_eq!(
            tree.allocate(SkillId::Sharpen),
            Err(SkillError::NotEnoughPoints),
        );
    }

    #[test]
    fn allocate_at_max_rank_fails() {
        let mut tree = SkillTree::new();
        // Sharpen has max_rank = 5
        for _ in 0..6 {
            tree.add_point();
        }
        for _ in 0..5 {
            tree.allocate(SkillId::Sharpen).unwrap();
        }
        assert_eq!(
            tree.allocate(SkillId::Sharpen),
            Err(SkillError::MaxRankReached),
        );
    }

    #[test]
    fn allocate_prerequisite_not_met() {
        let mut tree = SkillTree::new();
        tree.add_point();
        // QuickReflexes requires Sharpen
        assert_eq!(
            tree.allocate(SkillId::QuickReflexes),
            Err(SkillError::PrerequisiteNotMet),
        );
    }

    #[test]
    fn allocate_with_prerequisite_met() {
        let mut tree = SkillTree::new();
        tree.add_point();
        tree.add_point();
        tree.allocate(SkillId::Sharpen).unwrap();
        assert!(tree.allocate(SkillId::QuickReflexes).is_ok());
    }

    #[test]
    fn lucky_requires_both_prerequisites() {
        let mut tree = SkillTree::new();
        for _ in 0..5 {
            tree.add_point();
        }
        // Only TreasureHunter — should fail
        tree.allocate(SkillId::TreasureHunter).unwrap();
        assert_eq!(
            tree.allocate(SkillId::Lucky),
            Err(SkillError::PrerequisiteNotMet),
        );
        // Now add QuickLearner too
        tree.allocate(SkillId::QuickLearner).unwrap();
        assert!(tree.allocate(SkillId::Lucky).is_ok());
    }

    #[test]
    fn total_effects_sharpen_rank_3() {
        let mut tree = SkillTree::new();
        for _ in 0..3 {
            tree.add_point();
            tree.allocate(SkillId::Sharpen).unwrap();
        }
        let effects = tree.total_effects();
        assert!((effects.attack_bonus - 9.0).abs() < f32::EPSILON);
        assert!((effects.defense_bonus - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn total_effects_iron_stomach_rank_3() {
        let mut tree = SkillTree::new();
        for _ in 0..3 {
            tree.add_point();
            tree.allocate(SkillId::IronStomach).unwrap();
        }
        let effects = tree.total_effects();
        // 0.9^3 = 0.729
        assert!((effects.hunger_rate_multiplier - 0.729).abs() < 0.001);
        // Others untouched
        assert!((effects.happiness_rate_multiplier - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn total_effects_defaults_are_neutral() {
        let tree = SkillTree::new();
        let effects = tree.total_effects();
        assert!((effects.attack_bonus - 0.0).abs() < f32::EPSILON);
        assert!((effects.hunger_rate_multiplier - 1.0).abs() < f32::EPSILON);
        assert!((effects.xp_multiplier - 1.0).abs() < f32::EPSILON);
        assert!((effects.loot_chance_bonus - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn serialize_deserialize_round_trip() {
        let mut tree = SkillTree::new();
        tree.add_point();
        tree.add_point();
        tree.allocate(SkillId::Sharpen).unwrap();

        let json = serde_json::to_string(&tree).expect("serialize");
        let loaded: SkillTree = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(loaded.rank(SkillId::Sharpen), 1);
        assert_eq!(loaded.available_points(), 1);
    }

    #[test]
    fn total_effects_lucky_combines_all() {
        let mut tree = SkillTree::new();
        for _ in 0..3 {
            tree.add_point();
        }
        tree.allocate(SkillId::TreasureHunter).unwrap();
        tree.allocate(SkillId::QuickLearner).unwrap();
        tree.allocate(SkillId::Lucky).unwrap();

        let effects = tree.total_effects();
        // TreasureHunter r1: +0.05 loot, QuickLearner r1: 1.1x XP
        // Lucky r1: +1.0 ATK, +0.02 loot, 1.05x XP
        assert!((effects.attack_bonus - 1.0).abs() < f32::EPSILON);
        assert!((effects.loot_chance_bonus - 0.07).abs() < 0.001);
        // XP: 1.1 * 1.05 = 1.155
        assert!((effects.xp_multiplier - 1.155).abs() < 0.001);
    }
}
