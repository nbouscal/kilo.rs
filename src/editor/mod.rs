mod key;
mod row;

use self::key::{Key, ArrowKey};
use self::row::Row;
use terminal;
use util;

use std::cmp;
use std::io::{self, Read, BufRead, BufReader, Write};
use std::fs::File;
use std::process;
use std::time::{Duration, SystemTime};

const KILO_VERSION: &'static str = "0.0.1";
const KILO_QUIT_TIMES: u8 = 3;

pub struct Editor {
    cursor_x: u16,
    cursor_y: u16,
    row_offset: u16,
    col_offset: u16,
    screen_rows: u16,
    screen_cols: u16,
    write_buffer: String,
    rows: Vec<Row>,
    dirty: bool,
    quit_times: u8,
    filename: String,
    status_msg: String,
    status_time: SystemTime,
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
            screen_rows: rows - 2, // Leave space for status and message bars
            screen_cols: cols,
            write_buffer: String::new(),
            rows: Vec::new(),
            dirty: false,
            quit_times: KILO_QUIT_TIMES,
            filename: String::new(),
            status_msg: String::new(),
            status_time: SystemTime::now(),
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_past_end() {
            self.rows.push(Row::new());
        }
        let cursor_x = self.cursor_x as usize;
        {
            let mut current_row = self.current_row_mut().unwrap();
            current_row.insert_char(cursor_x, c);
        }
        self.cursor_x += 1;
        self.dirty = true;
    }

    pub fn insert_newline(&mut self) {
        if self.cursor_x == 0 {
            let cursor_y = self.cursor_y as usize;
            self.insert_row(cursor_y, String::new());
        } else {
            let cursor_x = self.cursor_x as usize;
            let remainder = self.current_row_mut().unwrap().split_off(cursor_x);
            let cursor_y = self.cursor_y as usize;
            self.insert_row(cursor_y + 1, remainder);
        }
        self.cursor_y += 1;
        self.cursor_x = 0;
        self.dirty = true;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_past_end() { return };
        if self.cursor_x == 0 && self.cursor_y == 0 { return };
        let cursor_x = self.cursor_x as usize;
        if cursor_x == 0 {
            let cursor_y = self.cursor_y as usize;
            self.cursor_x = self.rows[cursor_y - 1].contents.len() as u16;
            // Is there a way to avoid this clone?
            let s = self.current_row().unwrap().contents.clone();
            self.rows[cursor_y - 1].append_string(&s);
            self.delete_row(cursor_y);
            self.cursor_y -= 1;
        } else {
            self.current_row_mut().unwrap().delete_char(cursor_x - 1);
            self.cursor_x -= 1;
        }
        self.dirty = true;
    }

    fn insert_row(&mut self, at: usize, s: String) {
        if at <= self.rows.len() {
            self.rows.insert(at, Row::from_string(s));
        };
    }

    fn delete_row(&mut self, at: usize) {
        if at >= self.rows.len() { return }
        self.rows.remove(at);
    }

    fn rows_to_string(&self) -> String {
        self.rows.iter()
            .map(|row| row.contents.clone())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn open_file(&mut self, filename: &str) {
        self.filename = filename.to_string();
        let f = File::open(filename).unwrap(); // TODO: Handle error
        let reader = BufReader::new(f);
        self.rows = reader.lines()
            .map(|line| line.unwrap_or(String::new()))
            .map(Row::from_string).collect();
        self.dirty = false;
    }

    pub fn save_file(&mut self) {
        if self.filename.is_empty() { return }
        let mut f = File::create(&self.filename).unwrap(); // TODO: Handle error
        let bytes = f.write(&self.rows_to_string().as_bytes()).unwrap();
        self.set_status_message(&format!("{} bytes written to disk", bytes));
        self.dirty = false;
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
        self.draw_status_bar();
        self.draw_message_bar();
        let cursor_y = self.cursor_y - self.row_offset + 1;
        let cursor_x = self.rendered_cursor_x() - self.col_offset + 1;
        let set_cursor = format!("\x1b[{};{}H", cursor_y, cursor_x);
        self.write_buffer.push_str(&set_cursor);
        self.write_buffer.push_str("\x1b[?25h");
        let _ = io::stdout().write(self.write_buffer.as_bytes());
        let _ = io::stdout().flush();
        self.write_buffer.clear();
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
                    util::safe_truncate(&mut welcome, self.screen_cols as usize);

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
                util::safe_truncate(&mut row, self.screen_cols as usize);
                self.write_buffer.push_str(&row);
            }

            self.write_buffer.push_str("\x1b[K");
            self.write_buffer.push_str("\r\n");
        }
    }

    fn draw_status_bar(&mut self) {
        self.write_buffer.push_str("\x1b[7m");

        let mut filename = self.filename.clone();
        if filename.is_empty() {
            filename.push_str("[No Name]")
        } else {
            util::safe_truncate(&mut filename, 20);
        }
        let modified = if self.dirty { "(modified)" } else { "" };
        let mut status = format!("{} - {} lines {}", filename, self.rows.len(), modified);
        let rstatus = format!("{}/{}", self.cursor_y + 1, self.rows.len());
        if self.screen_cols as usize > status.len() + rstatus.len() {
            let padding = self.screen_cols as usize - status.len() - rstatus.len();
            status.push_str(&" ".repeat(padding));
        }
        status.push_str(&rstatus);
        util::safe_truncate(&mut status, self.screen_cols as usize);
        self.write_buffer.push_str(&status);

        self.write_buffer.push_str("\x1b[m");
        self.write_buffer.push_str("\r\n");
    }

    pub fn set_status_message(&mut self, msg: &str) {
        self.status_msg = msg.to_string();
        self.status_time = SystemTime::now();
    }

    fn draw_message_bar(&mut self) {
        self.write_buffer.push_str("\x1b[K");
        let mut message = self.status_msg.clone();
        util::safe_truncate(&mut message, self.screen_cols as usize);
        if self.status_time.elapsed().unwrap() < Duration::from_secs(5) {
            self.write_buffer.push_str(&message);
        }
    }

    fn read_key() -> Option<Key> {
        // FIXME: This is the likely source of a bug where if you spam
        //        arrow keys fast enough, it inserts [ characters
        let mut bytes = [0; 4];
        let _ = io::stdin().read(&mut bytes);
        Key::from_bytes(&bytes)
    }

    fn cursor_past_end(&self) -> bool {
        self.cursor_y as usize >= self.rows.len()
    }

    fn current_row(&self) -> Option<&Row> {
        if self.cursor_past_end() {
            None
        } else {
            Some(&self.rows[self.cursor_y as usize])
        }
    }

    fn current_row_mut(&mut self) -> Option<&mut Row> {
        if self.cursor_past_end() {
            None
        } else {
            Some(&mut self.rows[self.cursor_y as usize])
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
        let key = Self::read_key();
        if key.is_none() { return }
        match key.unwrap() {
            Key::Character(c) => self.insert_char(c),
            Key::Control('M') => self.insert_newline(),
            Key::Control('S') => self.save_file(),
            Key::Control('Q') => {
                self.exit();
                return;
            },
            Key::Control(_)   => (),
            Key::Arrow(a)     => self.move_cursor(a),
            Key::Escape       => (),
            Key::Backspace    => self.delete_char(),
            Key::Delete       => {
                self.move_cursor(ArrowKey::Right);
                self.delete_char();
            },
            Key::Home         => self.cursor_x = 0,
            Key::End          => self.cursor_x = self.current_row_size().unwrap_or(0),
            Key::PageUp       => self.page_up(),
            Key::PageDown     => self.page_down(),
        }
        self.quit_times = KILO_QUIT_TIMES;
    }

    fn exit(&mut self) {
        if self.dirty && self.quit_times > 0 {
            let quit_times = self.quit_times;
            self.set_status_message(&format!("WARNING!!! File has unsaved changes. Press Ctrl-Q {} more times to quit.", quit_times));
            self.quit_times -= 1;
        } else {
            let _ = io::stdout().write(b"\x1b[2J");
            let _ = io::stdout().write(b"\x1b[H");
            let _ = io::stdout().flush();
            process::exit(0)
        }
    }

    fn page_up(&mut self) {
        self.cursor_y = self.row_offset;
        for _ in 0..self.screen_rows {
            self.move_cursor(ArrowKey::Up)
        }
    }

    fn page_down(&mut self) {
        self.cursor_y = cmp::min(self.rows.len() as u16, self.row_offset + self.screen_rows - 1);
        for _ in 0..self.screen_rows {
            self.move_cursor(ArrowKey::Down)
        }
    }
}
