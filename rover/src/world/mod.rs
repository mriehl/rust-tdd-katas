use coords::Position2D;
use std::collections::HashMap;

#[cfg(test)]
mod world_test;

pub struct World{
    pub width: int,
    pub height: int,
    occupants: HashMap<Position2D, String>,
}

fn overflowed_coord(bound: int, wanted: int) -> int{
    match (bound, wanted){
        (bound, negative) if negative < 0 => bound + negative + 1,
        (bound, lower_eq) if lower_eq <= bound => lower_eq,
        (bound, higher) => higher % bound - 1,
    }
}

pub fn new(width: int, height: int) -> World{
    World{width: width, height: height, occupants: HashMap::new()}
}

impl World{

    pub fn recalculate_position_overflow(&self, pos: &Position2D) -> Option<Position2D>{
        let actual_x = overflowed_coord(self.width, pos.x);
        let actual_y = overflowed_coord(self.height, pos.y);
        let actual_position = Position2D{x: actual_x, y: actual_y};

        if self.occupants.contains_key(&actual_position){
            None
        } else {
            Some(actual_position)
        }
    }

    pub fn insert(&mut self, name: &str, pos: Position2D){
        self.occupants.insert(pos, String::from_str(name));
    }
}

