use serde::{Deserialize, Serialize};

/// Rarity tier for items. Affects modifier count and strength.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
}

impl Rarity {
    /// Human-readable label.
    #[allow(unreachable_patterns)]
    pub fn label(self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            _ => "Unknown",
        }
    }
}

/// Equipment slot that an item occupies.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
    Accessory,
}

impl EquipmentSlot {
    /// Human-readable label.
    #[allow(unreachable_patterns)]
    pub fn label(self) -> &'static str {
        match self {
            Self::Weapon => "Weapon",
            Self::Armor => "Armor",
            Self::Accessory => "Accessory",
            _ => "Unknown",
        }
    }
}

/// A stat type that can be modified by equipment.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatType {
    Attack,
    Defense,
    Speed,
    MaxHp,
}

impl StatType {
    /// Human-readable label.
    #[allow(unreachable_patterns)]
    pub fn label(self) -> &'static str {
        match self {
            Self::Attack => "ATK",
            Self::Defense => "DEF",
            Self::Speed => "SPD",
            Self::MaxHp => "HP",
            _ => "???",
        }
    }
}

/// A single stat modification from an item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatModifier {
    pub stat: StatType,
    pub value: f32,
}

/// A generated item with a name, rarity, slot, and stat modifiers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub rarity: Rarity,
    pub slot: EquipmentSlot,
    pub modifiers: Vec<StatModifier>,
}
