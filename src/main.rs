mod app;
mod event;
mod pet;
mod ui;

use app::App;
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state and event handler
    let mut app = App::new();
    let event_handler = EventHandler::new(Duration::from_millis(250));

    // Main loop
    let result = run_app(&mut terminal, &mut app, &event_handler);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_handler: &EventHandler,
) -> io::Result<()> {
    while app.running {
        // Render UI
        terminal.draw(|frame| ui::render(frame, app))?;

        // Handle events
        match event_handler.next()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => app.quit(),
                KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                    app.quit()
                }
                KeyCode::Char('f') | KeyCode::Char('F') => app.feed(),
                KeyCode::Char('p') | KeyCode::Char('P') => app.play(),
                _ => {}
            },
            Event::Tick => {
                app.tick();
            }
        }
    }
    Ok(())
}
