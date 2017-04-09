use key::{Key, ArrowKey};
use terminal;

use std::io::{self, Read, BufRead, BufReader, Write};
use std::iter;
use std::fs::File;
use std::process;

const KILO_VERSION: &'static str = "0.0.1";
const KILO_TAB_STOP: usize = 8;

struct Row {
    contents: String,
    render: String,
}

pub struct Editor {
    cursor_x: u16,
    cursor_y: u16,
    row_offset: u16,
    col_offset: u16,
    screen_rows: u16,
    screen_cols: u16,
    write_buffer: String,
    rows: Vec<Row>,
}

impl Row {
    pub fn from_string(s: String) -> Self {
        Row {
            contents: s.clone(),
            render: Self::render_string(s),
        }
    }

    pub fn rendered_cursor_x(&self, cursor_x: u16) -> u16 {
        self.contents.chars()
            .take(cursor_x as usize)
            .fold(0, |acc, c| {
                if c == '\t' {
                    acc + KILO_TAB_STOP as u16 - (acc % KILO_TAB_STOP as u16)
                } else {
                    acc + 1
                }
        })
    }

    fn render_string(s: String) -> String {
        let mut idx = 0;
        let renderer = |c|
            if c == '\t' {
                let n = KILO_TAB_STOP - (idx % KILO_TAB_STOP);
                idx += n;
                iter::repeat(' ').take(n)
            } else {
                idx += 1;
                // This is the same as iter::once(c), but the types of
                // the branches of the conditional have to line up.
                iter::repeat(c).take(1)
            };
        s.chars().flat_map(renderer).collect()
    }
}

impl Editor {
    pub fn new() -> Self {
        // TODO: Default to 24x80 if None?
        let (rows, cols) = terminal::get_window_size().unwrap();
        Editor {
            cursor_x: 0,
            cursor_y: 0,
            row_offset: 0,
            col_offset: 0,
            screen_rows: rows,
            screen_cols: cols,
            write_buffer: String::new(),
            rows: Vec::new(),
        }
    }

    pub fn open_file(&mut self, filename: &str) {
        let f = File::open(filename).unwrap(); // TODO: Handle error
        let reader = BufReader::new(f);
        self.rows = reader.lines()
            .map(|line| line.unwrap_or(String::new()))
            .map(Row::from_string).collect();
    }

    fn rendered_cursor_x(&self) -> u16 {
        self.current_row()
            .map_or(0, |row| row.rendered_cursor_x(self.cursor_x))
    }

    pub fn refresh_screen(&mut self) {
        self.scroll();
        self.write_buffer.push_str("\x1b[?25l");
        self.write_buffer.push_str("\x1b[H");
        self.draw_rows();
        let cursor_y = self.cursor_y - self.row_offset + 1;
        let cursor_x = self.rendered_cursor_x() - self.col_offset + 1;
        let set_cursor = format!("\x1b[{};{}H", cursor_y, cursor_x);
        self.write_buffer.push_str(&set_cursor);
        self.write_buffer.push_str("\x1b[?25h");
        let _ = io::stdout().write(self.write_buffer.as_bytes());
        let _ = io::stdout().flush();
        self.write_buffer.clear();
    }

    fn safe_truncate(string: &mut String, i: usize) {
        if string.len() <= i {
            return
        } else if string.is_char_boundary(i) {
            string.truncate(i)
        } else {
            Self::safe_truncate(string, i - 1)
        }
    }

    fn scroll(&mut self) {
        let rx = self.rendered_cursor_x();
        if self.cursor_y < self.row_offset {
            self.row_offset = self.cursor_y;
        } else if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }
        if rx < self.col_offset {
            self.col_offset = rx;
        } else if rx >= self.col_offset + self.screen_cols {
            self.col_offset = rx - self.screen_cols + 1;
        }
    }

    fn draw_rows(&mut self) {
        for i in 0..self.screen_rows {
            let file_row = i + self.row_offset;
            if file_row as usize >= self.rows.len() {
                if self.rows.is_empty() && i == self.screen_rows / 3 {
                    let mut welcome = format!("Kilo editor -- version {}", KILO_VERSION);
                    Self::safe_truncate(&mut welcome, self.screen_cols as usize);

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
                let ref mut row = self.rows[file_row as usize].render;
                let mut row = row.chars().skip(self.col_offset as usize).collect::<String>();
                Self::safe_truncate(&mut row, self.screen_cols as usize);
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

    fn current_row(&self) -> Option<&Row> {
        if self.cursor_y as usize >= self.rows.len() {
            None
        } else {
            Some(&self.rows[self.cursor_y as usize])
        }
    }

    fn current_row_size(&self) -> Option<u16> {
        self.current_row().map(|row| row.contents.len() as u16)
    }

    fn move_cursor(&mut self, key: ArrowKey) {
        match key {
            ArrowKey::Left  => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.current_row_size().unwrap();
                }
            },
            ArrowKey::Right => {
                match self.current_row_size() {
                    Some(current_row_size) => {
                        if self.cursor_x < current_row_size {
                            self.cursor_x += 1
                        } else if self.cursor_x == current_row_size {
                            self.cursor_y += 1;
                            self.cursor_x = 0;
                        }
                    },
                    None => ()
                }
            },
            ArrowKey::Up    => {
                if self.cursor_y > 0 { self.cursor_y -= 1 }
            },
            ArrowKey::Down  => {
                if (self.cursor_y as usize) < self.rows.len() {
                    self.cursor_y += 1
                }
            },
        }
        let current_row_size = self.current_row_size().unwrap_or(0);
        if (self.cursor_x) > current_row_size {
            self.cursor_x = current_row_size;
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
