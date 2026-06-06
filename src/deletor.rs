use crate::app::App;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start(app: Arc<Mutex<App>>) {
    thread::spawn(move || {
        let mut interval = Duration::from_secs(10);

        loop {
            thread::sleep(interval);

            let mut app = app.lock().unwrap();

            if !app.running {
                break;
            }

            if app.buffer.is_empty() {
                app.game_over = true;
                app.running = false;
                break;
            }

            let len = app.buffer.len();
            let idx = (pseudo_random(len as u64) % len as u64) as usize;

            let safe_idx = app
                .buffer
                .char_indices()
                .map(|(i, _)| i)
                .filter(|&i| i <= idx)
                .last()
                .unwrap_or(0);

            app.buffer.remove(safe_idx);
            app.chars_deleted += 1;

            if app.cursor_pos > app.buffer.len() {
                app.cursor_pos = app.buffer.len();
            }

            if app.chars_deleted % 10 == 0 && interval > Duration::from_secs(2) {
                interval = Duration::from_millis((interval.as_millis() as f64 * 0.8) as u64);
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
    (seed.wrapping_mul(6364136223846793005).wrapping_add(t)) % u64::MAX
}
