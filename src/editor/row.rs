use util;

use std::iter;

const KILO_TAB_STOP: usize = 8;

pub struct Row {
    pub contents: String,
    pub render: String,
    pub highlight: Vec<Highlight>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Highlight {
    Normal,
    Number,
    Match,
}

impl Highlight {
    pub fn to_color(&self) -> u8 {
        match *self {
            Highlight::Normal => 37,
            Highlight::Number => 31,
            Highlight::Match  => 34,
        }
    }
}

fn is_separator(c: char) -> bool {
    c.is_whitespace() ||
        c == '\0' ||
        ",.()+-/*=~%<>[];".contains(c)
}

impl Row {
    pub fn new() -> Self {
        Row {
            contents: String::new(),
            render: String::new(),
            highlight: Vec::new(),
        }
    }

    pub fn from_string(s: String) -> Self {
        let mut row = Self::new();
        row.append_string(&s);
        row
    }

    fn update(&mut self) {
        self.update_render();
        self.update_syntax();
    }

    fn update_syntax(&mut self) {
        self.highlight = iter::repeat(Highlight::Normal)
            .take(self.render.chars().count()).collect();
        let mut prev_sep = true;
        for (i, c) in self.render.chars().enumerate() {
            let prev_hl = if i > 0 {
                self.highlight[i - 1]
            } else {
                Highlight::Normal
            };
            if (c.is_digit(10) && (prev_sep || prev_hl == Highlight::Number)) || (c == '.' && prev_hl == Highlight::Number) {
                prev_sep = false;
                self.highlight[i] = Highlight::Number;
                continue;
            }
            prev_sep = is_separator(c);
        }
    }

    pub fn insert_char(&mut self, at: usize, c: char) {
        self.contents.insert(at, c);
        self.update();
    }

    pub fn delete_char(&mut self, at: usize) {
        if at >= self.contents.len() { return }
        self.contents.remove(at);
        self.update();
    }

    pub fn append_string(&mut self, s: &str) {
        self.contents.push_str(s);
        self.update();
    }

    pub fn split_off(&mut self, at: usize) -> String {
        let remainder = util::safe_split_off(&mut self.contents, at);
        self.update();
        remainder
    }

    pub fn rendered_cursor_x(&self, cursor_x: usize) -> usize {
        self.contents.chars()
            .take(cursor_x)
            .fold(0, |acc, c| {
                if c == '\t' {
                    acc + KILO_TAB_STOP - (acc % KILO_TAB_STOP)
                } else {
                    acc + 1
                }
        })
    }

    pub fn raw_cursor_x(&self, rendered_x: usize) -> usize {
        self.contents.chars()
            .scan(0, |acc, c| {
                if c == '\t' {
                    *acc = *acc + KILO_TAB_STOP - (*acc % KILO_TAB_STOP)
                } else {
                    *acc += 1
                };
                Some(*acc)
            }).position(|rx| rx > rendered_x).unwrap()
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
