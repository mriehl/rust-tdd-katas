#![desc = "A mars rover kata in rust"]
#![license = "public domain"]

extern crate core;

mod coords;

#[cfg(test)]
mod rover_test;

mod rover{
    use coords::Position2D;

    pub fn new() -> Rover{
        Rover{position: Position2D{x: 0, y: 0}}
    }

    pub struct Rover{
        pub position: Position2D,
    }
}
