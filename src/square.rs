use crate::{coordinate::Coordinate, color::Color, direction::Direction, board::Board};

pub struct Square {
    position : Coordinate,
    color : Color,
}

impl Square {
    pub fn new(x : f32, y : f32, color : Color) -> Square {
        Square {position : Coordinate {x:x, y:y}, color : color.clone()}
    }

    pub fn move_square(&mut self, direction : Direction, board : &Board) {
        let new_position = self.position.clone() + direction.to_coordinates();

        if !self.can_move_position(new_position.clone(), board) {
            panic!("Cannot move to the specified location, cell is not empty or destination is outside the board");
        }

        self.position = new_position;
    }
    
    pub fn can_move_direction(&self, direction : Direction, board : &Board) -> bool {
        let new_position = self.calculate_new_position(direction);
        self.can_move_position(new_position, board)
    }

    pub fn rotate(&mut self, pivot : Coordinate, board : &Board) {
        if self.can_rotate(pivot.clone(), board) {
            self.position = self.get_position_after_rotation(pivot);
        }
    }

    pub fn can_rotate(&self, pivot : Coordinate, board : &Board) -> bool {
        let new_position = self.get_position_after_rotation(pivot);
        self.can_move_position(new_position, board)
    }
    

    fn get_position_after_rotation (&self, pivot : Coordinate) -> Coordinate {
        let mut new_coordinates = Coordinate {
            x : self.position.x + 0.5,
            y : self.position.y + 0.5,
        };

        new_coordinates = new_coordinates.rotate(pivot);
        
        new_coordinates.x -= 0.5;
        new_coordinates.y -= 0.5;
        new_coordinates
    }
    
    fn can_move_position(&self, new_position : Coordinate, board : &Board) -> bool {
        match board.is_cell_empty(new_position.y as usize, new_position.x as usize) {
            Ok(res) => res,
            Err(_) => false
        }
    }

    fn calculate_new_position(&self, direction : Direction) -> Coordinate {
        self.position.clone() + direction.to_coordinates()
    }
}


#[cfg(test)]
mod tests {
    use crate::{board::{Board, Cell}, color::Color, coordinate::Coordinate, direction::Direction};

    use super::Square;

    #[test]
    fn calculate_new_position() {
        let s = Square {
            position : Coordinate {x: 1f32, y:1f32},
            color : Color {r:0, g:0, b:0}
        };

        assert!(s.calculate_new_position(Direction::Left) ==  Coordinate {x:0f32, y:1f32});
        assert!(s.calculate_new_position(Direction::Right) ==  Coordinate {x:2f32, y:1f32});
        assert!(s.calculate_new_position(Direction::Down) ==  Coordinate {x:1f32, y:2f32});
    }

    #[test]
    fn can_move_direction() {
        let red = Color{r:255, g:0, b:0};

        // Should draw this pattern (R = red, G = green, B = blue, E = empty) 
        // RER
        // ERE
        // EEE
        let mut b = Board::new(3, 3);
        b.set_cell(Cell::Full(red.clone()), 0, 0);
        b.set_cell(Cell::Full(red.clone()), 0, 2);
        b.set_cell(Cell::Full(red.clone()), 1, 1);

        let s = Square {
            position : Coordinate {x: 1f32, y:0f32},
            color : Color {r:0, g:0, b:0}
        };

        assert!(!s.can_move_direction(Direction::Left, &b));
        assert!(!s.can_move_direction(Direction::Right, &b));
        assert!(!s.can_move_direction(Direction::Down, &b));
        
        let s2 = Square {
            position : Coordinate {x: 2f32, y:2f32},
            color : Color {r:0, g:0, b:0}
        };

        assert!(s2.can_move_direction(Direction::Left, &b));
        assert!(!s2.can_move_direction(Direction::Right, &b));
        assert!(!s2.can_move_direction(Direction::Down, &b));
    }

    #[test]
    fn move_square() {
        let red = Color{r:255, g:0, b:0};

        // Should draw this pattern (R = red, G = green, B = blue, E = empty) 
        // RER
        // ERE
        // EEE
        let mut b = Board::new(3, 3);
        b.set_cell(Cell::Full(red.clone()), 0, 0);
        b.set_cell(Cell::Full(red.clone()), 0, 2);
        b.set_cell(Cell::Full(red.clone()), 1, 1);

        let mut s = Square {
            position : Coordinate {x: 0f32, y:1f32},
            color : Color {r:0, g:0, b:0}
        };

        s.move_square(Direction::Down, &b);
        assert!(s.position ==  Coordinate {x:0f32, y:2f32});

        s.move_square(Direction::Right, &b);
        assert!(s.position ==  Coordinate {x:1f32, y:2f32});

        s.move_square(Direction::Left, &b);
        assert!(s.position ==  Coordinate {x:0f32, y:2f32});
    }
}