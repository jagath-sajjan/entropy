use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.size());

    let editor = Paragraph::new(app.buffer.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" entropy — type faster than the void "),
    );

    f.render_widget(editor, chunks[0]);

    let status = Paragraph::new(Line::from(vec![
        Span::styled(" chars: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            app.buffer.len().to_string(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  |  deleted: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            app.chars_deleted.to_string(),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  |  score: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            app.score.to_string(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  |  ctrl+q to quit", Style::default().fg(Color::DarkGray)),
    ]));

    f.render_widget(status, chunks[1]);
}
