extern crate libc;

use std::io;

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
    loop {
        let _ = io::stdin().read_line(&mut buffer);
        if buffer.contains("q") { break; }
    }

    println!("{}", buffer);
}
