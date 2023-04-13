use crate::chess::{Board, Color, Move, Square};

#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn get_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        match self.piece_type {
            PieceType::Pawn => self.get_pawn_moves(board, from_square),
            PieceType::Knight => self.get_knight_moves(board, from_square),
            PieceType::Bishop => self.get_bishop_moves(board, from_square),
            PieceType::Rook => self.get_rook_moves(board, from_square),
            PieceType::Queen => self.get_queen_moves(board, from_square),
            PieceType::King => self.get_king_moves(board, from_square),
        }
    }

    fn get_pawn_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        let mut moves = Vec::new();
        // Get the direction the pawn can move in
        let forward = match self.color {
            Color::White => 1,
            Color::Black => -1,
        };

        // Check if we can push the pawn forwards
        let valid_square = board.validate_square_offset(from_square, 0, forward);
        if let Some(new_square) = valid_square {
            if board.get(new_square).is_none() {
                moves.push(Move {
                    from: from_square.clone(),
                    to: new_square,
                });
            }
        }

        // Check if we can take a piece diagonally
        for file_offset in (-1..=1).step_by(2) {
            let valid_square = board.validate_square_offset(from_square, file_offset, forward);
            if let Some(new_square) = valid_square {
                if let Some(taken_piece) = board.get(new_square) {
                    if taken_piece.color != self.color {
                        moves.push(Move {
                            from: new_square.clone(),
                            to: new_square,
                        });
                    }
                }
            }
        }
        moves
    }
    fn get_knight_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        let mut moves = Vec::new();
        // The knight moves by checking every square in a 5x5 box centered on itself,
        // only moving to squares that are 2 squares away in one direction and 1 square away in the other
        for file_offset in -2i8..=2 {
            for rank_offset in -2i8..=2 {
                if file_offset == 0 || rank_offset == 0 || file_offset.abs() == rank_offset.abs() {
                    continue;
                }
                if let Some(new_square) =
                    board.validate_square_offset(from_square, file_offset, rank_offset)
                {
                    // We cannot move to a square occupied by one of our own pieces
                    if let Some(taken_piece) = board.get(new_square) {
                        if taken_piece.color == self.color {
                            continue;
                        }
                    }
                    moves.push(Move {
                        from: from_square.clone(),
                        to: new_square,
                    });
                }
            }
        }
        moves
    }
    fn get_bishop_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        Vec::new()
    }
    fn get_rook_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        Vec::new()
    }
    fn get_queen_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        Vec::new()
    }
    fn get_king_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        Vec::new()
    }
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
