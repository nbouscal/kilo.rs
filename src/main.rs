extern crate libc;

use std::io::{self, Read, Write};
use std::process;

static mut ORIG_TERMIOS: Option<libc::termios> = None;

fn ctrl_key(key: u8) -> u8 { key & 0x1f }

extern "C" fn disable_raw_mode() {
    unsafe {
        let mut termios = ORIG_TERMIOS.unwrap();
        let errno = libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut termios);
        if errno == -1 { panic!("tcsetattr") }
    }
}

fn enable_raw_mode() {
    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();

        let errno = libc::tcgetattr(libc::STDIN_FILENO, &mut termios as *mut libc::termios);
        if errno == -1 { panic!("tcgetattr") }

        ORIG_TERMIOS = Some(termios);
        libc::atexit(disable_raw_mode);

        termios.c_iflag &= !(libc::BRKINT | libc::ICRNL | libc::INPCK | libc::ISTRIP | libc::IXON);
        termios.c_oflag &= !libc::OPOST;
        termios.c_cflag |= libc::CS8;
        termios.c_lflag &= !(libc::ECHO | libc::ICANON | libc::IEXTEN | libc::ISIG);
        termios.c_cc[libc::VMIN] = 0;
        termios.c_cc[libc::VTIME] = 1;

        let errno = libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut termios);
        if errno == -1 { panic!("tcsetattr") }
    }
}

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
    enable_raw_mode();
    loop {
        editor_refresh_screen();
        editor_process_keypress();
    }
}
