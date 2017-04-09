pub fn safe_truncate(string: &mut String, i: usize) {
    if string.len() <= i {
        return
    } else if string.is_char_boundary(i) {
        string.truncate(i)
    } else {
        safe_truncate(string, i - 1)
    }
}

pub fn ctrl_key(key: u8) -> u8 { key & 0x1f }
