pub enum Key {
    Character(char),
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
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes[0] == 0 { return None }
        let default = Some(Key::Character(bytes[0] as char));
        if bytes[0] == b'\x1b' {
            if bytes[1] == b'[' {
                if bytes[2] >= b'0' && bytes[2] <= b'9' {
                    if bytes[3] == b'~' {
                        match bytes[2] {
                            b'1' => Some(Key::Home),
                            b'3' => Some(Key::Delete),
                            b'4' => Some(Key::End),
                            b'5' => Some(Key::PageUp),
                            b'6' => Some(Key::PageDown),
                            b'7' => Some(Key::Home),
                            b'8' => Some(Key::End),
                            _    => default,
                        }
                    } else {
                        default
                    }
                } else {
                    match bytes[2] {
                        b'A' => Some(Key::Arrow(ArrowKey::Up)),
                        b'B' => Some(Key::Arrow(ArrowKey::Down)),
                        b'C' => Some(Key::Arrow(ArrowKey::Right)),
                        b'D' => Some(Key::Arrow(ArrowKey::Left)),
                        b'H' => Some(Key::Home),
                        b'F' => Some(Key::End),
                        _    => default,
                    }
                }
            } else if bytes[1] == b'O' {
                match bytes[2] {
                    b'H' => Some(Key::Home),
                    b'F' => Some(Key::End),
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
