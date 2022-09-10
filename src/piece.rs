use crate::board::Board;
use crate::direction::Direction;
use crate::square::Square;
use crate::coordinate::Coordinate;

pub struct Piece {
    squares : Vec<Square>,
    pivot_index : usize,
    position : Coordinate,
}


impl Piece {
    pub fn move_piece(&mut self, direction : Direction, board : &Board) {
        if self.whole_piece_can_move(direction.clone(), board) {
            for p in &mut self.squares {
                p.move_square(direction.clone(), board);
            }
        }
    }

    fn whole_piece_can_move(&self, direction : Direction, board : &Board) -> bool {
        self.squares.iter().all(|s| s.can_move_direction(direction.clone(), board))
    }
}