use crate::app::App;
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
    let mood = app.pet.mood();
    let (mood_label, mood_color) = match mood {
        Mood::Happy => ("Happy 😊", Color::Green),
        Mood::Content => ("Content 😌", Color::Yellow),
        Mood::Sad => ("Sad 😞", Color::LightRed),
        Mood::Miserable => ("Miserable 😭", Color::Red),
    };
    
    let header_text = format!("{} is {}", app.pet.name, mood_label);
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
    let hunger_ratio = app.pet.hunger as f64 / 100.0;
    let hunger_color = stat_color(app.pet.hunger);
    let hunger_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Hunger"))
        .gauge_style(Style::default().fg(hunger_color))
        .ratio(hunger_ratio)
        .label(format!("{}/100", app.pet.hunger));
    frame.render_widget(hunger_gauge, chunks[2]);

    // Happiness gauge
    let happiness_ratio = app.pet.happiness as f64 / 100.0;
    let happiness_color = stat_color(app.pet.happiness);
    let happiness_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Happiness"))
        .gauge_style(Style::default().fg(happiness_color))
        .ratio(happiness_ratio)
        .label(format!("{}/100", app.pet.happiness));
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
