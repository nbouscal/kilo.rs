extern crate libc;

use std::io::{self, Read};

static mut ORIG_TERMIOS: Option<libc::termios> = None;

extern "C" fn disable_raw_mode() {
    unsafe {
        let mut termios = ORIG_TERMIOS.unwrap();
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut termios);
    }
}

fn enable_raw_mode() {
    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut termios as *mut libc::termios);
        ORIG_TERMIOS = Some(termios);
        libc::atexit(disable_raw_mode);
        termios.c_lflag &= !(libc::ECHO | libc::ICANON);
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut termios);
    }
}

fn main() {
    enable_raw_mode();

    let mut c = [0; 1];
    loop {
        let _ = io::stdin().read(&mut c);
        println!("{:?}", c);
        if c.contains(&b'q') { break; }
    }
}
