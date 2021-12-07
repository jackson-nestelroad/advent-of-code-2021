use num::Integer;

#[derive(PartialEq, Eq, Hash)]
pub struct Point<T: Integer + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Integer + Copy> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}
