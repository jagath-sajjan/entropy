use crate::app::App;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

fn danger_color(level: u8) -> Color {
    match level {
        0 => Color::White,
        1 => Color::Yellow,
        2 => Color::LightRed,
        _ => Color::Red,
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.size());

    let color = danger_color(app.danger_level);
    let display_text = build_display(app);

    let title = match &app.file_path {
        Some(p) => format!(
            " entropy — {} ",
            p.file_name().and_then(|n| n.to_str()).unwrap_or("unknown")
        ),
        None => " entropy ~ type faster than the void ".to_string(),
    };

    let editor = Paragraph::new(display_text.as_str()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .title(Span::styled(title, Style::default().fg(color))),
    );

    f.render_widget(editor, chunks[0]);

    let status_spans = build_status(app, color);
    let status = Paragraph::new(Line::from(status_spans));
    f.render_widget(status, chunks[1]);

    if app.game_over {
        draw_game_over(f, app);
    }
}

fn build_display(app: &App) -> String {
    let mut chars: Vec<char> = app.buffer.chars().collect();
    let len = chars.len();

    if let Some(fidx) = app.flicker_pos {
        if fidx < len && fidx != app.cursor_pos {
            chars[fidx] = app.flicker_char;
        }
    }

    if app.cursor_visible {
        let pos = app.cursor_pos.min(len);
        if pos < len {
            chars[pos] = '█';
        } else {
            chars.push('█');
        }
    }

    chars.iter().collect()
}

fn build_status(app: &App, color: Color) -> Vec<Span<'static>> {
    if let Some(msg) = app.save_msg {
        return vec![Span::styled(
            msg,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )];
    }

    if let Some(msg) = app.warning_msg {
        return vec![Span::styled(
            format!(" ⚠  {} ", msg),
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::RAPID_BLINK),
        )];
    }

    let save_hint = if app.file_path.is_some() {
        "  |  ctrl+s to save"
    } else {
        ""
    };

    vec![
        Span::styled(" chars: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            app.buffer.chars().count().to_string(),
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
        Span::styled("  |  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            danger_bar(app.danger_level),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(save_hint, Style::default().fg(Color::DarkGray)),
        Span::styled("  |  ctrl+q to quit", Style::default().fg(Color::DarkGray)),
    ]
}

fn danger_bar(level: u8) -> String {
    let filled = level as usize;
    let empty = 3usize.saturating_sub(filled);
    let mut s = String::new();
    for _ in 0..filled {
        s.push('▓');
    }
    for _ in 0..empty {
        s.push('░');
    }
    s
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
            Span::styled("  chars typed:   ", Style::default().fg(Color::DarkGray)),
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
