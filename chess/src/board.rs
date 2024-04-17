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
    pub board: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
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

#[cfg(test)]
mod tests {
    use crate::board::{Chessboard, Square};
    use crate::chess_piece::{Color, PieceType};

    fn assert_piece_at_square(chessboard: &Chessboard, square: Square, piece_type: PieceType, color: Color) {
        if let Some(piece) = chessboard.get_piece_at(square) {
            assert_eq!(piece.piece_type, piece_type);
            assert_eq!(piece.color, color);
        } else {
            panic!("Piece not found at square {:?}", square);
        }
    }
    
    #[test]
    fn initialized_pawns() {
        let board = Chessboard::new_with_pieces();
        for index in 0..7 {
            let white_pawn_square = Square {x: index, y: 6};
            let black_pawn_square = Square {x: index, y: 1};

            if let Some(white_pawn) = board.get_piece_at(white_pawn_square){
                assert_eq!(white_pawn.color, Color::White)
            } else {
                panic!("No piece present at {:?}", white_pawn_square);
            }

            if let Some(black_pawn) = board.get_piece_at(black_pawn_square){
                assert_eq!(black_pawn.color,Color::Black)
            } else {
                panic!("No piece present at {:?}", black_pawn_square);
            }
        }
    }
    
    #[test]
    fn initialized_rooks() {
        let chessboard = Chessboard::new_with_pieces();

        assert_piece_at_square(&chessboard, Square { x: 0, y: 0 }, PieceType::Rook, Color::Black);
        assert_piece_at_square(&chessboard, Square { x: 7, y: 0 }, PieceType::Rook, Color::Black);
        assert_piece_at_square(&chessboard, Square { x: 0, y: 7 }, PieceType::Rook, Color::White);
        assert_piece_at_square(&chessboard, Square { x: 7, y: 7 }, PieceType::Rook, Color::White);
    }

    #[test]
    fn initialized_knights() {
        let chessboard = Chessboard::new_with_pieces();

        assert_piece_at_square(&chessboard, Square { x: 1, y: 0 }, PieceType::Knight, Color::Black);
        assert_piece_at_square(&chessboard, Square { x: 6, y: 0 }, PieceType::Knight, Color::Black);
        assert_piece_at_square(&chessboard, Square { x: 1, y: 7 }, PieceType::Knight, Color::White);
        assert_piece_at_square(&chessboard, Square { x: 6, y: 7 }, PieceType::Knight, Color::White);
    }

    #[test]
    fn initialized_bishops() {
        let chessboard = Chessboard::new_with_pieces();

        assert_piece_at_square(&chessboard, Square { x: 2, y: 0 }, PieceType::Bishop, Color::Black);
        assert_piece_at_square(&chessboard, Square { x: 5, y: 0 }, PieceType::Bishop, Color::Black);
        assert_piece_at_square(&chessboard, Square { x: 2, y: 7 }, PieceType::Bishop, Color::White);
        assert_piece_at_square(&chessboard, Square { x: 5, y: 7 }, PieceType::Bishop, Color::White);
    }

    #[test]
    fn initialized_kings() {
        let chessboard = Chessboard::new_with_pieces();

        assert_piece_at_square(&chessboard, Square { x: 4, y: 0 }, PieceType::King, Color::Black);
        assert_piece_at_square(&chessboard, Square { x: 4, y: 7 }, PieceType::King, Color::White);
    }

    #[test]
    fn initialized_queens() {
        let chessboard = Chessboard::new_with_pieces();

        assert_piece_at_square(&chessboard, Square { x: 3, y: 0 }, PieceType::Queen, Color::Black);
        assert_piece_at_square(&chessboard, Square { x: 3, y: 7 }, PieceType::Queen, Color::White);
    }
}










