pub enum Key {
    Character(u8),
    Arrow(ArrowKey),
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
}

pub enum ArrowKey {
    Left,
    Right,
    Up,
    Down,
}

impl Key {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let default = Key::Character(bytes[0]);
        if bytes[0] == b'\x1b' {
            if bytes[1] == b'[' {
                if bytes[2] >= b'0' && bytes[2] <= b'9' {
                    if bytes[3] == b'~' {
                        match bytes[2] {
                            b'1' => Key::Home,
                            b'3' => Key::Delete,
                            b'4' => Key::End,
                            b'5' => Key::PageUp,
                            b'6' => Key::PageDown,
                            b'7' => Key::Home,
                            b'8' => Key::End,
                            _    => default,
                        }
                    } else {
                        default
                    }
                } else {
                    match bytes[2] {
                        b'A' => Key::Arrow(ArrowKey::Up),
                        b'B' => Key::Arrow(ArrowKey::Down),
                        b'C' => Key::Arrow(ArrowKey::Right),
                        b'D' => Key::Arrow(ArrowKey::Left),
                        b'H' => Key::Home,
                        b'F' => Key::End,
                        _    => default,
                    }
                }
            } else if bytes[1] == b'O' {
                match bytes[2] {
                    b'H' => Key::Home,
                    b'F' => Key::End,
                    _    => default,
                }
            } else {
                default
            }
        } else {
            default
        }
    }
}
