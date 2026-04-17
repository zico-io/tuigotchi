use std::path::PathBuf;

use tuigotchi_core::{
    action::{self, Action, ALL_ACTIONS},
    event::{EventBus, GameEvent},
    game_state::GameMode,
    offline,
    pet::Pet,
    save::{self, SaveData},
    tick,
};

pub struct App {
    pub pet: Pet,
    pub events: EventBus,
    pub selected_action: usize,
    pub status_message: Option<String>,
    pub running: bool,
    pub save_path: PathBuf,
    pub game_mode: GameMode,
}

impl App {
    pub fn new(pet_name: impl Into<String>, save_path: PathBuf) -> Self {
        Self {
            pet: Pet::new(pet_name),
            events: EventBus::new(),
            selected_action: 0,
            status_message: None,
            running: true,
            save_path,
            game_mode: GameMode::default(),
        }
    }

    /// Restore from save data, simulating offline time. Returns the app with a welcome-back message.
    pub fn from_save(data: SaveData, save_path: PathBuf) -> Self {
        let mut pet = data.pet;
        let mut events = EventBus::new();
        let game_mode = data.game_mode;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let elapsed = now.saturating_sub(data.last_saved_at);
        let summary = offline::simulate_offline(&mut pet, elapsed, &mut events);
        let status_message = if elapsed > 60 {
            Some(summary.message())
        } else {
            None
        };

        Self {
            pet,
            events,
            selected_action: 0,
            status_message,
            running: true,
            save_path,
            game_mode,
        }
    }

    /// Save current state to disk.
    pub fn save(&self) -> Result<(), save::SaveError> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let data = SaveData::new(self.pet.clone(), now, self.game_mode);
        save::save(&data, &self.save_path)
    }

    /// Whether the pet is healthy enough to explore.
    pub fn can_explore(&self) -> bool {
        !self.pet.needs_care
    }

    /// Toggle between Camp and Explore modes.
    pub fn toggle_mode(&mut self) {
        match self.game_mode {
            GameMode::Camp => {
                if self.can_explore() {
                    self.game_mode = GameMode::Explore;
                    self.status_message = Some("Heading out to explore!".into());
                } else {
                    self.status_message = Some("Your pet needs care before exploring!".into());
                }
            }
            GameMode::Explore => {
                self.game_mode = GameMode::Camp;
                self.status_message = Some("Returned to camp.".into());
            }
            _ => {}
        }
    }

    pub fn current_action(&self) -> Action {
        ALL_ACTIONS[self.selected_action]
    }

    pub fn next_action(&mut self) {
        self.selected_action = (self.selected_action + 1) % ALL_ACTIONS.len();
    }

    pub fn prev_action(&mut self) {
        self.selected_action = if self.selected_action == 0 {
            ALL_ACTIONS.len() - 1
        } else {
            self.selected_action - 1
        };
    }

    pub fn perform_action(&mut self) {
        let action = self.current_action();
        action::apply_action(&mut self.pet, action);
        self.status_message = Some(format!("You {}!", action_past_tense(action)));
    }

    pub fn tick(&mut self, elapsed_secs: u64) {
        tick::tick(
            &mut self.pet,
            elapsed_secs,
            &mut self.events,
            self.game_mode,
        );
        self.process_events();
    }

    fn process_events(&mut self) {
        for event in self.events.drain() {
            match event {
                GameEvent::StatWarning(stat) => {
                    self.status_message = Some(format!("Warning: {} is critical!", stat));
                }
                GameEvent::Evolved { from: _, to } => {
                    self.status_message = Some(format!("{} evolved to {:?}!", self.pet.name, to));
                }
                GameEvent::Died => {
                    self.status_message = Some(format!("{} has died...", self.pet.name));
                }
                GameEvent::NeedsCare => {
                    self.status_message =
                        Some(format!("{} needs care! Stats are critical.", self.pet.name));
                }
                GameEvent::Recovered => {
                    self.status_message = Some(format!("{} is feeling better!", self.pet.name));
                }
                GameEvent::ForcedCamp => {
                    self.game_mode = GameMode::Camp;
                    self.status_message =
                        Some(format!("{} was forced back to camp!", self.pet.name));
                }
                GameEvent::EnteredExplore | GameEvent::EnteredCamp => {}
                _ => {}
            }
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

fn action_past_tense(action: Action) -> &'static str {
    match action {
        Action::Feed => "fed your pet",
        Action::Play => "played with your pet",
        Action::Clean => "cleaned your pet",
        Action::Sleep => "put your pet to sleep",
        _ => "did something",
    }
}
