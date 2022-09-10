use crate::board::Board;
use crate::color::Color;
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

    pub fn piece_i() -> Piece {
        let color = Color {r:255, g:0,b:255};
        Piece {
            squares : vec![
                Square::new(0, 0, color.clone()),
                Square::new(0, 1, color.clone()),
                Square::new(0, 2, color.clone()),
                Square::new(0, 3, color.clone()),
            ],
            pivot_index:2,
            position : Coordinate { x: 0, y: 0 }
        }
    }
    

    pub fn piece_o() -> Piece {
        let color = Color {r:255, g:255,b:0};
        Piece {
            squares : vec![
                Square::new(0, 0, color.clone()),
                Square::new(0, 1, color.clone()),
                Square::new(1, 0, color.clone()),
                Square::new(1, 1, color.clone()),
            ],
            pivot_index:2,
            position : Coordinate { x: 0, y: 0 }
        }
    }

    pub fn piece_t() -> Piece {
        let color = Color {r:0, g:0,b:255};
        Piece {
            squares : vec![
                Square::new(0, 0, color.clone()),
                Square::new(0, 1, color.clone()),
                Square::new(0, 2, color.clone()),
                Square::new(1, 1, color.clone()),
            ],
            pivot_index:2,
            position : Coordinate { x: 0, y: 0 }
        }
    }

    pub fn piece_l() -> Piece {
        let color = Color {r:255, g:0,b:0};
        Piece {
            squares : vec![
                Square::new(0, 0, color.clone()),
                Square::new(0, 1, color.clone()),
                Square::new(0, 2, color.clone()),
                Square::new(1, 2, color.clone()),
            ],
            pivot_index:2,
            position : Coordinate { x: 0, y: 0 }
        }
    }

    pub fn piece_j() -> Piece {
        let color = Color {r:0, g:255,b:0};
        Piece {
            squares : vec![
                Square::new(0, 0, color.clone()),
                Square::new(0, 1, color.clone()),
                Square::new(0, 2, color.clone()),
                Square::new(1, 0, color.clone()),
            ],
            pivot_index:2,
            position : Coordinate { x: 0, y: 0 }
        }
    }

    pub fn piece_z() -> Piece {
        let color = Color {r:255, g:255,b:255};
        Piece {
            squares : vec![
                Square::new(1, 0, color.clone()),
                Square::new(1, 1, color.clone()),
                Square::new(0, 1, color.clone()),
                Square::new(0, 2, color.clone()),
            ],
            pivot_index:2,
            position : Coordinate { x: 0, y: 0 }
        }
    }

    pub fn piece_s() -> Piece {
        let color = Color {r:0, g:255,b:255};
        Piece {
            squares : vec![
                Square::new(0, 0, color.clone()),
                Square::new(0, 1, color.clone()),
                Square::new(1, 1, color.clone()),
                Square::new(2, 1, color.clone()),
            ],
            pivot_index:2,
            position : Coordinate { x: 0, y: 0 }
        }
    }

    fn whole_piece_can_move(&self, direction : Direction, board : &Board) -> bool {
        self.squares.iter().all(|s| s.can_move_direction(direction.clone(), board))
    }
}