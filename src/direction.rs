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
            Direction::Left => Coordinate{x:-1, y:0},
            Direction::Right => Coordinate{x:1, y:0},
            Direction::Down => Coordinate{x:0, y:1},
        }
    }
}

