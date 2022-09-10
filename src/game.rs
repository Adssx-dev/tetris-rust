use std::f32::consts::PI;

use crate::piece::Piece;


pub struct Game {
    possible_pieces : Vec<Piece>
}

impl Game {
    pub fn new () -> Game { 
        Game {
            possible_pieces : vec![ 
                Piece::piece_i(),
                Piece::piece_j(),
                Piece::piece_l(),
                Piece::piece_o(),
                Piece::piece_s(),
                Piece::piece_t(),
                Piece::piece_z(),
            ]
        }
    }
}