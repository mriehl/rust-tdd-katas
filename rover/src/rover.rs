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

    pub enum Delta{
        Position2D,
        XDelta(int),
        YDelta(int),
    }

    pub struct Rover{
        pub position: Position2D,
    }

    impl Rover{
        pub fn move(&mut self, delta: Delta){
            let new_position = match delta{
                YDelta(y) => Position2D{x: self.position.x, y: self.position.y + y},
                XDelta(x) => Position2D{x: self.position.x + x, y: self.position.y},
                _ => self.position
            };
            self.position = new_position;
        }
    }
}
