use crate::pet::Pet;

/// Application state
pub struct App {
    /// Whether the application is running
    pub running: bool,
    /// The pet
    pub pet: Pet,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Self {
        Self {
            running: true,
            pet: Pet::new("Tomo".to_string()),
        }
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Tick: update pet state
    pub fn tick(&mut self) {
        self.pet.tick();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
