pub enum Direction {
    Current,
    Pos(usize),
    Prev(usize),
    Next(usize),
}

impl Direction {
    pub(crate) fn resolve(&self, pos: usize) -> usize {
        match self {
            Self::Current => pos,
            Self::Pos(u) => u.clone(),
            Self::Next(u) => pos + u,
            Self::Prev(u) => pos - u,
        }
    }
}
