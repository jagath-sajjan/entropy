mod app;
mod args;
mod deletor;
mod events;
mod filepicker;
mod ui;

use app::App;
use args::Mode;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mode = args::parse();

    let app = match mode {
        Mode::Empty => App::new(),
        Mode::File(path) => {
            if !path.exists() {
                std::fs::write(&path, "")?;
            }
            App::with_file(path).unwrap_or_else(|_| App::new())
        }
        Mode::Picker(dir) => {
            let chosen = filepicker::pick_file(&mut terminal, &dir)?;
            match chosen {
                Some(path) => App::with_file(path).unwrap_or_else(|_| App::new()),
                None => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    return Ok(());
                }
            }
        }
    };

    let app = Arc::new(Mutex::new(app));
    deletor::start(Arc::clone(&app));

    let mut save_ticks: u8 = 0;

    loop {
        let (game_over, running, snapshot) = {
            let mut app_lock = app.lock().unwrap();
            app_lock.tick_cursor();

            let char_count = app_lock.buffer.chars().count();
            if app_lock.cursor_pos > char_count {
                app_lock.cursor_pos = char_count;
            }
            if let Some(fp) = app_lock.flicker_pos {
                if fp >= char_count {
                    app_lock.flicker_pos = None;
                }
            }

            let game_over = app_lock.game_over;
            let running = app_lock.running;
            let snapshot = app_lock.clone();
            (game_over, running, snapshot)
        };

        terminal.draw(|f| ui::draw(f, &snapshot))?;

        if game_over {
            loop {
                if events::wait_for_any_key()? {
                    break;
                }
            }
            break;
        }

        if !running {
            break;
        }

        events::handle_events(Arc::clone(&app))?;

        {
            let mut app_lock = app.lock().unwrap();
            if app_lock.save_msg.is_some() {
                save_ticks += 1;
                if save_ticks > 20 {
                    app_lock.save_msg = None;
                    save_ticks = 0;
                }
            } else {
                save_ticks = 0;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
