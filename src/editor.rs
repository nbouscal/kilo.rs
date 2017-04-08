use key::{Key, ArrowKey};
use terminal;

use std::io::{self, Read, Write};
use std::process;

const KILO_VERSION: &'static str = "0.0.1";

pub struct Editor {
    cursor_x: u16,
    cursor_y: u16,
    screen_rows: u16,
    screen_cols: u16,
    buffer: String,
}

impl Editor {
    pub fn new() -> Self {
        // TODO: Default to 24x80 if None?
        let (rows, cols) = terminal::get_window_size().unwrap();
        Editor {
            cursor_x: 0,
            cursor_y: 0,
            screen_rows: rows,
            screen_cols: cols,
            buffer: String::new(),
        }
    }

    pub fn refresh_screen(&mut self) {
        self.buffer.push_str("\x1b[?25l");
        self.buffer.push_str("\x1b[H");
        self.draw_rows();
        let set_cursor = format!("\x1b[{};{}H", self.cursor_y + 1, self.cursor_x + 1);
        self.buffer.push_str(&set_cursor);
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

                let padding = (self.screen_cols as usize - welcome.len()) / 2;
                if padding > 0 {
                    self.buffer.push_str("~");
                    let spaces = " ".repeat(padding - 1);
                    self.buffer.push_str(&spaces);
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

    fn read_key() -> Key {
        let mut bytes = [0; 4];
        let _ = io::stdin().read(&mut bytes);
        Key::from_bytes(&bytes)
    }

    fn move_cursor(&mut self, key: ArrowKey) {
        match key {
            ArrowKey::Left  => {
                if self.cursor_x > 0 { self.cursor_x -= 1 }
            },
            ArrowKey::Right => {
                if self.cursor_x < self.screen_cols - 1 { self.cursor_x += 1 }
            },
            ArrowKey::Up    => {
                if self.cursor_y > 0 { self.cursor_y -= 1 }
            },
            ArrowKey::Down  => {
                if self.cursor_y < self.screen_rows - 1 { self.cursor_y += 1 }
            },
        }
    }

    pub fn process_keypress(&mut self) {
        match Self::read_key() {
            Key::Character(c) => {
                if c == Self::ctrl_key(b'q') {
                    let _ = io::stdout().write(b"\x1b[2J");
                    let _ = io::stdout().write(b"\x1b[H");
                    let _ = io::stdout().flush();
                    process::exit(0)
                }
            },
            Key::Arrow(a) => self.move_cursor(a),
            Key::Delete => (),
            Key::Home => self.cursor_x = 0,
            Key::End => self.cursor_x = self.screen_cols - 1,
            Key::PageUp => {
                for _ in 0..self.screen_rows {
                    self.move_cursor(ArrowKey::Up)
                }
            },
            Key::PageDown => {
                for _ in 0..self.screen_rows {
                    self.move_cursor(ArrowKey::Down)
                }
            },
        }
    }
}
