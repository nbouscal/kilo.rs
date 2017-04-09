pub enum Key {
    Character(char),
    Control(char),
    Arrow(ArrowKey),
    Escape,
    Backspace,
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
            b'\x1b' => Some(Self::from_escape_sequence(&bytes[1..])),
            8 | 127 => Some(Key::Backspace),
            1...31  => Some(Key::Control((bytes[0] | 0x40) as char)),
            _       => Some(Key::Character(bytes[0] as char))
        }
    }

    fn from_escape_sequence(bytes: &[u8]) -> Self {
        match bytes {
            b"[A\0" => Key::Arrow(ArrowKey::Up),
            b"[B\0" => Key::Arrow(ArrowKey::Down),
            b"[C\0" => Key::Arrow(ArrowKey::Right),
            b"[D\0" => Key::Arrow(ArrowKey::Left),
            b"[3~"  => Key::Delete,
            b"[1~" | b"[7~" | b"[H" | b"OH" => Key::Home,
            b"[4~" | b"[8~" | b"[F" | b"OF" => Key::End,
            b"[5~" => Key::PageUp,
            b"[6~" => Key::PageDown,
            _      => Key::Escape,
        }
    }
}
