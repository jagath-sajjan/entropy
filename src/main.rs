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

    let mut save_ticks: u8 = 0; // FIXED: moved out of unsafe block, plain stack variable

    loop {
        {
            let app_lock = app.lock().unwrap();
            terminal.draw(|f| ui::draw(f, &app_lock))?;

            if app_lock.game_over {
                drop(app_lock);
                loop {
                    if events::wait_for_any_key()? {
                        break;
                    }
                }
                break;
            }

            if !app_lock.running {
                // FIXED: was app.lock.running, now correct
                break;
            }
        }

        events::handle_events(Arc::clone(&app))?;

        // clear save_msg after ~1 second of display
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
