use crate::pet::Pet;
use std::time::Instant;

/// Application state
pub struct App {
    /// Whether the application is running
    pub running: bool,
    /// The pet
    pub pet: Pet,
    /// Last action feedback message with timestamp
    pub last_action: Option<(String, Instant)>,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Self {
        Self {
            running: true,
            pet: Pet::new("Tomo".to_string()),
            last_action: None,
        }
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Tick: update pet state
    pub fn tick(&mut self) {
        self.pet.tick();
        // Clear action messages after 2 seconds
        if let Some((_, timestamp)) = self.last_action {
            if timestamp.elapsed().as_secs() >= 2 {
                self.last_action = None;
            }
        }
    }

    /// Feed the pet
    pub fn feed(&mut self) {
        self.pet.feed();
        self.last_action = Some((format!("You fed {}!", self.pet.name), Instant::now()));
    }

    /// Play with the pet
    pub fn play(&mut self) {
        self.pet.play();
        self.last_action = Some((format!("You played with {}!", self.pet.name), Instant::now()));
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
