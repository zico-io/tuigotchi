use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::item::{EquipmentSlot, Item, StatModifier};

/// Error returned when an inventory operation fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EquipError {
    /// The index is out of bounds.
    IndexOutOfBounds,
    /// The inventory is full.
    InventoryFull,
}

impl std::fmt::Display for EquipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IndexOutOfBounds => write!(f, "item index out of bounds"),
            Self::InventoryFull => write!(f, "inventory is full"),
        }
    }
}

impl std::error::Error for EquipError {}

/// Player inventory: a bounded list of items with equipment slots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    items: Vec<Item>,
    equipped: HashMap<EquipmentSlot, usize>,
    capacity: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Inventory {
    /// Create an empty inventory with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            equipped: HashMap::new(),
            capacity,
        }
    }

    /// Add an item to the inventory. Returns `Err` if at capacity.
    pub fn add_item(&mut self, item: Item) -> Result<(), EquipError> {
        if self.is_full() {
            return Err(EquipError::InventoryFull);
        }
        self.items.push(item);
        Ok(())
    }

    /// Remove the item at `index`, returning it. Updates equipped map.
    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index >= self.items.len() {
            return None;
        }

        // Check if the removed item was equipped and unequip it
        let mut slot_to_remove = None;
        for (&slot, &eq_idx) in &self.equipped {
            if eq_idx == index {
                slot_to_remove = Some(slot);
            }
        }
        if let Some(slot) = slot_to_remove {
            self.equipped.remove(&slot);
        }

        let item = self.items.remove(index);

        // Shift equipped indices that are above the removed index
        let mut updated = HashMap::new();
        for (&slot, &eq_idx) in &self.equipped {
            if eq_idx > index {
                updated.insert(slot, eq_idx - 1);
            } else {
                updated.insert(slot, eq_idx);
            }
        }
        self.equipped = updated;

        Some(item)
    }

    /// Equip the item at `index`. Automatically unequips any previous item in the same slot.
    pub fn equip(&mut self, index: usize) -> Result<(), EquipError> {
        if index >= self.items.len() {
            return Err(EquipError::IndexOutOfBounds);
        }
        let slot = self.items[index].slot;
        self.equipped.insert(slot, index);
        Ok(())
    }

    /// Unequip the item in the given slot.
    pub fn unequip(&mut self, slot: EquipmentSlot) {
        self.equipped.remove(&slot);
    }

    /// Get the equipped item for a slot, if any.
    pub fn equipped_item(&self, slot: EquipmentSlot) -> Option<&Item> {
        self.equipped.get(&slot).and_then(|&i| self.items.get(i))
    }

    /// View all items.
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    /// Whether the inventory is at capacity.
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }

    /// Current item count.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the inventory is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Capacity of the inventory.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Check if the item at the given index is equipped.
    pub fn is_equipped(&self, index: usize) -> bool {
        self.equipped.values().any(|&i| i == index)
    }

    /// Aggregate stat modifiers from all equipped items.
    pub fn total_modifiers(&self) -> Vec<StatModifier> {
        let mut mods = Vec::new();
        for &idx in self.equipped.values() {
            if let Some(item) = self.items.get(idx) {
                mods.extend(item.modifiers.iter().cloned());
            }
        }
        mods
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::{EquipmentSlot, Item, Rarity, StatModifier, StatType};

    fn test_item(name: &str, slot: EquipmentSlot) -> Item {
        Item {
            name: name.to_string(),
            rarity: Rarity::Common,
            slot,
            modifiers: vec![StatModifier {
                stat: StatType::Attack,
                value: 5.0,
            }],
        }
    }

    #[test]
    fn add_and_view_items() {
        let mut inv = Inventory::new(5);
        inv.add_item(test_item("Sword", EquipmentSlot::Weapon))
            .unwrap();
        assert_eq!(inv.len(), 1);
        assert_eq!(inv.items()[0].name, "Sword");
    }

    #[test]
    fn capacity_enforcement() {
        let mut inv = Inventory::new(1);
        inv.add_item(test_item("Sword", EquipmentSlot::Weapon))
            .unwrap();
        let result = inv.add_item(test_item("Axe", EquipmentSlot::Weapon));
        assert_eq!(result, Err(EquipError::InventoryFull));
        assert!(inv.is_full());
    }

    #[test]
    fn remove_item_shifts_equipped() {
        let mut inv = Inventory::new(10);
        inv.add_item(test_item("Sword", EquipmentSlot::Weapon))
            .unwrap();
        inv.add_item(test_item("Vest", EquipmentSlot::Armor))
            .unwrap();
        inv.equip(1).unwrap(); // equip vest at index 1

        let removed = inv.remove_item(0).unwrap(); // remove sword at index 0
        assert_eq!(removed.name, "Sword");
        // Vest should now be at index 0 and still equipped
        assert_eq!(
            inv.equipped_item(EquipmentSlot::Armor).unwrap().name,
            "Vest"
        );
    }

    #[test]
    fn equip_and_unequip() {
        let mut inv = Inventory::new(10);
        inv.add_item(test_item("Sword", EquipmentSlot::Weapon))
            .unwrap();
        inv.equip(0).unwrap();
        assert!(inv.is_equipped(0));
        assert_eq!(
            inv.equipped_item(EquipmentSlot::Weapon).unwrap().name,
            "Sword"
        );

        inv.unequip(EquipmentSlot::Weapon);
        assert!(!inv.is_equipped(0));
        assert!(inv.equipped_item(EquipmentSlot::Weapon).is_none());
    }

    #[test]
    fn equip_replaces_previous() {
        let mut inv = Inventory::new(10);
        inv.add_item(test_item("Sword", EquipmentSlot::Weapon))
            .unwrap();
        inv.add_item(test_item("Axe", EquipmentSlot::Weapon))
            .unwrap();
        inv.equip(0).unwrap();
        inv.equip(1).unwrap();
        assert_eq!(
            inv.equipped_item(EquipmentSlot::Weapon).unwrap().name,
            "Axe"
        );
        assert!(!inv.is_equipped(0));
        assert!(inv.is_equipped(1));
    }

    #[test]
    fn equip_out_of_bounds() {
        let inv = Inventory::new(10);
        assert_eq!(
            Inventory::equip(&mut inv.clone(), 5),
            Err(EquipError::IndexOutOfBounds)
        );
    }

    #[test]
    fn total_modifiers_aggregation() {
        let mut inv = Inventory::new(10);
        inv.add_item(Item {
            name: "Sword".into(),
            rarity: Rarity::Common,
            slot: EquipmentSlot::Weapon,
            modifiers: vec![StatModifier {
                stat: StatType::Attack,
                value: 5.0,
            }],
        })
        .unwrap();
        inv.add_item(Item {
            name: "Vest".into(),
            rarity: Rarity::Common,
            slot: EquipmentSlot::Armor,
            modifiers: vec![StatModifier {
                stat: StatType::Defense,
                value: 3.0,
            }],
        })
        .unwrap();

        // Nothing equipped yet
        assert!(inv.total_modifiers().is_empty());

        inv.equip(0).unwrap();
        inv.equip(1).unwrap();

        let mods = inv.total_modifiers();
        assert_eq!(mods.len(), 2);
    }

    #[test]
    fn serialize_deserialize_round_trip() {
        let mut inv = Inventory::new(20);
        inv.add_item(test_item("Sword", EquipmentSlot::Weapon))
            .unwrap();
        inv.equip(0).unwrap();

        let json = serde_json::to_string(&inv).unwrap();
        let loaded: Inventory = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded.items()[0].name, "Sword");
        assert!(loaded.is_equipped(0));
    }
}
