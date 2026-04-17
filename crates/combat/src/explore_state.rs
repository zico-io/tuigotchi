use serde::{Deserialize, Serialize};

/// Tracks statistics for the current explore session.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExploreState {
    pub battles_won: u32,
    pub battles_lost: u32,
    pub total_xp_earned: u32,
    pub last_battle_log: Option<String>,
}
