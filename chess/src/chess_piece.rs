use crate::board::Square;

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

trait PieceMovement {
    fn can_move(&self, from: Square, to: Square) -> bool;
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

impl PieceMovement for PieceType {
    fn can_move(&self, _from: Square, _to: Square) -> bool {
        match *self {
            PieceType::Pawn => {unimplemented!()}
            PieceType::Rook => {unimplemented!()}
            PieceType::Knight => {unimplemented!()}
            PieceType::Bishop => {unimplemented!()}
            PieceType::Queen => {unimplemented!()}
            PieceType::King => {unimplemented!()}
        }
    }
}
