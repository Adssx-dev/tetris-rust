use std::ops;

// 90 degrees CCW
const ROTATION_MATRIX : &'static [f32] = &[0f32, -1f32, 1f32, 0f32];

#[derive(Clone, PartialEq)]
pub struct Coordinate {
    pub x : f32,
    pub y : f32
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
        let c1 = Coordinate {x:1f32,y:2f32};
        let c2 = Coordinate {x:3f32,y:4f32};
        let c3 = c1 + c2;
        assert_eq!(c3.x, 4f32);
        assert_eq!(c3.y, 6f32);
    }

    
    #[test]
    fn sub() {
        let c1 = Coordinate {x:1f32,y:2f32};
        let c2 = Coordinate {x:3f32,y:4f32};
        let c3 = c1 - c2;
        assert_eq!(c3.x, -2f32);
        assert_eq!(c3.y, -2f32);
    }

    
    
    #[test]
    fn rotation() {
        let c1 = Coordinate {x:1f32,y:2f32};
        let c2 = Coordinate {x:3f32,y:4f32};
        let c3 = c1.rotate(c2);
        assert_eq!(c3.x, 5f32);
        assert_eq!(c3.y, 2f32);
    }
}

