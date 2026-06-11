use std::path::PathBuf;

pub struct App {
    pub buffer: String,
    pub cursor_pos: usize,
    pub running: bool,
    pub game_over: bool,
    pub chars_deleted: usize,
    pub score: usize,
    pub danger_level: u8,
    pub flicker_pos: Option<usize>,
    pub warning_msg: Option<&'static str>,
    pub file_path: Option<PathBuf>,
    pub save_msg: Option<&'static str>,
}

impl App {
    pub fn new() -> Self {
        App {
            buffer: String::new(),
            cursor_pos: 0,
            running: true,
            game_over: false,
            chars_deleted: 0,
            score: 0,
            danger_level: 0,
            flicker_pos: None,
            warning_msg: None,
            file_path: None,
            save_msg: None,
        }
    }

    pub fn with_file(path: PathBuf) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(&path)?;
        let buffer = content.replace("\r\n", "\n");
        let cursor_pos = buffer.len();
        let mut app = Self::new();
        app.buffer = buffer;
        app.cursor_pos = cursor_pos;
        app.file_path = Some(path);
        Ok(app)
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
        self.score += 1;
    }

    pub fn delete_char_before_cursor(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.buffer.remove(self.cursor_pos);
        }
    }

    pub fn save(&mut self) {
        if let Some(ref path) = self.file_path {
            let content = if cfg!(target_os = "windows") {
                self.buffer.replace('\n', "\r\n")
            } else {
                self.buffer.clone()
            };
            if std::fs::write(path, content).is_ok() {
                self.save_msg = Some("  ✓ saved");
            } else {
                self.save_msg = Some("  ✗ save failed");
            }
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn update_danger(&mut self) {
        self.danger_level = match self.chars_deleted {
            0..=9 => 0,
            10..=24 => 1,
            25..=49 => 2,
            _ => 3,
        };
    }
}
