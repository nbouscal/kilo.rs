extern crate libc;

use std::io::{self, Read};

fn enable_raw_mode() {
    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        libc::tcgetattr(libc::STDIN_FILENO, &mut termios as *mut libc::termios);
        termios.c_lflag &= !libc::ECHO;
        libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut termios);
    }
}

fn main() {
    enable_raw_mode();

    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);

    println!("{}", buffer);
}
