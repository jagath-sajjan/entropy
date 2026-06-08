use crate::app::App;
use ratatui::{
    Frame,
    layout::{
        Alignment,
        Constraint::{self, Percentage},
        Direction, Layout, Rect,
    },
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage((percent_y)),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage((percent_x)),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

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

    if app.game_over {
        draw_game_over(f, app);
    }
}

fn draw_game_over(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 40, f.size());

    f.render_widget(Clear, area);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ░░ ENTROPY WINS ░░",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(" chars typed: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                app.score.to_string(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  chars deleted: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                app.chars_deleted.to_string(),
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  press any key to exit",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let popup = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title(" game over ")
            .title_alignment(Alignment::Center),
    );
    f.render_widget(popup, area);
}
