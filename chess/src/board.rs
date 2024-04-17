use crate::chess_piece::{Color, Piece, PieceType};
use crate::chess_piece::PieceType::*;

const BOARD_SIZE: usize = 8;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub x: usize,
    pub y: usize,
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
    pub fn new_with_pieces() -> Chessboard {
        let mut board = Chessboard::new();
        board.initialize_pieces();
        board
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
    
    fn initialize_pieces(&mut self) {
        self.with_mirrored_piece(Pawn, &[0, 1, 2, 3, 4, 5, 6, 7], 1);
        self.with_mirrored_piece(Rook, &[0, 7], 0);
        self.with_mirrored_piece(Knight, &[1, 6], 0);
        self.with_mirrored_piece(Bishop, &[2, 5], 0);
        self.with_mirrored_piece(Queen, &[3], 0);
        self.with_mirrored_piece(King, &[4], 0);
    }
    
    fn with_mirrored_piece(
        &mut self, 
        piece_type: PieceType, 
        x_coordinates: &[usize], 
        y_coordinate: usize) {
        for &x in x_coordinates {
            let black_piece = Piece {
                piece_type,
                color: Color::Black,
                location: Square { x, y: y_coordinate },
            };
            let white_piece = Piece {
                piece_type,
                color: Color::White,
                location: Square {
                    x,
                    y: BOARD_SIZE - 1 - y_coordinate,
                },
            };
            self.add_piece(black_piece);
            self.add_piece(white_piece);
        }
    }
}
