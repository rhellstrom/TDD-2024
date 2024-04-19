use crate::board::{Chessboard, Square};
use crate::chess_piece::Color::{Black, White};
use crate::chess_piece::Piece;

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

#[cfg(test)]
mod tests {
    use crate::board::{Chessboard, Square};
    use crate::chess_piece::Color::{Black, White};
    use crate::chess_piece::Piece;
    use crate::chess_piece::PieceType::Pawn;
    use crate::moves::pawn_movements;

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
}
