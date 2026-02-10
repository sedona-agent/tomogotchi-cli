use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;

/// Events that can occur in the application
pub enum Event {
    /// A keyboard key was pressed
    Key(KeyEvent),
    /// A tick occurred (for periodic updates)
    Tick,
}

/// Event handler that manages input and tick events
pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// Get the next event (blocking with timeout)
    pub fn next(&self) -> std::io::Result<Event> {
        if event::poll(self.tick_rate)? {
            if let CrosstermEvent::Key(key) = event::read()? {
                return Ok(Event::Key(key));
            }
        }
        Ok(Event::Tick)
    }
}
