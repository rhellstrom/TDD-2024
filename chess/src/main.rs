use crate::board::Square;
use crate::chess_piece::Color::{Black, White};
use crate::chess_piece::Piece;
use crate::chess_piece::PieceType::{King, Pawn};

mod board;
mod chess_piece;

fn main() {
    let mut board = board::Chessboard::new();
    board.add_piece(Piece {
        piece_type: Pawn,
        color: Black,
        location: Square { x: 2, y: 1},
    });
    board.add_piece(Piece {
        piece_type: King,
        color: White,
        location: Square { x: 5, y: 5},
    });
    board.print_board();
}
