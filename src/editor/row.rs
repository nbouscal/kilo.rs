use util;

use std::iter;

const KILO_TAB_STOP: usize = 8;

pub struct Row {
    pub contents: String,
    pub render: String,
}

impl Row {
    pub fn new() -> Self {
        Row { contents: String::new(), render: String::new() }
    }

    pub fn from_string(s: String) -> Self {
        Row {
            contents: s.clone(),
            render: Self::render_string(s),
        }
    }

    pub fn insert_char(&mut self, at: usize, c: char) {
        self.contents.insert(at, c);
        self.update_render();
    }

    pub fn delete_char(&mut self, at: usize) {
        if at >= self.contents.len() { return }
        self.contents.remove(at);
        self.update_render();
    }

    pub fn append_string(&mut self, s: &str) {
        self.contents.push_str(s);
        self.update_render();
    }

    pub fn split_off(&mut self, at: usize) -> String {
        let remainder = util::safe_split_off(&mut self.contents, at);
        self.update_render();
        remainder
    }

    pub fn rendered_cursor_x(&self, cursor_x: u16) -> u16 {
        self.contents.chars()
            .take(cursor_x as usize)
            .fold(0, |acc, c| {
                if c == '\t' {
                    acc + KILO_TAB_STOP as u16 - (acc % KILO_TAB_STOP as u16)
                } else {
                    acc + 1
                }
        })
    }

    pub fn raw_cursor_x(&self, rendered_x: u16) -> u16 {
        self.contents.chars()
            .scan(0, |acc, c| {
                if c == '\t' {
                    *acc = *acc + KILO_TAB_STOP as u16 - (*acc % KILO_TAB_STOP as u16)
                } else {
                    *acc += 1
                };
                Some(*acc)
            }).position(|rx| rx > rendered_x).unwrap() as u16
    }

    fn update_render(&mut self) {
        self.render = Self::render_string(self.contents.clone());
    }

    fn render_string(s: String) -> String {
        let mut idx = 0;
        let renderer = |c|
            if c == '\t' {
                let n = KILO_TAB_STOP - (idx % KILO_TAB_STOP);
                idx += n;
                iter::repeat(' ').take(n)
            } else {
                idx += 1;
                // This is the same as iter::once(c), but the types of
                // the branches of the conditional have to line up.
                iter::repeat(c).take(1)
            };
        s.chars().flat_map(renderer).collect()
    }
}
