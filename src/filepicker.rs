use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::io;
use std::path::{Path, PathBuf};

pub fn pick_file(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    dir: &Path,
) -> Result<Option<PathBuf>, io::Error> {
    let mut entries: Vec<PathBuf> = vec![];

    if let Some(parent) = dir.parent() {
        entries.push(parent.to_path_buf());
    }

    let mut dirs: Vec<PathBuf> = vec![];
    let mut files: Vec<PathBuf> = vec![];

    if let Ok(read) = std::fs::read_dir(dir) {
        for entry in read.flatten() {
            let p = entry.path();
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with('.') {
                continue;
            }
            if p.is_dir() {
                dirs.push(p);
            } else {
                files.push(p);
            }
        }
    }

    dirs.sort();
    files.sort();
    entries.extend(dirs);
    entries.extend(files);

    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)])
                .split(f.size());

            let items: Vec<ListItem> = entries
                .iter()
                .map(|p| {
                    let is_dir = p.is_dir();
                    let name = if p.parent() == Some(dir) || p == dir {
                        p.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("..")
                            .to_string()
                    } else {
                        "..".to_string()
                    };

                    let label = if is_dir {
                        format!("  📁 {}/", name)
                    } else {
                        format!("  📄 {}", name)
                    };

                    ListItem::new(Line::from(Span::raw(label)))
                })
                .collect();

            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!(" entropy — {}", dir.display())),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::White)
                        .add_modifier(Modifier::BOLD),
                );

            f.render_stateful_widget(list, chunks[0], &mut state);

            let hint = ratatui::widgets::Paragraph::new(Line::from(vec![Span::styled(
                " enter: open  |  esc: cancel",
                Style::default().fg(Color::DarkGray),
            )]));
            f.render_widget(hint, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    let i = state.selected().unwrap_or(0);
                    if i > 0 {
                        state.select(Some(i - 1));
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let i = state.selected().unwrap_or(0);
                    if i + 1 < entries.len() {
                        state.select(Some(i + 1));
                    }
                }
                KeyCode::Enter => {
                    if let Some(i) = state.selected() {
                        let selected = &entries[i];
                        if selected.is_dir() {
                            return pick_file(terminal, selected);
                        } else {
                            return Ok(Some(selected.clone()));
                        }
                    }
                }
                KeyCode::Esc => return Ok(None),
                _ => {}
            }
        }
    }
}
