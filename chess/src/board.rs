use crate::chess_piece::Color::*;
use crate::chess_piece::Piece;
use crate::chess_piece::PieceType::*;

const BOARD_SIZE: usize = 8;
const INITIAL_BOARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Square {
    pub x: usize,
    pub y: usize,
}

#[allow(dead_code)]
impl Square {
    pub fn algebraic_to_coords(algebraic: &str) -> Result<Self, &str> {
        if let Some(letter) = algebraic.chars().nth(0) {
            if let Some(number_char) = algebraic.chars().nth(1) {
                if !letter.is_ascii_alphabetic() || !('a'..='h').contains(&letter) {
                    return Err("Not a valid letter");
                }

                let number = match number_char.to_digit(10) {
                    Some(n) if (1..=8).contains(&n) => n as usize,
                    _ => return Err("Not a valid number"),
                };

                let x = (letter as usize) - ('a' as usize);
                let y = 8 - number;

                return Ok(Self { x, y });
            }
        }
        Err("Not valid input")
    }
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
        let board = Chessboard::from_fen(INITIAL_BOARD_FEN).unwrap();
        board
    }

    /// VERY simple FEN parsing function for initializing pieces on board.
    /// Does not support active color, en passant, castling, Half/full move
    pub fn from_fen(fen: &str) -> Result<Chessboard, &str>{
        let mut board = Chessboard::new();
        let mut row = 0; 
        let mut col = 0;

        for c in fen.chars() {
            match c {
                '/' => {
                    row += 1;
                    col = 0; 
                }
                '1'..='8' => {
                    col += c.to_digit(10).unwrap() as usize;
                }
                _ => {
                    let color = if c.is_lowercase() { Black } else { White };
                    let piece_type = match c.to_ascii_lowercase() {
                        'p' => Pawn,
                        'n' => Knight,
                        'b' => Bishop,
                        'r' => Rook,
                        'q' => Queen,
                        'k' => King,
                        _ => return Err("Invalid piece"), 
                    };

                    let piece = Piece {
                        piece_type,
                        color,
                        location: Square { y: row, x: col},
                    };
                    board.add_piece(piece);
                    col += 1; 
                }
            }
        }
        Ok(board)
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
                            Black => piece.piece_type.get_symbol(),
                            White => piece.piece_type.get_symbol(),
                        };
                        let color = match piece.color {
                            Black => {'B'}
                            White => {'W'}
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

    #[test]
    fn algebraic_to_coordinates() {
        let a1_square = Square { x: 0, y: 7 };
        let f3_square = Square { x: 5, y: 5 };
        let a8_square = Square { x: 0, y: 0 };
        let g8_square = Square { x: 6, y: 0 };
        assert_eq!(Square::algebraic_to_coords("a1"), Ok(a1_square));
        assert_eq!(Square::algebraic_to_coords("f3"), Ok(f3_square));
        assert_eq!(Square::algebraic_to_coords("a8"), Ok(a8_square));
        assert_eq!(Square::algebraic_to_coords("g8"), Ok(g8_square));
    }
    
    #[test]
    fn bad_algebraic_formatting() {
        assert_eq!(Square::algebraic_to_coords("A1"), Err("Not a valid letter"));
        assert_eq!(Square::algebraic_to_coords("I1"), Err("Not a valid letter"));
        assert_eq!(Square::algebraic_to_coords("a0"), Err("Not a valid number"));
        assert_eq!(Square::algebraic_to_coords("a9"), Err("Not a valid number"));
    }
    
}










