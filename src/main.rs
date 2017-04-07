extern crate libc;

use std::io::{self, Read};

static mut ORIG_TERMIOS: Option<libc::termios> = None;

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

fn main() {
    enable_raw_mode();

    let mut c = [0; 1];
    loop {
        c[0] = 0;
        let _ = io::stdin().read(&mut c);
        print!("{:?}\r\n", c);
        if c.contains(&b'q') { break; }
    }
}
