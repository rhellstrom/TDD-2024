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
                Err(e) => Err(e) //TODO: Handle me
            }
        }
    fn can_move(&self, destination: Square) -> bool {
        self.piece_type.can_move(self.location, destination)
    }
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

