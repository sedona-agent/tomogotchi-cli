use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the application UI
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Header
    let header = Paragraph::new("Tomogotchi CLI")
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    // Main area (placeholder for now)
    let main = Paragraph::new("Your pet will appear here!")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(main, chunks[1]);

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
    frame.render_widget(feedback, chunks[2]);

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
    frame.render_widget(footer, chunks[3]);
}
