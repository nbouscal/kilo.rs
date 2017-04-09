pub fn safe_truncate(string: &mut String, i: usize) {
    if string.len() <= i {
        return
    } else if string.is_char_boundary(i) {
        string.truncate(i)
    } else {
        safe_truncate(string, i - 1)
    }
}
