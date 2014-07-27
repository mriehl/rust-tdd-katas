use coords::Position2D;

#[cfg(test)]
mod world_test;

pub struct World{
    pub width: int,
    pub height: int,
}

impl World{
    pub fn recalculate_position_overflow(&self, pos: &Position2D) -> Position2D{
        let actual_x = match (self.width, pos.x){
            (x, y) if x > y => y,
            (x, y) if x <= y => x,
            _ => pos.x
        };

        let actual_y = match (self.height, pos.y){
            (x, y) if x > y => y,
            (x, y) if x <= y => x,
            _ => pos.y
        };
        Position2D{x: actual_x, y: actual_y}
    }
}
