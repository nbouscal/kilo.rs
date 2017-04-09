use std::iter;

const KILO_TAB_STOP: usize = 8;

pub struct Row {
    pub contents: String,
    pub render: String,
}

impl Row {
    pub fn from_string(s: String) -> Self {
        Row {
            contents: s.clone(),
            render: Self::render_string(s),
        }
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
