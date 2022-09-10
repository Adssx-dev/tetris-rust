use crate::{coordinate::Coordinate, color::Color, direction::Direction, board::Board};

pub struct Square {
    position : Coordinate,
    color : Color,
}

impl Square {
    pub fn new(x : i32, y : i32, color : Color) -> Square {
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
            position : Coordinate {x: 1, y:1},
            color : Color {r:0, g:0, b:0}
        };

        assert!(s.calculate_new_position(Direction::Left) ==  Coordinate {x:0, y:1});
        assert!(s.calculate_new_position(Direction::Right) ==  Coordinate {x:2, y:1});
        assert!(s.calculate_new_position(Direction::Down) ==  Coordinate {x:1, y:2});
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
            position : Coordinate {x: 1, y:0},
            color : Color {r:0, g:0, b:0}
        };

        assert!(!s.can_move_direction(Direction::Left, &b));
        assert!(!s.can_move_direction(Direction::Right, &b));
        assert!(!s.can_move_direction(Direction::Down, &b));
        
        let s2 = Square {
            position : Coordinate {x: 2, y:2},
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
            position : Coordinate {x: 0, y:1},
            color : Color {r:0, g:0, b:0}
        };

        s.move_square(Direction::Down, &b);
        assert!(s.position ==  Coordinate {x:0, y:2});

        s.move_square(Direction::Right, &b);
        assert!(s.position ==  Coordinate {x:1, y:2});

        s.move_square(Direction::Left, &b);
        assert!(s.position ==  Coordinate {x:0, y:2});
    }
}