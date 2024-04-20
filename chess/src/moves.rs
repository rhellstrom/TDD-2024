use crate::board::{Chessboard, Square};
use crate::chess_piece::Color::{Black, White};
use crate::chess_piece::Piece;
#[allow(dead_code)]
/// Returns a vector of possible moves for that piece on the board 
fn pawn_movements(board: Chessboard, piece: Piece) -> Vec<Square>{
    let row= piece.location.y;
    let col = piece.location.x;
    let mut possible_moves: Vec<Square> = vec![]; 
    match piece.color{
        Black => {
            let right_capture = board.board[row + 1][col + 1];
            let left_capture= board.board[row + 1][col - 1];
            let forward_once = board.board[row + 1][col];
            let forward_twice = board.board[row + 2][col];
            
            if let Some(right_cap) = right_capture{
                if right_cap.color == White {
                    possible_moves.push(right_cap.location);
                }
            }
            if let Some(left_cap) = left_capture {
                if left_cap.color == White {
                    possible_moves.push(left_cap.location);
                }
            }
            if forward_once.is_none(){
                possible_moves.push(Square {y: row + 1, x: col});
            }
            if forward_twice.is_none() && piece.location.y == 1 {
                possible_moves.push( Square { y: row + 2, x: col});
            }
        }
        White => {
            let left_capture = board.board[row - 1][col -1];
            let right_capture = board.board[row - 1][col + 1];
            let forward_once = board.board[row - 1][col];
            let forward_twice = board.board[row - 2][col];
            if let Some(left_cap) = left_capture {
                if left_cap.color == Black {
                    possible_moves.push(left_cap.location);
                }
            }
            if let Some(right_cap) = right_capture {
                if right_cap.color == Black {
                    possible_moves.push(right_cap.location);
                }
            }
            if forward_once.is_none() {
                possible_moves.push(Square { y: row - 1, x: col});
            }

            if forward_twice.is_none() && piece.location.y == 6 {
                possible_moves.push(Square { y: row -2, x: col});
            }
        }
    }
    possible_moves
}

#[allow(dead_code)]
fn rook_movements(board: Chessboard, piece: Piece) -> Vec<Square> {
    let mut possible_moves: Vec<Square> = vec![];
    let row = piece.location.y;
    let col = piece.location.x;

    for x in (col + 1)..8 {
        if let Some(occupied_pos) = board.board[row][x] {
            if occupied_pos.color != piece.color {
                possible_moves.push(Square { y: row, x });
            }
            break;
        }
        possible_moves.push(Square { y: row, x });
    }

    for x in (0..col).rev() {
        if let Some(occupied_pos) = board.board[row][x] {
            if occupied_pos.color != piece.color {
                possible_moves.push(Square { y: row, x });
            }
            break;
        }
        possible_moves.push(Square { y: row, x });
    }

    for y in (0..row).rev() {
        if let Some(occupied_pos) = board.board[y][col] {
            if occupied_pos.color != piece.color {
                possible_moves.push(Square { y, x: col });
            }
            break;
        }
        possible_moves.push(Square { y, x: col });
    }

    for y in (row + 1)..8 {
        if let Some(occupied_pos) = board.board[y][col] {
            if occupied_pos.color != piece.color {
                possible_moves.push(Square { y, x: col });
            }
            break;
        }
        possible_moves.push(Square { y, x: col });
    }
    possible_moves
}

#[cfg(test)]
mod tests {
    use crate::board::{Chessboard, Square};
    use crate::chess_piece::Color::{Black, White};
    use crate::chess_piece::Piece;
    use crate::chess_piece::PieceType::*;
    use crate::moves::{pawn_movements, rook_movements};

    #[test]
    fn moving_white_pawn_backward() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e2").unwrap();
        board.add_piece(pawn);
        let destination_square = Square::algebraic_to_coords("e1").unwrap();
        assert!(!pawn_movements(board, pawn).contains(&destination_square));
    }
    
    #[test]
    fn moving_white_pawn_forward() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e2").unwrap();
        board.add_piece(pawn);
        let destination_square = Square::algebraic_to_coords("e3").unwrap();
        assert!(pawn_movements(board, pawn).contains(&destination_square));
    }

    #[test]
    fn moving_black_pawn_forward() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, Black, "e7").unwrap();
        board.add_piece(pawn);
        let destination_square = Square::algebraic_to_coords("e6").unwrap();
        assert!(pawn_movements(board, pawn).contains(&destination_square));
    }
    #[test]
    fn moving_pawn_diagonally_to_square_occupied_by_opponent() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e2").unwrap();
        let opponent_pawn = Piece::new(Pawn, Black, "d3").unwrap();
        board.add_piece(pawn);
        board.add_piece(opponent_pawn);
        let destination_square = Square::algebraic_to_coords("d3").unwrap();
        assert!(pawn_movements(board, pawn).contains(&destination_square));
    }
    
    #[test]
    fn moving_pawn_diagonally_without_opponent() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e2").unwrap();
        board.add_piece(pawn);
        let destination_square = Square::algebraic_to_coords("d3").unwrap();
        assert!(!pawn_movements(board, pawn).contains(&destination_square));
    }
    
    #[test]
    fn moving_pawn_diagonally_to_square_occupied_by_ally() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e2").unwrap();
        let allied_pawn = Piece::new(Pawn, White, "d3").unwrap();
        board.add_piece(pawn);
        board.add_piece(allied_pawn);
        let destination_square = Square::algebraic_to_coords("d3").unwrap();
        assert!(!pawn_movements(board, pawn).contains(&destination_square));
    }
    
    #[test]
    fn moving_black_pawn_two_squares_first_move(){
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, Black, "e7").unwrap();
        board.add_piece(pawn);
        let destination_square = Square::algebraic_to_coords("e5").unwrap();
        assert!(pawn_movements(board, pawn).contains(&destination_square));
    }
    
    #[test]
    fn moving_rook_sideways_across_entire_board(){
        let mut board = Chessboard::new();
        let rook = Piece::new(Rook, Black, "a4").unwrap();
        board.add_piece(rook);
        let destination_square = Square::algebraic_to_coords("h4").unwrap();
        assert!(rook_movements(board, rook).contains(&destination_square));
    }

    #[test]
    fn moving_rook_diagonally(){
        let mut board = Chessboard::new();
        let rook = Piece::new(Rook, Black, "e4").unwrap();
        board.add_piece(rook);
        let destination_square = Square::algebraic_to_coords("a8").unwrap();
        assert!(!rook_movements(board, rook).contains(&destination_square));
    }

    #[test]
    fn moving_rook_over_piece() {
        let mut board = Chessboard::new();
        let rook = Piece::new(Rook, Black, "e4").unwrap();
        let allied_piece = Piece::new(Pawn, Black, "e6").unwrap();
        board.add_piece(rook);
        board.add_piece(allied_piece);
        let destination_square = Square::algebraic_to_coords("e8").unwrap();
        assert!(!rook_movements(board, rook).contains(&destination_square));
    }

    #[test]
    fn moving_rook_upwards_and_capturing_opponent(){
        let mut board = Chessboard::new();
        let rook = Piece::new(Rook, Black, "e4").unwrap();
        let opponent_piece= Piece::new(Pawn, White, "e6").unwrap();
        board.add_piece(rook);
        board.add_piece(opponent_piece);
        let destination_square = Square::algebraic_to_coords("e6").unwrap();
        assert!(rook_movements(board, rook).contains(&destination_square));
    }

    #[test]
    fn moving_rook_and_capturing_ally(){
        let mut board = Chessboard::new();
        let rook = Piece::new(Rook, Black, "e4").unwrap();
        let allied_piece = Piece::new(Pawn, Black, "e6").unwrap();
        board.add_piece(rook);
        board.add_piece(allied_piece);
        let destination_square = Square::algebraic_to_coords("e6").unwrap();
        assert!(!rook_movements(board, rook).contains(&destination_square));
    }
    #[test]
    fn moving_rook_upwards_across_entire_board(){
        let mut board = Chessboard::new();
        let rook = Piece::new(Rook, Black, "e1").unwrap();
        board.add_piece(rook);
        let destination_square = Square::algebraic_to_coords("e8").unwrap();
        assert!(rook_movements(board, rook).contains(&destination_square));
    }
    #[test]
    fn moving_rook_downwards_across_entire_board(){
        let mut board = Chessboard::new();
        let rook = Piece::new(Rook, Black, "e8").unwrap();
        board.add_piece(rook);
        let destination_square = Square::algebraic_to_coords("e1").unwrap();
        assert!(rook_movements(board, rook).contains(&destination_square));
    }
}
