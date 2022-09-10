use crate::board::Board;
use crate::color::Color;
use crate::direction::Direction;
use crate::square::Square;
use crate::coordinate::Coordinate;

pub struct Piece {
    squares : Vec<Square>,
    pivot_coordinates : Coordinate,
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
                Square::new(0.0, 0.0, color.clone()),
                Square::new(1.0, 0.0, color.clone()),
                Square::new(2.0, 0.0, color.clone()),
                Square::new(3.0, 0.0, color.clone()),
            ],
            pivot_coordinates : Coordinate { x: 2.0, y: 1.0 },
            position : Coordinate { x: 0.0, y: 0.0 }
        }
    }
    

    pub fn piece_o() -> Piece {
        let color = Color {r:255, g:255,b:0};
        Piece {
            squares : vec![
                Square::new(0.0, 0.0, color.clone()),
                Square::new(0.0, 1.0, color.clone()),
                Square::new(1.0, 0.0, color.clone()),
                Square::new(1.0, 1.0, color.clone()),
            ],
            pivot_coordinates : Coordinate { x: 1.0, y: 1.0 },
            position : Coordinate { x: 0.0, y: 0.0 }
        }
    }

    pub fn piece_t() -> Piece {
        let color = Color {r:0, g:0,b:255};
        Piece {
            squares : vec![
                Square::new(0.0, 1.0, color.clone()),
                Square::new(1.0, 1.0, color.clone()),
                Square::new(2.0, 1.0, color.clone()),
                Square::new(1.0, 0.0, color.clone()),
            ],
            pivot_coordinates : Coordinate { x: 1.5, y: 1.5 },
            position : Coordinate { x: 0.0, y: 0.0 }
        }
    }

    pub fn piece_l() -> Piece {
        let color = Color {r:255, g:0,b:0};
        Piece {
            squares : vec![
                Square::new(0.0, 1.0, color.clone()),
                Square::new(1.0, 1.0, color.clone()),
                Square::new(2.0, 1.0, color.clone()),
                Square::new(2.0, 0.0, color.clone()),
            ],
            pivot_coordinates : Coordinate { x: 1.5, y: 1.5 },
            position : Coordinate { x: 0.0, y: 0.0 }
        }
    }

    pub fn piece_j() -> Piece {
        let color = Color {r:0, g:255,b:0};
        Piece {
            squares : vec![
                Square::new(0.0, 0.0, color.clone()),
                Square::new(0.0, 1.0, color.clone()),
                Square::new(1.0, 1.0, color.clone()),
                Square::new(2.0, 1.0, color.clone()),
            ],
            pivot_coordinates : Coordinate { x: 1.5, y: 1.5 },
            position : Coordinate { x: 0.0, y: 0.0 }
        }
    }

    pub fn piece_z() -> Piece {
        let color = Color {r:255, g:255,b:255};
        Piece {
            squares : vec![
                Square::new(0.0, 0.0, color.clone()),
                Square::new(1.0, 0.0, color.clone()),
                Square::new(1.0, 1.0, color.clone()),
                Square::new(2.0, 1.0, color.clone()),
            ],
            pivot_coordinates : Coordinate { x: 1.5, y: 1.5 },
            position : Coordinate { x: 0.0, y: 0.0 }
        }
    }

    pub fn piece_s() -> Piece {
        let color = Color {r:0, g:255,b:255};
        Piece {
            squares : vec![
                Square::new(0.0, 1.0, color.clone()),
                Square::new(1.0, 1.0, color.clone()),
                Square::new(1.0, 0.0, color.clone()),
                Square::new(2.0, 0.0, color.clone()),
            ],
            pivot_coordinates : Coordinate { x: 1.5, y: 1.5 },
            position : Coordinate { x: 0.0, y: 0.0 }
        }
    }

    fn whole_piece_can_move(&self, direction : Direction, board : &Board) -> bool {
        self.squares.iter().all(|s| s.can_move_direction(direction.clone(), board))
    }
}