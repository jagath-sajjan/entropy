use crate::app::App;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn handle_events(app: Arc<Mutex<App>>) -> Result<(), io::Error> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            let mut app = app.lock().unwrap();
            match (key.code, key.modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => app.quit(),
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => app.quit(), // ADDED: ctrl+c also quits
                (KeyCode::Char(c), _) => app.insert_char(c),
                (KeyCode::Backspace, _) => app.delete_char_before_cursor(),
                (KeyCode::Enter, _) => app.insert_char('\n'),
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn wait_for_any_key() -> Result<bool, io::Error> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(_) = event::read()? {
            return Ok(true);
        }
    }
    Ok(false)
}
