use crate::board::{Chessboard, Square};
use crate::chess_piece::Piece;

fn pawn_movement(board: Chessboard, piece: Piece, destination: Square) -> Result<(), String> {
    // Check color of piece to decide what direction is permitted
    // First move can do 2 squares
    // If there are opponents diagonally they can also consume in that direction
    
    // White pawns move up 
    // Black move down
    Ok(())
}

// A piece can only consume the opposite color. A function might come in handy 
// Initialize board from FEN to set up tests easier? Worth the effort? 
#[cfg(test)]
mod tests {
    use crate::board::{Chessboard, Square};
    use crate::chess_piece::Color::{Black, White};
    use crate::chess_piece::Piece;
    use crate::chess_piece::PieceType::Pawn;
    use crate::moves::pawn_movement;

    #[test]
    fn moving_white_pawn_backward() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e2").unwrap();
        board.add_piece(pawn);
        let destination_square = Square::algebraic_to_coords("e1").unwrap();
        assert!(pawn_movement(board, pawn, destination_square).is_err());
    }
    
    #[test]
    fn moving_white_pawn_forward() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e2").unwrap();
        board.add_piece(pawn);
        let destination_square = Square::algebraic_to_coords("e3").unwrap();
        assert!(pawn_movement(board, pawn, destination_square).is_ok());
    }

    #[test]
    fn moving_black_pawn_forward() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e7").unwrap();
        board.add_piece(pawn);
        let destination_square = Square::algebraic_to_coords("e6").unwrap();
        assert!(pawn_movement(board, pawn, destination_square).is_ok());
    }
    #[test]
    fn moving_pawn_diagonally_with_opponent() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e2").unwrap();
        let opponent_pawn = Piece::new(Pawn, Black, "d3").unwrap();
        board.add_piece(pawn);
        board.add_piece(opponent_pawn);
        let destination_square = Square::algebraic_to_coords("d3").unwrap();
        assert!(pawn_movement(board, pawn, destination_square).is_ok());
    }
    
    #[test]
    fn moving_pawn_diagonally_without_opponent() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e2").unwrap();
        board.add_piece(pawn);
        let destination_square = Square::algebraic_to_coords("d3").unwrap();
        assert!(pawn_movement(board, pawn, destination_square).is_err());
    }
    
    #[test]
    fn moving_pawn_diagonally_with_square_occupied_by_ally() {
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e2").unwrap();
        let allied_pawn = Piece::new(Pawn, White, "d3").unwrap();
        board.add_piece(pawn);
        board.add_piece(allied_pawn);
        let destination_square = Square::algebraic_to_coords("d3").unwrap();
        assert!(pawn_movement(board, pawn, destination_square).is_err());
    }
    
    #[test]
    fn moving_black_pawn_two_squares_first_first_move(){
        let mut board = Chessboard::new();
        let pawn = Piece::new(Pawn, White, "e7").unwrap();
        board.add_piece(pawn);
        let destination_square = Square::algebraic_to_coords("e5").unwrap();
        assert!(pawn_movement(board, pawn, destination_square).is_ok());
    }
}
