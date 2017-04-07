use libc;

use std::mem;

static mut ORIG_TERMIOS: Option<libc::termios> = None;

extern "C" fn disable_raw_mode() {
    unsafe {
        let mut termios = ORIG_TERMIOS.unwrap();
        let errno = libc::tcsetattr(libc::STDIN_FILENO, libc::TCSAFLUSH, &mut termios);
        if errno == -1 { panic!("tcsetattr") }
    }
}

pub fn enable_raw_mode() {
    unsafe {
        let mut termios: libc::termios = mem::zeroed();

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

pub fn get_window_size() -> Option<(u16, u16)> {
    unsafe {
        let mut ws: libc::winsize = mem::zeroed();
        let errno = libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws);
        if errno == -1 || ws.ws_col == 0 {
            None
        } else {
            Some((ws.ws_row, ws.ws_col))
        }
    }
}
