use std::cmp::Eq;
use core::fmt::Show;
use std::hash::Hash;
use std::hash::sip::SipState;
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

impl Hash for Position2D {
    fn hash(&self, state: &mut SipState) {
        (self.x, self.y).hash(state);
    }
}
