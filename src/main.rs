mod app;
mod deletor;
mod events;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
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

    let app = Arc::new(Mutex::new(App::new()));

    deletor::start(Arc::clone(&app));

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
        }

        events::handle_events(Arc::clone(&app))?;
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
