use coords::Position2D;

#[cfg(test)]
mod world_test;

pub struct World{
    pub width: int,
    pub height: int,
}

fn overflowed_coord(bound: int, wanted: int) -> int{
    match (bound, wanted){
        (bound, lower_eq) if lower_eq <= bound => lower_eq,
        (bound, higher) => higher % bound,
    }
}

impl World{
    pub fn recalculate_position_overflow(&self, pos: &Position2D) -> Position2D{
        let actual_x = overflowed_coord(self.width, pos.x);

        let actual_y = overflowed_coord(self.height, pos.y);
        Position2D{x: actual_x, y: actual_y}
    }
}
