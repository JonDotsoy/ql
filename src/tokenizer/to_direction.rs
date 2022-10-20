use super::direction::Direction;

pub trait ToDirection {
    fn resolve(self, pos: usize) -> usize;
}

impl ToDirection for Direction {
    fn resolve(self, pos: usize) -> usize {
        Direction::resolve(&self, pos)
    }
}

impl ToDirection for i32 {
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
