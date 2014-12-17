#![desc = "A mars rover kata in rust"]
#![license = "public domain"]
#![feature(phase)]
#[cfg(test)] #[phase(plugin,link)] extern crate shiny;

extern crate core;
extern crate collections;

mod coords;
mod world;

#[cfg(test)]
mod rover_test;

mod rover{
    use coords::Position2D;
    use world;
    use world::World;

    pub fn new() -> Rover{
        Rover{
                position: Position2D{x: 0, y: 0},
                facing: Orientation::North,
                world: world::new(100, 100),
            }
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
        pub world: World,
    }

    impl Rover{
        pub fn move_(&mut self, delta: Delta){
            let new_position = match delta{
                Delta::YDelta(y) => Position2D{x: self.position.x, y: self.position.y + y},
                Delta::XDelta(x) => Position2D{x: self.position.x + x, y: self.position.y},
                Delta::Vector(x, y) => Position2D{x: self.position.x + x, y: self.position.y + y},
            };
            self.position = match self.world.recalculate_position_overflow(&new_position){
                Some(position) => position,
                None => self.position.clone(),
            }
        }

        pub fn advance(&mut self){
            let (dx, dy) = match self.facing{
                Orientation::North => (0, 1),
                Orientation::South => (0, -1),
                Orientation::East => (1, 0),
                Orientation::West => (-1, 0),
            };
            self.move_(Delta::Vector(dx, dy))
        }
    }
}
