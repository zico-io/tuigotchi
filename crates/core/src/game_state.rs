use serde::{Deserialize, Serialize};

/// The current game mode.
///
/// V2 starts with Camp (pet care) and Explore (auto-battling placeholder).
/// Future phases may add more modes (e.g., Town, Dungeon).
#[non_exhaustive]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    /// Home base — feed, play, clean, sleep.
    #[default]
    Camp,
    /// Out adventuring — combat ticks will land in Phase 3.
    Explore,
}
