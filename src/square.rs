use crate::{coordinate::Coordinate, color::Color, direction::Direction, board::Board};

pub struct Square {
    position : Coordinate,
    color : Color,
}

impl Square {
    pub fn move_square(&mut self, direction : Direction, board : &Board) {
        let new_position = self.position.clone() + direction.to_coordinates();

        if !self.can_move_position(new_position.clone(), board) {
            panic!("Cannot move to the specified location, cell is not empty");
        }

        self.position = new_position;
    }

    pub fn can_move_position(&self, new_position : Coordinate, board : &Board) -> bool {
        board.is_cell_empty(new_position.x as usize, new_position.y as usize)
    }

    pub fn can_move_direction(&self, direction : Direction, board : &Board) -> bool {
        let new_position = self.calculate_new_position(direction);
        self.can_move_position(new_position, board)
    }

    fn calculate_new_position(&self, direction : Direction) -> Coordinate {
        self.position.clone() + direction.to_coordinates()
    }
}