use serde::{Deserialize, Serialize};

/// Tracks the pet's combat level and experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatProfile {
    level: u32,
    xp: u32,
    xp_to_next: u32,
}

impl Default for CombatProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl CombatProfile {
    /// Create a new profile at level 1 with no XP.
    pub fn new() -> Self {
        Self {
            level: 1,
            xp: 0,
            xp_to_next: 100,
        }
    }

    /// Add XP. Returns `true` if at least one level-up occurred.
    pub fn add_xp(&mut self, amount: u32) -> bool {
        self.xp += amount;
        let mut leveled = false;

        while self.xp >= self.xp_to_next {
            self.xp -= self.xp_to_next;
            self.level += 1;
            self.xp_to_next = 100 * self.level;
            leveled = true;
        }

        leveled
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn xp(&self) -> u32 {
        self.xp
    }

    pub fn xp_to_next(&self) -> u32 {
        self.xp_to_next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_profile_starts_at_level_1() {
        let p = CombatProfile::new();
        assert_eq!(p.level(), 1);
        assert_eq!(p.xp(), 0);
        assert_eq!(p.xp_to_next(), 100);
    }

    #[test]
    fn add_xp_levels_up_correctly() {
        let mut p = CombatProfile::new();
        let leveled = p.add_xp(100);
        assert!(leveled);
        assert_eq!(p.level(), 2);
        assert_eq!(p.xp(), 0);
        assert_eq!(p.xp_to_next(), 200); // 100 * 2
    }

    #[test]
    fn multi_level_up_works() {
        let mut p = CombatProfile::new();
        // 100 to reach level 2, then 200 to reach level 3 = 300 total
        let leveled = p.add_xp(300);
        assert!(leveled);
        assert_eq!(p.level(), 3);
        assert_eq!(p.xp(), 0);
        assert_eq!(p.xp_to_next(), 300); // 100 * 3
    }

    #[test]
    fn xp_curve_increases_per_level() {
        let mut p = CombatProfile::new();
        p.add_xp(100); // level 2, needs 200
        assert_eq!(p.xp_to_next(), 200);
        p.add_xp(200); // level 3, needs 300
        assert_eq!(p.xp_to_next(), 300);
        p.add_xp(300); // level 4, needs 400
        assert_eq!(p.xp_to_next(), 400);
    }

    #[test]
    fn partial_xp_does_not_level() {
        let mut p = CombatProfile::new();
        let leveled = p.add_xp(50);
        assert!(!leveled);
        assert_eq!(p.level(), 1);
        assert_eq!(p.xp(), 50);
    }
}
