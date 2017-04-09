pub enum Key {
    Character(char),
    Control(char),
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
        match bytes[0] {
            0       => None,
            b'\x1b' => Self::from_escape_sequence(&bytes[1..]),
            1...31  => Some(Key::Control((bytes[0] | 0x40) as char)),
            _       => Some(Key::Character(bytes[0] as char))
        }
    }

    fn from_escape_sequence(bytes: &[u8]) -> Option<Self> {
        let default = Some(Key::Character('\x1b'));
        if bytes[0] == b'[' {
            if bytes[1] >= b'0' && bytes[1] <= b'9' {
                if bytes[2] == b'~' {
                    match bytes[1] {
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
                match bytes[1] {
                    b'A' => Some(Key::Arrow(ArrowKey::Up)),
                    b'B' => Some(Key::Arrow(ArrowKey::Down)),
                    b'C' => Some(Key::Arrow(ArrowKey::Right)),
                    b'D' => Some(Key::Arrow(ArrowKey::Left)),
                    b'H' => Some(Key::Home),
                    b'F' => Some(Key::End),
                    _    => default,
                }
            }
        } else if bytes[0] == b'O' {
            match bytes[1] {
                b'H' => Some(Key::Home),
                b'F' => Some(Key::End),
                _    => default,
            }
        } else {
            default
        }
    }
}
