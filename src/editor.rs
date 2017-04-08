use key::{Key, ArrowKey};
use terminal;

use std::io::{self, Read, BufRead, BufReader, Write};
use std::fs::File;
use std::process;

const KILO_VERSION: &'static str = "0.0.1";

pub struct Editor {
    cursor_x: u16,
    cursor_y: u16,
    screen_rows: u16,
    screen_cols: u16,
    write_buffer: String,
    rows: Vec<String>,
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
            write_buffer: String::new(),
            rows: Vec::new(),
        }
    }

    pub fn open_file(&mut self, filename: &str) {
        let f = File::open(filename).unwrap(); // TODO: Handle error
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        let has_newline;
        {
            let bytes = line.as_bytes();
            has_newline = bytes[bytes.len() - 1] == b'\n' || bytes[bytes.len() - 1] == b'\r';
        }
        if has_newline { line.pop(); }
        self.rows = vec![line];
    }

    pub fn refresh_screen(&mut self) {
        self.write_buffer.push_str("\x1b[?25l");
        self.write_buffer.push_str("\x1b[H");
        self.draw_rows();
        let set_cursor = format!("\x1b[{};{}H", self.cursor_y + 1, self.cursor_x + 1);
        self.write_buffer.push_str(&set_cursor);
        self.write_buffer.push_str("\x1b[?25h");
        let _ = io::stdout().write(self.write_buffer.as_bytes());
        let _ = io::stdout().flush();
        self.write_buffer.clear();
    }

    fn draw_rows(&mut self) {
        for i in 0..self.screen_rows {
            if i as usize >= self.rows.len() {
                if self.rows.is_empty() && i == self.screen_rows / 3 {
                    let mut welcome = format!("Kilo editor -- version {}", KILO_VERSION);
                    welcome.truncate(self.screen_cols as usize);

                    let padding = (self.screen_cols as usize - welcome.len()) / 2;
                    if padding > 0 {
                        self.write_buffer.push_str("~");
                        let spaces = " ".repeat(padding - 1);
                        self.write_buffer.push_str(&spaces);
                    }

                    self.write_buffer.push_str(&welcome);
                } else {
                    self.write_buffer.push_str("~");
                }
            } else {
                let ref mut row = self.rows[0];
                row.truncate(self.screen_cols as usize);
                self.write_buffer.push_str(&row);
            }

            self.write_buffer.push_str("\x1b[K");
            if i < self.screen_rows - 1 {
                self.write_buffer.push_str("\r\n");
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
