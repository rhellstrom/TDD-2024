use crate::board::{Chessboard, Square};
use crate::chess_piece::Color::{Black, White};
use crate::chess_piece::Piece;

#[allow(dead_code)]
pub fn pawn_movements(board: Chessboard, piece: Piece) -> Vec<Square>{
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
pub fn rook_movements(board: Chessboard, piece: Piece) -> Vec<Square> {
    let mut possible_moves: Vec<Square>= vec![];
    for (r, c) in &[(1,0), (-1, 0), (0, 1), (0, -1)] {
        let mut row = piece.location.y as i8 + r;
        let mut col = piece.location.x as i8 + c;

        while (0..8).contains(&(row as usize)) && (0..8).contains(&(col as usize)) {
            let location = board.board[row as usize][col as usize];
            if location.is_none() {
                possible_moves.push(Square { y: row as usize, x: col as usize });
            } else {
                if let Some(obstacle_piece) = location {
                    if obstacle_piece.color != piece.color {
                        possible_moves.push(obstacle_piece.location);
                    }
                }
                break; 
            }
            row += r;
            col += c;
        }
    }
    possible_moves
}

#[cfg(test)]
mod tests {
    use crate::board::{Chessboard, Square};
    use crate::chess_piece::Color::{Black, White};
    use crate::chess_piece::{Piece, PieceMovement};
    use crate::chess_piece::PieceType::*;
    use crate::moves::{pawn_movements, rook_movements};

    // test_moves(BOARD_FEN, Piece to test in algebraic, destination(s?) in algebraic)
// test_moves(FEN_STRING, "e2", vec of all possible destinations?)
    fn test_moves(fen_state: &str, piece_pos_alg: &str, possible_moves: Vec<&str>){
        let board = Chessboard::from_fen(fen_state).unwrap();
        let test_piece_location = Square::algebraic_to_coords(piece_pos_alg).unwrap();
        let test_piece = board.get_piece_at(test_piece_location).unwrap();
        let mut moves_to_test: Vec<Square> = vec![];
        for square in possible_moves {
           moves_to_test.push(Square::algebraic_to_coords(square).unwrap()) 
        }
        
        let possible_moves = test_piece.can_move(board);
        assert!(moves_to_test.iter().all(|moves| possible_moves.contains(moves)));
        
    }
    #[test]
    fn moving_pawn_white_first_move_no_captures(){
        test_moves("8/8/8/8/8/8/4P3/8", "e2", vec!["e3", "e4"])
    }
    
    #[test]
    fn moving_pawn_black_second_move_with_captures(){
       test_moves("8/8/4p3/3P1P2/8/8/8/8", "e6", vec!["d5", "e5", "f5"])
    }
    
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
        let board = Chessboard::from_fen("8/8/8/8/8/8/4P3/8").unwrap();
        let pawn = board.get_piece_at(Square::algebraic_to_coords("e2").unwrap()).unwrap();
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
