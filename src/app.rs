use crate::pet::Pet;
use std::time::Instant;

/// Application lifecycle state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    /// Player is naming their pet
    Naming,
    /// Normal running state with pet
    Running,
}

/// Application state
pub struct App {
    /// Whether the application is running
    pub running: bool,
    /// Current app state (Naming or Running)
    pub state: AppState,
    /// The pet (created after naming)
    pub pet: Option<Pet>,
    /// Name input buffer (used during naming)
    pub name_input: String,
    /// Last action feedback message with timestamp
    pub last_action: Option<(String, Instant)>,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Self {
        Self {
            running: true,
            state: AppState::Naming,
            pet: None,
            name_input: String::new(),
            last_action: None,
        }
    }

    /// Add a character to the name input
    pub fn input_char(&mut self, c: char) {
        if self.state == AppState::Naming && self.name_input.len() < 20 {
            self.name_input.push(c);
        }
    }

    /// Remove last character from name input
    pub fn input_backspace(&mut self) {
        if self.state == AppState::Naming {
            self.name_input.pop();
        }
    }

    /// Confirm the pet name and transition to running state
    pub fn confirm_name(&mut self) {
        if self.state == AppState::Naming {
            let name = if self.name_input.is_empty() {
                "Tomo".to_string()
            } else {
                self.name_input.clone()
            };
            self.pet = Some(Pet::new(name));
            self.state = AppState::Running;
        }
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Tick: update pet state
    pub fn tick(&mut self) {
        if let Some(pet) = &mut self.pet {
            pet.tick();
        }
        // Clear action messages after 2 seconds
        if let Some((_, timestamp)) = self.last_action {
            if timestamp.elapsed().as_secs() >= 2 {
                self.last_action = None;
            }
        }
    }

    /// Feed the pet
    pub fn feed(&mut self) {
        if let Some(pet) = &mut self.pet {
            pet.feed();
            self.last_action = Some((format!("You fed {}!", pet.name), Instant::now()));
        }
    }

    /// Play with the pet
    pub fn play(&mut self) {
        if let Some(pet) = &mut self.pet {
            pet.play();
            self.last_action = Some((format!("You played with {}!", pet.name), Instant::now()));
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
