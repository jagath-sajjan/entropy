pub struct App {
    pub buffer: String,
    pub cursor_pos: usize,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            buffer: String::new(),
            cursor_pos: 0,
            running: true,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    pub fn delete_char_before_cursor(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.buffer.remove(self.cursor_pos);
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
