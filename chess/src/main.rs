use crate::board::Chessboard;

mod board;
mod chess_piece;
mod moves;

fn main() {
    let board = Chessboard::new_with_pieces();
    board.print_board();
}
