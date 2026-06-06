use crate::app::App;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;
use std::sync::{Arc, Mutex};

pub fn handle_events(app: Arc<Mutex<App>>) -> Result<(), io::Error> {
    if let Event::Key(key) = event::read()? {
        let mut app = app.lock().unwrap();
        match (key.code, key.modifiers) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => app.quit(),
            (KeyCode::Char(c), _) => app.insert_char(c),
            (KeyCode::Backspace, _) => app.delete_char_before_cursor(),
            (KeyCode::Enter, _) => app.insert_char('\n'),
            _ => {}
        }
    }
    Ok(())
}
