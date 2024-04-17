use crate::chess_piece::{Color, Piece};

const BOARD_SIZE: usize = 8;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

#[allow(dead_code)]
pub struct Chessboard {
    board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
}

#[allow(dead_code)]
impl Chessboard {
    pub fn new() -> Chessboard {
        Chessboard{
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn get_piece_at(&self, square: Square) -> Option<Piece> {
        self.board[square.y][square.x]
    }

    pub fn add_piece(&mut self, piece: Piece) {
        let piece_location = piece.location;
        self.board[piece_location.y][piece_location.x] = Some(piece);
    }

    pub fn remove_piece(&mut self,square: Square) {
        self.board[square.y][square.x] = None;
    }
    
    pub fn print_board(&self) {
        let mut result = String::new();

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                match self.get_piece_at(Square { x, y }) {
                    Some(piece) => {
                        let symbol = match piece.color {
                            Color::Black => piece.piece_type.get_symbol(),
                            Color::White => piece.piece_type.get_symbol(),
                        };
                        let color = match piece.color {
                            Color::Black => {'B'}
                            Color::White => {'W'}
                        };
                        result.push_str(&format!("{}-{}", color, symbol));
                    }
                    None => {
                        result.push_str(" * ");
                    }
                }

                if x < BOARD_SIZE - 1 {
                    result.push_str("  ");
                }
            }
            result.push('\n');

            if y < BOARD_SIZE - 1 {
                result.push('\n');
            }
        }
        println!("{}",result);
    }
}
