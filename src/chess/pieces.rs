use crate::chess::{move_, Board, Color, Square};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn from_char(c: char) -> Option<PieceType> {
        match c {
            'N' => Some(PieceType::Knight),
            'B' => Some(PieceType::Bishop),
            'R' => Some(PieceType::Rook),
            'Q' => Some(PieceType::Queen),
            'K' => Some(PieceType::King),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

// Could this be better?
// pub enum Piece2 {
//     Pawn(Color),
//     Knight(Color),
//     Bishop(Color),
//     Rook(Color),
//     Queen(Color),
//     King(Color),
// }

impl Piece {
    // pub fn get_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
    //     match self.piece_type {
    //         PieceType::Pawn => self.get_pawn_moves(board, from_square),
    //         PieceType::Knight => self.get_knight_moves(board, from_square),
    //         PieceType::Bishop => self.get_bishop_moves(board, from_square),
    //         PieceType::Rook => self.get_rook_moves(board, from_square),
    //         PieceType::Queen => self.get_queen_moves(board, from_square),
    //         PieceType::King => self.get_king_moves(board, from_square),
    //     }
    // }
    //
    fn get_pawn_moves(&self, board: &Board, from_square: &Square) -> Vec<MoveType> {
        let mut moves = Vec::new();
        // Get the direction the pawn can move in
        let forward = match self.color {
            Color::White => 1,
            Color::Black => -1,
        };

        // Check if we can push the pawn forwards
        let result = board.get_offset(from_square, 0, forward);
        if let Some((new_square, None)) = result {
            moves.push(MoveType::Normal {
                from: Some(Disambiguation::Square(*from_square)),
                to: new_square,
                piece: PieceType::Pawn,
            });
        }

        // Check if we can take a piece diagonally
        for file_offset in (-1..=1).step_by(2) {
            let result = board.get_offset(from_square, file_offset, forward);
            if let Some((new_square, Some(taken_piece))) = result {
                if taken_piece.color != self.color {
                    moves.push(MoveType::Normal {
                        from: Some(Disambiguation::Square(*from_square)),
                        to: new_square,
                        piece: PieceType::Pawn,
                    });
                }
            }
        }

        // moves = moves.iter().map(|m| m).collect();

        moves
    }
    // fn get_knight_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
    //     let mut moves = Vec::new();
    //     // The knight moves by checking every square in a 5x5 box centered on itself,
    //     // only moving to squares that are 2 squares away in one direction and 1 square away in the other
    //     for file_offset in -2i8..=2 {
    //         for rank_offset in -2i8..=2 {
    //             if file_offset == 0 || rank_offset == 0 || file_offset.abs() == rank_offset.abs() {
    //                 continue;
    //             }
    //             let result = board.get_offset(from_square, file_offset, rank_offset);
    //             if let Some((new_square, Some(taken_piece))) = result {
    //                 // If we hit an opponent's piece, we can take it
    //                 if taken_piece.color != self.color {
    //                     moves.push(move_!(*from_square, new_square));
    //                 }
    //             } else if let Some((new_square, None)) = result {
    //                 // Move into empty space
    //                 moves.push(move_!(*from_square, new_square));
    //             }
    //         }
    //     }
    //     moves
    // }
    //
    // // Get all moves for a piece that can move in straight lines,
    // // without the ability to move over pieces (e.g. bishop, rook, queen)
    // fn get_sliding_moves(
    //     &self,
    //     board: &Board,
    //     from_square: &Square,
    //     directions: [(i8, i8); 4],
    // ) -> Vec<Move> {
    //     let mut moves = Vec::new();
    //     for (file_direction, rank_direction) in directions.iter() {
    //         // Offsets represent the vector from the starting square
    //         let (mut file_offset, mut rank_offset) = (*file_direction, *rank_direction);
    //         // Sliding is true until we hit another piece or the edge of the board
    //         let mut sliding: bool = true;
    //         while sliding {
    //             let result = board.get_offset(from_square, file_offset, rank_offset);
    //
    //             if let Some((new_square, Some(taken_piece))) = result {
    //                 // If we hit another piece stop sliding,
    //                 // if that piece is the opposite color, we can take it
    //                 if taken_piece.color != self.color {
    //                     moves.push(move_!(*from_square, new_square));
    //                 }
    //                 sliding = false;
    //             } else if let Some((new_square, None)) = result {
    //                 // If we find an empty square, keep sliding
    //                 moves.push(move_!(*from_square, new_square));
    //                 file_offset += *file_direction;
    //                 rank_offset += *rank_direction;
    //             } else if result.is_none() {
    //                 // If we hit the edge of the board, stop sliding
    //                 sliding = false;
    //             }
    //         }
    //     }
    //     moves
    // }
    //
    // fn get_bishop_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
    //     const DIRECTIONS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    //     self.get_sliding_moves(board, from_square, DIRECTIONS)
    // }
    //
    // fn get_rook_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
    //     const DIRECTIONS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    //     self.get_sliding_moves(board, from_square, DIRECTIONS)
    // }
    //
    // fn get_queen_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
    //     [
    //         self.get_bishop_moves(board, from_square),
    //         self.get_rook_moves(board, from_square),
    //     ]
    //     .concat()
    // }
    //
    // fn get_king_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
    //     let mut moves = Vec::new();
    //     // Check every square in a 3x3 box around the king
    //     for file_offset in -1i8..=1 {
    //         for rank_offset in -1i8..=1 {
    //             if file_offset == 0 && rank_offset == 0 {
    //                 continue;
    //             }
    //
    //             let result = board.get_offset(from_square, file_offset, rank_offset);
    //             if let Some((new_square, Some(taken_piece))) = result {
    //                 // If we hit an opponent's piece, we can take it
    //                 if taken_piece.color != self.color {
    //                     moves.push(move_!(*from_square, new_square));
    //                 }
    //             } else if let Some((new_square, None)) = result {
    //                 // Move into empty space
    //                 moves.push(move_!(*from_square, new_square));
    //             }
    //         }
    //     }
    //     moves
    // }
}

#[macro_export]
macro_rules! piece {
    ($color:ident, $type:ident) => {
        Piece {
            color: Color::$color,
            piece_type: PieceType::$type,
        }
    };
}
pub(crate) use piece;

use super::{
    moves::{Disambiguation, MoveType},
    Move,
};
