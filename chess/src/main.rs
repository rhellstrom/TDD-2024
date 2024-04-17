use crate::board::Chessboard;

mod board;
mod chess_piece;

fn main() {
    let board = Chessboard::new_with_pieces();
    board.print_board();
}
