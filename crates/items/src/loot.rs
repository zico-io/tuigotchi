use rand::Rng;

use crate::item::{EquipmentSlot, Item, Rarity, StatModifier, StatType};

const WEAPON_PREFIXES: &[&str] = &["Iron", "Steel", "Shadow", "Crystal", "Flame"];
const WEAPON_BASES: &[&str] = &["Sword", "Axe", "Dagger", "Staff", "Bow"];

const ARMOR_PREFIXES: &[&str] = &["Leather", "Chain", "Plate", "Mystic", "Dragon"];
const ARMOR_BASES: &[&str] = &["Vest", "Mail", "Shield", "Helm", "Boots"];

const ACCESSORY_PREFIXES: &[&str] = &["Silver", "Gold", "Enchanted", "Ancient", "Lucky"];
const ACCESSORY_BASES: &[&str] = &["Ring", "Amulet", "Charm", "Pendant", "Band"];

/// Attempt to generate a loot drop from a defeated enemy.
///
/// Returns `None` ~70% of the time (30% drop rate).
pub fn generate_loot(enemy_level: u32, rng: &mut impl Rng) -> Option<Item> {
    // 30% drop rate
    if !rng.gen_bool(0.30) {
        return None;
    }

    let rarity = roll_rarity(rng);
    let slot = roll_slot(rng);
    let name = generate_name(slot, rng);
    let modifiers = generate_modifiers(slot, rarity, enemy_level, rng);

    Some(Item {
        name,
        rarity,
        slot,
        modifiers,
    })
}

fn roll_rarity(rng: &mut impl Rng) -> Rarity {
    let roll: f32 = rng.gen();
    if roll < 0.70 {
        Rarity::Common
    } else if roll < 0.95 {
        Rarity::Uncommon
    } else {
        Rarity::Rare
    }
}

fn roll_slot(rng: &mut impl Rng) -> EquipmentSlot {
    match rng.gen_range(0..3) {
        0 => EquipmentSlot::Weapon,
        1 => EquipmentSlot::Armor,
        _ => EquipmentSlot::Accessory,
    }
}

fn generate_name(slot: EquipmentSlot, rng: &mut impl Rng) -> String {
    #[allow(unreachable_patterns)]
    let (prefixes, bases) = match slot {
        EquipmentSlot::Weapon => (WEAPON_PREFIXES, WEAPON_BASES),
        EquipmentSlot::Armor => (ARMOR_PREFIXES, ARMOR_BASES),
        EquipmentSlot::Accessory => (ACCESSORY_PREFIXES, ACCESSORY_BASES),
        _ => (WEAPON_PREFIXES, WEAPON_BASES),
    };

    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let base = bases[rng.gen_range(0..bases.len())];
    format!("{prefix} {base}")
}

fn generate_modifiers(
    slot: EquipmentSlot,
    rarity: Rarity,
    enemy_level: u32,
    rng: &mut impl Rng,
) -> Vec<StatModifier> {
    let base_value = 2.0 + enemy_level as f32 * 0.5;

    #[allow(unreachable_patterns)]
    let (mod_count, multiplier) = match rarity {
        Rarity::Common => (1, 1.0_f32),
        Rarity::Uncommon => (rng.gen_range(1..=2), 1.5_f32),
        Rarity::Rare => (rng.gen_range(2..=3), 2.5_f32),
        _ => (1, 1.0),
    };

    #[allow(unreachable_patterns)]
    let stat_pool = match slot {
        EquipmentSlot::Weapon => vec![StatType::Attack, StatType::Speed],
        EquipmentSlot::Armor => vec![StatType::Defense, StatType::MaxHp],
        EquipmentSlot::Accessory => {
            vec![
                StatType::Attack,
                StatType::Defense,
                StatType::Speed,
                StatType::MaxHp,
            ]
        }
        _ => vec![StatType::Attack],
    };

    let mut modifiers = Vec::new();
    for _ in 0..mod_count {
        let stat = stat_pool[rng.gen_range(0..stat_pool.len())];
        // Add some variance: +/- 20%
        let variance = rng.gen_range(0.8..1.2);
        let value = (base_value * multiplier * variance * 10.0).round() / 10.0;
        modifiers.push(StatModifier { stat, value });
    }

    modifiers
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn generate_loot_produces_valid_items() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut found = false;

        for _ in 0..100 {
            if let Some(item) = generate_loot(5, &mut rng) {
                assert!(!item.name.is_empty());
                assert!(!item.modifiers.is_empty());
                for m in &item.modifiers {
                    assert!(m.value > 0.0);
                }
                found = true;
            }
        }

        assert!(
            found,
            "should have generated at least one item in 100 tries"
        );
    }

    #[test]
    fn rarity_distribution_roughly_correct() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(123);
        let mut common = 0u32;
        let mut uncommon = 0u32;
        let mut rare = 0u32;

        for _ in 0..1000 {
            if let Some(item) = generate_loot(1, &mut rng) {
                #[allow(unreachable_patterns)]
                match item.rarity {
                    Rarity::Common => common += 1,
                    Rarity::Uncommon => uncommon += 1,
                    Rarity::Rare => rare += 1,
                    _ => {}
                }
            }
        }

        let total = common + uncommon + rare;
        assert!(total > 200, "should drop items ~30% of 1000 tries");

        // Check rough ratios (with generous tolerance for randomness)
        let common_pct = common as f64 / total as f64;
        let uncommon_pct = uncommon as f64 / total as f64;
        let rare_pct = rare as f64 / total as f64;

        assert!(
            common_pct > 0.50,
            "common should be > 50%, was {common_pct:.2}"
        );
        assert!(
            uncommon_pct > 0.10,
            "uncommon should be > 10%, was {uncommon_pct:.2}"
        );
        assert!(rare_pct < 0.15, "rare should be < 15%, was {rare_pct:.2}");
    }

    #[test]
    fn weapon_items_get_attack_or_speed_mods() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(77);
        for _ in 0..100 {
            if let Some(item) = generate_loot(3, &mut rng) {
                if item.slot == EquipmentSlot::Weapon {
                    for m in &item.modifiers {
                        assert!(
                            matches!(m.stat, StatType::Attack | StatType::Speed),
                            "weapon mod should be Attack or Speed, got {:?}",
                            m.stat
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn rare_items_have_more_modifiers() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(999);
        let mut rare_mod_counts = Vec::new();

        for _ in 0..10000 {
            if let Some(item) = generate_loot(5, &mut rng) {
                if item.rarity == Rarity::Rare {
                    rare_mod_counts.push(item.modifiers.len());
                }
            }
        }

        assert!(!rare_mod_counts.is_empty());
        let avg: f64 = rare_mod_counts.iter().sum::<usize>() as f64 / rare_mod_counts.len() as f64;
        assert!(
            avg >= 2.0,
            "rare items should average 2+ mods, got {avg:.2}"
        );
    }
}
