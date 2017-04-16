#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Forward,
    Backward,
}

pub struct SearchState {
    pub last_match: Option<usize>,
    pub direction: Direction,
}

impl SearchState {
    pub fn new() -> Self {
        SearchState {
            last_match: None,
            direction: Direction::Forward,
        }
    }
}