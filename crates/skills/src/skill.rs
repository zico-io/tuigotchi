use serde::{Deserialize, Serialize};

/// Unique identifier for each learnable skill.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillId {
    // Combat
    Sharpen,
    Toughen,
    QuickReflexes,
    // Survival
    IronStomach,
    Resilience,
    Endurance,
    // Fortune
    TreasureHunter,
    QuickLearner,
    Lucky,
}

/// Category grouping for skills in the tree UI.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillCategory {
    Combat,
    Survival,
    Fortune,
}

impl SkillCategory {
    /// Human-readable label.
    #[allow(unreachable_patterns)]
    pub fn label(self) -> &'static str {
        match self {
            Self::Combat => "Combat",
            Self::Survival => "Survival",
            Self::Fortune => "Fortune",
            _ => "???",
        }
    }
}

/// A single effect that a skill grants per rank.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillEffect {
    /// Adds a flat bonus to a combat stat per rank.
    CombatStatBonus { stat: String, value: f32 },
    /// Multiplies a decay rate per rank (e.g., 0.9 = 10% slower).
    DecayRateModifier { stat: String, multiplier: f32 },
    /// Adds to the base loot drop chance per rank.
    LootChanceBonus { bonus: f32 },
    /// Multiplies XP earned per rank (e.g., 1.1 = +10%).
    XpBonus { multiplier: f32 },
}

/// Definition of a skill — its metadata, effects, and prerequisites.
pub struct SkillDef {
    pub id: SkillId,
    pub name: &'static str,
    pub description: &'static str,
    pub category: SkillCategory,
    pub max_rank: u32,
    pub effects_per_rank: Vec<SkillEffect>,
    pub prerequisites: Vec<SkillId>,
}

/// All skill IDs in display order.
pub const ALL_SKILL_IDS: &[SkillId] = &[
    SkillId::Sharpen,
    SkillId::Toughen,
    SkillId::QuickReflexes,
    SkillId::IronStomach,
    SkillId::Resilience,
    SkillId::Endurance,
    SkillId::TreasureHunter,
    SkillId::QuickLearner,
    SkillId::Lucky,
];

/// Return the full list of skill definitions.
pub fn all_skills() -> Vec<SkillDef> {
    vec![
        // -- Combat --
        SkillDef {
            id: SkillId::Sharpen,
            name: "Sharpen",
            description: "Increases attack power",
            category: SkillCategory::Combat,
            max_rank: 5,
            effects_per_rank: vec![SkillEffect::CombatStatBonus {
                stat: "attack".into(),
                value: 3.0,
            }],
            prerequisites: vec![],
        },
        SkillDef {
            id: SkillId::Toughen,
            name: "Toughen",
            description: "Increases defense",
            category: SkillCategory::Combat,
            max_rank: 5,
            effects_per_rank: vec![SkillEffect::CombatStatBonus {
                stat: "defense".into(),
                value: 2.0,
            }],
            prerequisites: vec![],
        },
        SkillDef {
            id: SkillId::QuickReflexes,
            name: "Quick Reflexes",
            description: "Increases speed",
            category: SkillCategory::Combat,
            max_rank: 3,
            effects_per_rank: vec![SkillEffect::CombatStatBonus {
                stat: "speed".into(),
                value: 2.0,
            }],
            prerequisites: vec![SkillId::Sharpen],
        },
        // -- Survival --
        SkillDef {
            id: SkillId::IronStomach,
            name: "Iron Stomach",
            description: "Hunger decays slower",
            category: SkillCategory::Survival,
            max_rank: 5,
            effects_per_rank: vec![SkillEffect::DecayRateModifier {
                stat: "hunger".into(),
                multiplier: 0.9,
            }],
            prerequisites: vec![],
        },
        SkillDef {
            id: SkillId::Resilience,
            name: "Resilience",
            description: "Happiness decays slower",
            category: SkillCategory::Survival,
            max_rank: 3,
            effects_per_rank: vec![SkillEffect::DecayRateModifier {
                stat: "happiness".into(),
                multiplier: 0.9,
            }],
            prerequisites: vec![],
        },
        SkillDef {
            id: SkillId::Endurance,
            name: "Endurance",
            description: "Energy decays slower",
            category: SkillCategory::Survival,
            max_rank: 3,
            effects_per_rank: vec![SkillEffect::DecayRateModifier {
                stat: "energy".into(),
                multiplier: 0.9,
            }],
            prerequisites: vec![SkillId::IronStomach],
        },
        // -- Fortune --
        SkillDef {
            id: SkillId::TreasureHunter,
            name: "Treasure Hunter",
            description: "Better loot drop rate",
            category: SkillCategory::Fortune,
            max_rank: 3,
            effects_per_rank: vec![SkillEffect::LootChanceBonus { bonus: 0.05 }],
            prerequisites: vec![],
        },
        SkillDef {
            id: SkillId::QuickLearner,
            name: "Quick Learner",
            description: "Earn more XP from battles",
            category: SkillCategory::Fortune,
            max_rank: 3,
            effects_per_rank: vec![SkillEffect::XpBonus { multiplier: 1.1 }],
            prerequisites: vec![],
        },
        SkillDef {
            id: SkillId::Lucky,
            name: "Lucky",
            description: "Better everything",
            category: SkillCategory::Fortune,
            max_rank: 3,
            effects_per_rank: vec![
                SkillEffect::CombatStatBonus {
                    stat: "attack".into(),
                    value: 1.0,
                },
                SkillEffect::LootChanceBonus { bonus: 0.02 },
                SkillEffect::XpBonus { multiplier: 1.05 },
            ],
            prerequisites: vec![SkillId::TreasureHunter, SkillId::QuickLearner],
        },
    ]
}

/// Look up a skill definition by its ID.
pub fn skill_def(id: SkillId) -> Option<SkillDef> {
    all_skills().into_iter().find(|s| s.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_skills_returns_nine() {
        assert_eq!(all_skills().len(), 9);
    }

    #[test]
    fn all_skill_ids_match_definitions() {
        let defs = all_skills();
        for id in ALL_SKILL_IDS {
            assert!(
                defs.iter().any(|d| d.id == *id),
                "missing definition for {:?}",
                id,
            );
        }
    }

    #[test]
    fn skill_def_lookup_works() {
        let def = skill_def(SkillId::Sharpen).expect("should find Sharpen");
        assert_eq!(def.name, "Sharpen");
        assert_eq!(def.max_rank, 5);
    }
}
