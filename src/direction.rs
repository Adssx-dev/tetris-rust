use crate::coordinate::Coordinate;

#[derive(Clone)]
pub enum Direction {
    Down,
    Left, 
    Right
}

impl Direction {
    pub fn to_coordinates(self : Direction) -> Coordinate {
        match self {
            Direction::Left => Coordinate{x:-1f32, y:0f32},
            Direction::Right => Coordinate{x:1f32, y:0f32},
            Direction::Down => Coordinate{x:0f32, y:1f32},
        }
    }
}

