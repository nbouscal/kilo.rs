use editor::row::Highlight;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

pub struct SearchState {
    pub last_match: Option<usize>,
    pub saved_highlight: Vec<Highlight>,
    pub direction: Direction,
}

impl SearchState {
    pub fn new() -> Self {
        SearchState {
            last_match: None,
            saved_highlight: Vec::new(),
            direction: Direction::Forward,
        }
    }
}
