use std::cmp::Eq;
use core::fmt::Show;
use std::fmt::Formatter;
use std::fmt::Result;

#[cfg(test)]
mod coords_test;


pub struct Position2D{
    pub x: int,
    pub y: int,
}

impl Eq for Position2D{

}

impl PartialEq for Position2D{
    fn eq(&self, other: &Position2D) -> bool{
        self.x == other.x && self.y == other.y
    }
}

impl Show for Position2D{
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "position ({},{})", self.x, self.y)
    }
}
