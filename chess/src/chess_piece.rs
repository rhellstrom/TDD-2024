use crate::board::{Chessboard, Square};
use crate::chess_piece::Color::{Black, White};
use crate::moves::{pawn_movements, rook_movements};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]     
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub location: Square,
}

impl Color {
    pub fn opposites(self, other_piece: Color) -> bool {
        matches!((self, other_piece), (Black, White) | (White, Black))
    }
}

impl Piece {
    /// Initializes a new piece with its square given as a string in algebraic notation e.g "e2"
    pub fn new(piece_type: PieceType, color: Color, square: &str) -> Result<Piece, &str> {
        match Square::algebraic_to_coords(square) {
            Ok(location) => {
                Ok(Piece {
                    piece_type,
                    color,
                    location,
                })
            }
                Err(e) => Err(e) 
            }
        }
}

impl PieceMovement for Piece {
    fn can_move(&self, board: Chessboard) -> Vec<Square>{
        match self.piece_type {
            PieceType::Pawn => pawn_movements(board, *self),
            PieceType::Rook => rook_movements(board, *self),
            PieceType::Knight => {unimplemented!()}
            PieceType::Bishop => {unimplemented!()}
            PieceType::Queen => {unimplemented!()}
            PieceType::King => {unimplemented!()}
        }
    }
}

pub trait PieceMovement {
    fn can_move(&self, board: Chessboard) -> Vec<Square>;
}

impl PieceType {
    pub fn get_symbol(&self) -> &str {
        match *self {
            PieceType::Pawn => "P",
            PieceType::Rook => "R",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        }
    }
}


