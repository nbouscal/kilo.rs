use terminal;

use std::io::{self, Read, Write};
use std::process;

const KILO_VERSION: &'static str = "0.0.1";

pub struct Editor {
    screen_rows: u16,
    screen_cols: u16,
    buffer: String,
}

impl Editor {
    pub fn new() -> Self {
        // TODO: Default to 24x80 if None?
        let (rows, cols) = terminal::get_window_size().unwrap();
        Editor {
            screen_rows: rows,
            screen_cols: cols,
            buffer: String::new(),
        }
    }

    pub fn refresh_screen(&mut self) {
        self.buffer.push_str("\x1b[?25l");
        self.buffer.push_str("\x1b[H");
        self.draw_rows();
        self.buffer.push_str("\x1b[H");
        self.buffer.push_str("\x1b[?25h");
        let _ = io::stdout().write(self.buffer.as_bytes());
        let _ = io::stdout().flush();
        self.buffer.clear();
    }

    fn draw_rows(&mut self) {
        for i in 0..self.screen_rows {
            if i == self.screen_rows / 3 {
                let mut welcome = format!("Kilo editor -- version {}", KILO_VERSION);
                if welcome.len() > self.screen_cols as usize {
                    welcome.truncate(self.screen_cols as usize)
                }
                self.buffer.push_str(&welcome);
            } else {
                self.buffer.push_str("~");
            }

            self.buffer.push_str("\x1b[K");
            if i < self.screen_rows - 1 {
                self.buffer.push_str("\r\n");
            }
        }
    }

    fn ctrl_key(key: u8) -> u8 { key & 0x1f }

    fn read_key() -> u8 {
        let mut c = [0; 1];
        let _ = io::stdin().read(&mut c);
        c[0]
    }

    pub fn process_keypress(&self) {
        let c = Self::read_key();
        if c == Self::ctrl_key(b'q') {
            let _ = io::stdout().write(b"\x1b[2J");
            let _ = io::stdout().write(b"\x1b[H");
            let _ = io::stdout().flush();
            process::exit(0)
        }
    }
}
