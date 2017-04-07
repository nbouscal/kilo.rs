use terminal;

use std::io::{self, Read, Write};
use std::process;

pub struct Editor {
    screen_rows: u16,
    screen_cols: u16,
}

impl Editor {
    pub fn new() -> Self {
        // TODO: Default to 24x80 if None?
        let (rows, cols) = terminal::get_window_size().unwrap();
        Editor {
            screen_rows: rows,
            screen_cols: cols,
        }
    }

    pub fn refresh_screen(&self) {
        let _ = io::stdout().write(b"\x1b[2J");
        let _ = io::stdout().write(b"\x1b[H");
        self.draw_rows();
        let _ = io::stdout().write(b"\x1b[H");
        let _ = io::stdout().flush();
    }

    fn draw_rows(&self) {
        for _ in 0..self.screen_rows {
            let _ = io::stdout().write(b"~\r\n");
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
