use crate::app::{App, AppState};
use crate::pet::Mood;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

/// ASCII art for each mood state
const HAPPY_PET: &str = r#"
   /\_/\  
  ( ^.^ ) 
   > ^ <
"#;

const CONTENT_PET: &str = r#"
   /\_/\  
  ( -.- ) 
   > - <
"#;

const SAD_PET: &str = r#"
   /\_/\  
  ( ;_; ) 
   > ~ <
"#;

const MISERABLE_PET: &str = r#"
   /\_/\  
  ( x_x ) 
   > _ <
"#;

/// Render the application UI
pub fn render(frame: &mut Frame, app: &App) {
    match app.state {
        AppState::Naming => render_naming_screen(frame, app),
        AppState::Running => render_running_screen(frame, app),
    }
}

/// Render the naming screen
fn render_naming_screen(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(3),
            Constraint::Percentage(40),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Title
    let title = Paragraph::new("Welcome to Tomogotchi!")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);
    frame.render_widget(title, chunks[0]);

    // Input field
    let input_text = if app.name_input.is_empty() {
        "Tomo".to_string() // Show default name
    } else {
        app.name_input.clone()
    };
    
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Name your pet:"));
    frame.render_widget(input, chunks[1]);

    // Instructions
    let instructions = Paragraph::new("Type a name and press Enter\n(Leave empty for 'Tomo' | Esc to quit)")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    frame.render_widget(instructions, chunks[3]);
}

/// Render the main running screen
fn render_running_screen(frame: &mut Frame, app: &App) {
    let Some(pet) = &app.pet else {
        return; // Should never happen, but safety first
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header (name + mood)
            Constraint::Min(5),     // Main area (pet ASCII)
            Constraint::Length(3),  // Hunger gauge
            Constraint::Length(3),  // Happiness gauge
            Constraint::Length(3),  // Action feedback
            Constraint::Length(3),  // Footer (controls)
        ])
        .split(frame.area());

    // Header: pet name and mood
    let mood = pet.mood();
    let (mood_label, mood_color) = match mood {
        Mood::Happy => ("Happy 😊", Color::Green),
        Mood::Content => ("Content 😌", Color::Yellow),
        Mood::Sad => ("Sad 😞", Color::LightRed),
        Mood::Miserable => ("Miserable 😭", Color::Red),
    };
    
    let header_text = format!("{} is {}", pet.name, mood_label);
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(mood_color))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    // Main area: ASCII art pet
    let pet_art = match mood {
        Mood::Happy => HAPPY_PET,
        Mood::Content => CONTENT_PET,
        Mood::Sad => SAD_PET,
        Mood::Miserable => MISERABLE_PET,
    };
    
    let pet_widget = Paragraph::new(pet_art)
        .style(Style::default().fg(mood_color))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(pet_widget, chunks[1]);

    // Hunger gauge
    let hunger_ratio = pet.hunger as f64 / 100.0;
    let hunger_color = stat_color(pet.hunger);
    let hunger_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Hunger"))
        .gauge_style(Style::default().fg(hunger_color))
        .ratio(hunger_ratio)
        .label(format!("{}/100", pet.hunger));
    frame.render_widget(hunger_gauge, chunks[2]);

    // Happiness gauge
    let happiness_ratio = pet.happiness as f64 / 100.0;
    let happiness_color = stat_color(pet.happiness);
    let happiness_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Happiness"))
        .gauge_style(Style::default().fg(happiness_color))
        .ratio(happiness_ratio)
        .label(format!("{}/100", pet.happiness));
    frame.render_widget(happiness_gauge, chunks[3]);

    // Action feedback area
    let feedback_text = if let Some((message, _)) = &app.last_action {
        message.clone()
    } else {
        String::new()
    };
    let feedback = Paragraph::new(feedback_text)
        .style(Style::default().fg(Color::Green))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(feedback, chunks[4]);

    // Footer with controls
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("f", Style::default().fg(Color::Yellow)),
        Span::raw(": Feed | "),
        Span::styled("p", Style::default().fg(Color::Yellow)),
        Span::raw(": Play | "),
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::raw(": Quit"),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[5]);
}

/// Get color based on stat value
fn stat_color(value: u8) -> Color {
    match value {
        70..=100 => Color::Green,
        40..=69 => Color::Yellow,
        _ => Color::Red,
    }
}
