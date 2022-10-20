use super::direction::Direction;

pub trait AtDirection {
    fn resolve(self, pos: usize) -> usize;
}

impl AtDirection for Direction {
    fn resolve(self, pos: usize) -> usize {
        Direction::resolve(&self, pos)
    }
}

impl AtDirection for i32 {
    fn resolve(self, pos: usize) -> usize {
        if self == 0 {
            Direction::Current.resolve(pos)
        } else if self < 0 {
            Direction::Prev(self.abs() as usize).resolve(pos)
        } else {
            Direction::Next(self as usize).resolve(pos)
        }
    }
}
