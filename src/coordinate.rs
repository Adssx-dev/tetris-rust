use std::ops;

// 90 degrees CCW
const ROTATION_MATRIX : &'static [f32] = &[0f32, -1f32, 1f32, 0f32];

#[derive(Clone, PartialEq, Eq)]
pub struct Coordinate {
    pub x : i32,
    pub y : i32
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
        let tmp = self - pivot.clone();
        let rotated = Coordinate {
            x : tmp.x * ROTATION_MATRIX[0] + tmp.y * ROTATION_MATRIX[1],
            y : tmp.x * ROTATION_MATRIX[2] + tmp.y * ROTATION_MATRIX[3]
        };
        rotated + pivot
    }
}


#[cfg(test)]
mod tests {
    use crate::coordinate::Coordinate;

    #[test]
    fn add() {
        let c1 = Coordinate {x:1,y:2};
        let c2 = Coordinate {x:3,y:4};
        let c3 = c1 + c2;
        assert_eq!(c3.x, 4);
        assert_eq!(c3.y, 6);
    }

    
    #[test]
    fn sub() {
        let c1 = Coordinate {x:1,y:2};
        let c2 = Coordinate {x:3,y:4};
        let c3 = c1 - c2;
        assert_eq!(c3.x, -2);
        assert_eq!(c3.y, -2);
    }

    
    
    #[test]
    fn rotation() {
        let c1 = Coordinate {x:1,y:2};
        let c2 = Coordinate {x:3,y:4};
        let c3 = c1.rotate(c2);
        assert_eq!(c3.x, 5);
        assert_eq!(c3.y, 2);
    }
}

