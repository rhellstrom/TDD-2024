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
    use crate::chess_piece::PieceMovement;
    
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
    fn moving_pawn_no_possible_moves(){
        test_moves("8/8/8/8/8/4P3/4P3/8", "e2", vec![])
    }
    #[test]
    fn moving_rook_allowed_directions_without_obstacles(){
        let expected_moves: Vec<&str> = vec![
            "a4", "b4", "c4", "d4", "f4", "g4", "h4", "e1", "e2", "e3", "e5", "e6", "e7", "e8"
        ];
        test_moves("8/8/8/8/4R3/8/8/8", "e4", expected_moves);
    }

    #[test]
    fn moving_rook_surrounded_by_allies(){
        test_moves("8/8/8/4P3/3PRP2/4P3/8/8", "e4", vec![]);
    }

    #[test]
    fn moving_rook_surrounded_by_captures_one_square_away(){
        let expected_moves: Vec<&str> = vec![
            "c4", "d4", "f4", "g4", "e6", "e5", "e3", "e2"
        ];
        test_moves("8/8/4P3/8/2P1r1P1/8/4P3/8", "e4", expected_moves);
    }

    #[test]
    fn moving_knight_without_obstacles(){
        let expected_moves: Vec<&str> = vec![
            "c5", "c3", "d6", "d2", "f6", "f2", "g5", "g3"
        ];
        test_moves("8/8/8/8/4N3/8/8/8", "e4", expected_moves);
    }

    #[test]
    fn moving_knight_without_possible_moves(){
        test_moves("8/8/3P1P2/2P3P1/4N3/2P3P1/3P1P2/8", "e4", vec![]);
    }

    #[test]
    fn moving_knight_with_captures_everywhere(){
        let expected_moves: Vec<&str> = vec![
            "c5", "c3", "d6", "d2", "f6", "f2", "g5", "g3"
        ];
        test_moves("8/8/3P1P2/2P3P1/4n3/2P3P1/3P1P2/8", "e4", expected_moves);
    }

    #[test]
    fn moving_bishop_allowed_directions_without_obstacles(){
        let expected_moves: Vec<&str> = vec![
            "a8", "b7", "c6", "d5", "f3", "g2", "h1", "f5", "g3", "h7", "d3", "c2", "b1"
        ];
        test_moves("8/8/8/8/4B3/8/8/8", "e4", expected_moves);
    }
    #[test]
    fn moving_bishop_surrounded_by_allies(){
        test_moves("8/8/8/3P1P2/4B3/3P1P2/8/8", "e4", vec![]);
    }
    #[test]
    fn moving_bishop_surrounded_by_captures_two_square_away(){
        let expected_moves: Vec<&str> = vec![
            "c6", "d5", "f5", "g6", "f3", "g2", "d3", "c2"
        ];
        test_moves("8/8/2P3P1/8/4b3/8/2P3P1/8", "e4", expected_moves);
    }

    #[test]
    fn moving_queen_all_possible_directions_empty_board(){
        let expected_moves: Vec<&str> = vec![
            "a8", "b7", "c6", "d5", "f3", "g2", "h1", "f5", "g3", "h7", "d3", "c2", "b1",
            "a4", "b4", "c4", "d4", "f4", "g4", "h4", "e1", "e2", "e3", "e5", "e6", "e7", "e8"
        ];
        test_moves("8/8/8/8/4Q3/8/8/8", "e4", expected_moves);
    }

    #[test]
    fn moving_queen_surrounded_by_allies(){
        test_moves("8/8/8/3PPP2/3PQP2/3PPP2/8/8", "e4", vec![]);
    }

    #[test]
    fn moving_queen_surrounded_by_captures_two_squares_away_all_directions(){
        let expected_moves: Vec<&str> = vec![
            "c6", "e6", "g6", "d5", "e5", "f5", "c4", "d4", "f4", "g4",
            "d3", "e3", "f3", "c2", "e2", "g2"
        ];
        test_moves("8/8/2P1P1P1/8/2P1q1P1/8/2P1P1P1/8", "e4", expected_moves);
    }

    #[test]
    fn moving_king_all_possible_directions_without_obstacles(){
        let expected_moves: Vec<&str> = vec![
            "d5", "e5", "f5", "d4", "f4", "d3", "e3", "f3"
        ];
        test_moves("8/8/8/8/4K3/8/8/8", "e4", expected_moves);
    }
    
    #[test]
    fn moving_king_surrounded_by_allies(){
        test_moves("8/8/8/3PPP2/3PKP2/3PPP2/8/8", "e4", vec![]);
    }
    
    #[test]
    fn moving_king_surrounded_by_captures(){
        let expected_moves: Vec<&str> = vec![
            "d5", "e5", "f5", "d4", "f4", "d3", "e3", "f3"
        ];
        test_moves("8/8/8/3PPP2/3PkP2/3PPP2/8/8", "e4", expected_moves);
    }

}
