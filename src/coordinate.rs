use std::ops;

const rotation_matrix : &'static [i32] = &[0, -1, 1, 0];

pub struct Coordinate {
    x : i32,
    y : i32
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x : self.x + other.x,
            y : self.y + other.y,
        }
    }
}

impl ops::Sub<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn sub(self, other: Coordinate) -> Coordinate {
        Coordinate {
            x : self.x - other.x,
            y : self.y - other.y,
        }
    }
}

impl Coordinate {
    pub fn rotate(self, pivot : Coordinate) -> Coordinate {
        let tmp = self - pivot;
        Coordinate {
            x : tmp.x * rotation_matrix[0] + tmp.y * rotation_matrix[1],
            y : tmp.x * rotation_matrix[2] + tmp.y * rotation_matrix[3]
        }
    }
}

