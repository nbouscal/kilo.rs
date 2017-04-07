extern crate libc;

mod terminal;

use std::io::{self, Read, Write};
use std::process;


fn ctrl_key(key: u8) -> u8 { key & 0x1f }

fn editor_read_key() -> u8 {
    let mut c = [0; 1];
    let _ = io::stdin().read(&mut c);
    c[0]
}

fn editor_draw_rows() {
    for _ in 0..24 {
        let _ = io::stdout().write(b"~\r\n");
    }
}

fn editor_refresh_screen() {
    let _ = io::stdout().write(b"\x1b[2J");
    let _ = io::stdout().write(b"\x1b[H");
    editor_draw_rows();
    let _ = io::stdout().write(b"\x1b[H");
    let _ = io::stdout().flush();
}

fn editor_process_keypress() {
    let c = editor_read_key();
    if c == ctrl_key(b'q') {
        let _ = io::stdout().write(b"\x1b[2J");
        let _ = io::stdout().write(b"\x1b[H");
        let _ = io::stdout().flush();
        process::exit(0)
    }
}

fn main() {
    terminal::enable_raw_mode();
    loop {
        editor_refresh_screen();
        editor_process_keypress();
    }
}
