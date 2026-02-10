/// Application state
pub struct App {
    /// Whether the application is running
    pub running: bool,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Self {
        Self { running: true }
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
