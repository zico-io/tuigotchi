use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use tuigotchi_combat::{
    combat_profile::CombatProfile, explore_state::ExploreState, manual_combat::BossEncounterState,
};
use tuigotchi_items::inventory::Inventory;

use crate::{game_state::GameMode, pet::Pet};

/// Current save format version. Bump when SaveData layout changes.
pub const SAVE_VERSION: u32 = 1;

/// Persisted game state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub pet: Pet,
    /// Unix timestamp (seconds) of last save.
    pub last_saved_at: u64,
    /// Active game mode at time of save (defaults to Camp for old saves).
    #[serde(default)]
    pub game_mode: GameMode,
    /// Combat profile (level, XP). None for pre-combat saves.
    #[serde(default)]
    pub combat_profile: Option<CombatProfile>,
    /// Explore session statistics. None for pre-combat saves.
    #[serde(default)]
    pub explore_state: Option<ExploreState>,
    /// Player inventory. None for pre-items saves.
    #[serde(default)]
    pub inventory: Option<Inventory>,
    /// In-progress boss encounter, if any.
    #[serde(default)]
    pub boss_encounter: Option<BossEncounterState>,
}

impl SaveData {
    pub fn new(
        pet: Pet,
        now: u64,
        game_mode: GameMode,
        combat_profile: Option<CombatProfile>,
        explore_state: Option<ExploreState>,
        inventory: Option<Inventory>,
        boss_encounter: Option<BossEncounterState>,
    ) -> Self {
        Self {
            version: SAVE_VERSION,
            pet,
            last_saved_at: now,
            game_mode,
            combat_profile,
            explore_state,
            inventory,
            boss_encounter,
        }
    }
}

#[derive(Debug)]
pub enum SaveError {
    Io(io::Error),
    Format(serde_json::Error),
    NotFound,
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "save I/O error: {e}"),
            Self::Format(e) => write!(f, "save format error: {e}"),
            Self::NotFound => write!(f, "no save file found"),
        }
    }
}

impl std::error::Error for SaveError {}

impl From<io::Error> for SaveError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for SaveError {
    fn from(e: serde_json::Error) -> Self {
        Self::Format(e)
    }
}

/// Serialize and write save data atomically (write tmp, then rename).
pub fn save(data: &SaveData, path: &Path) -> Result<(), SaveError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(data)?;

    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, &json)?;
    fs::rename(&tmp_path, path)?;

    Ok(())
}

/// Load save data from disk.
pub fn load(path: &Path) -> Result<SaveData, SaveError> {
    if !path.exists() {
        return Err(SaveError::NotFound);
    }

    let json = fs::read_to_string(path)?;
    let data: SaveData = serde_json::from_str(&json)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{game_state::GameMode, pet::Pet};

    #[test]
    fn save_load_round_trip() {
        let dir = std::env::temp_dir().join("tuigotchi_test_save");
        let path = dir.join("test_save.json");

        // Clean up from any previous run
        let _ = fs::remove_file(&path);
        let _ = fs::remove_dir(&dir);

        let pet = Pet::new("TestPet");
        let data = SaveData::new(
            pet.clone(),
            1000,
            GameMode::default(),
            None,
            None,
            None,
            None,
        );

        save(&data, &path).expect("save should succeed");
        let loaded = load(&path).expect("load should succeed");

        assert_eq!(loaded.version, SAVE_VERSION);
        assert_eq!(loaded.pet.name, "TestPet");
        assert_eq!(loaded.last_saved_at, 1000);
        assert!((loaded.pet.stats.hunger - pet.stats.hunger).abs() < f32::EPSILON);

        // Clean up
        let _ = fs::remove_file(&path);
        let _ = fs::remove_dir(&dir);
    }

    #[test]
    fn load_not_found() {
        let path = std::env::temp_dir().join("tuigotchi_nonexistent.json");
        let result = load(&path);
        assert!(matches!(result, Err(SaveError::NotFound)));
    }

    #[test]
    fn boss_encounter_state_serializes_round_trip() {
        use tuigotchi_combat::{
            battle::CombatStats, boss::generate_boss, manual_combat::BossEncounterState,
        };

        let mut rng = rand::thread_rng();
        let boss = generate_boss(3, &mut rng);
        let stats = CombatStats {
            attack: 30.0,
            defense: 15.0,
            speed: 10.0,
            max_hp: 100.0,
        };
        let encounter = BossEncounterState::new(boss, stats);

        let json = serde_json::to_string(&encounter).expect("serialize");
        let loaded: BossEncounterState = serde_json::from_str(&json).expect("deserialize");

        assert!((loaded.pet_hp - encounter.pet_hp).abs() < f32::EPSILON);
        assert!((loaded.boss_hp - encounter.boss_hp).abs() < f32::EPSILON);
        assert_eq!(loaded.boss.enemy.name, encounter.boss.enemy.name);
        assert_eq!(loaded.turn, 0);
    }
}
