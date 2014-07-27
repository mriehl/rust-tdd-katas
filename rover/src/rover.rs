#![desc = "A mars rover kata in rust"]
#![license = "public domain"]

extern crate core;

mod coords;
mod world;

#[cfg(test)]
mod rover_test;

mod rover{
    use coords::Position2D;

    pub fn new() -> Rover{
        Rover{position: Position2D{x: 0, y: 0}, facing: North}
    }

    pub enum Delta{
        Vector(int, int),
        XDelta(int),
        YDelta(int),
    }

    pub enum Orientation{
        North,
        South,
        East,
        West,
    }

    pub struct Rover{
        pub position: Position2D,
        pub facing: Orientation,
    }

    impl Rover{
        pub fn move(&mut self, delta: Delta){
            let new_position = match delta{
                YDelta(y) => Position2D{x: self.position.x, y: self.position.y + y},
                XDelta(x) => Position2D{x: self.position.x + x, y: self.position.y},
                Vector(x, y) => Position2D{x: self.position.x + x, y: self.position.y + y},
            };
            self.position = new_position;
        }

        pub fn advance(&mut self){
            let (dx, dy) = match self.facing{
                North => (0, 1),
                South => (0, -1),
                East => (1, 0),
                West => (-1, 0),
            };
            self.move(Vector(dx, dy))
        }
    }
}
