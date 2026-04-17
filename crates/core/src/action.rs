use crate::pet::Pet;

/// Actions the player can perform on their pet.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Feed,
    Play,
    Clean,
    Sleep,
}

impl Action {
    pub fn label(self) -> &'static str {
        match self {
            Self::Feed => "Feed",
            Self::Play => "Play",
            Self::Clean => "Clean",
            Self::Sleep => "Sleep",
        }
    }
}

/// All actions available in v1.
pub const ALL_ACTIONS: &[Action] = &[Action::Feed, Action::Play, Action::Clean, Action::Sleep];

/// Apply an action to a pet, mutating its stats.
pub fn apply_action(pet: &mut Pet, action: Action) {
    if !pet.alive {
        return;
    }

    match action {
        Action::Feed => {
            pet.stats.hunger -= 25.0;
            pet.stats.energy += 5.0;
        }
        Action::Play => {
            pet.stats.happiness += 20.0;
            pet.stats.hunger += 8.0;
            pet.stats.energy -= 8.0;
        }
        Action::Clean => {
            pet.stats.health += 15.0;
            pet.stats.happiness += 8.0;
        }
        Action::Sleep => {
            pet.stats.energy += 35.0;
            pet.stats.hunger += 3.0;
        }
    }

    pet.stats.clamp();
}
