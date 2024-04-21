use crate::board::{Chessboard, Square};
use crate::moves::{bishop_movements, king_movements, knight_movements, pawn_movements, queen_movements, rook_movements};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]     
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub location: Square,
}

impl PieceMovement for Piece {
    fn can_move(&self, board: Chessboard) -> Vec<Square>{
        match self.piece_type {
            PieceType::Pawn => pawn_movements(board, *self),
            PieceType::Rook => rook_movements(board, *self),
            PieceType::Knight => knight_movements(board, *self), 
            PieceType::Bishop => bishop_movements(board, *self), 
            PieceType::Queen => queen_movements(board, *self),
            PieceType::King => king_movements(board, *self),
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


