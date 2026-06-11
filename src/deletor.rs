use crate::app::App;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const WARNINGS: &[&str] = &[
    "the void is hungwey",
    "it know's you're here",
    "your words mean NTHHH!!",
    "resitance is temporary",
    "it accelerates",
    "you cannot win lad",
    "ever char is borrowed time",
    "tick.",
];

const GLYPHS: &[char] = &[
    '▓', '▒', '░', '█', '▄', '▀', '■', '□', '▪', '▫', '†', '‡', '§', '¶', '©', '®', '×', '÷', '¿',
    '¡', 'ψ', 'Ω', 'λ', 'Σ', 'Δ', 'Φ', 'Ξ', 'Π', 'Λ', 'Γ', '₿', '∞', '∅', '∇', '∂', '√', '∫', '≠',
    '≈', '±',
];

pub fn start(app: Arc<Mutex<App>>) {
    thread::spawn(move || {
        let mut interval = Duration::from_millis(3000);
        let mut tick: u64 = 0;

        loop {
            thread::sleep(Duration::from_millis(100));
            tick += 1;

            if tick % 20 == 0 {
                let mut app = app.lock().unwrap();
                let char_count = app.buffer.chars().count();
                if char_count > 0 {
                    let char_idx = pseudo_random(tick) as usize % char_count;
                    app.flicker_pos = Some(char_idx);
                    let glyph_idx = pseudo_random(tick + 99) as usize % GLYPHS.len();
                    app.flicker_char = GLYPHS[glyph_idx];
                }
            }

            if tick % 20 == 2 {
                app.lock().unwrap().flicker_pos = None;
            }

            if tick % 70 == 0 {
                let mut app = app.lock().unwrap();
                let idx = pseudo_random(tick + 1) as usize % WARNINGS.len();
                app.warning_msg = Some(WARNINGS[idx]);
            }

            if tick % 70 == 10 {
                app.lock().unwrap().warning_msg = None;
            }

            if tick % (interval.as_millis() as u64 / 100) == 0 {
                let mut app = app.lock().unwrap();

                if !app.running {
                    break;
                }

                let char_count = app.buffer.chars().count();

                if char_count == 0 {
                    app.game_over = true;
                    app.running = false;
                    break;
                }

                let char_idx = pseudo_random(tick) as usize % char_count;
                let byte_idx = app
                    .buffer
                    .char_indices()
                    .nth(char_idx)
                    .map(|(i, _)| i)
                    .unwrap_or(0);

                app.buffer.remove(byte_idx);
                app.chars_deleted += 1;
                app.update_danger();

                let new_char_count = app.buffer.chars().count();
                if app.cursor_pos > new_char_count {
                    app.cursor_pos = new_char_count;
                }

                if app.chars_deleted % 5 == 0 && interval > Duration::from_millis(800) {
                    interval = Duration::from_millis((interval.as_millis() as f64 * 0.75) as u64);
                }
            }
        }
    });
}

fn pseudo_random(seed: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as u64;
    t.wrapping_mul(6364136223846793005)
        .wrapping_add(seed.wrapping_mul(1442695040888963407))
        ^ (t >> 17)
}
