use crate::chess::{move_, Board, Color, Move, Square};

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
                moves.push(move_!(from_square.clone(), new_square));
            }
        }

        // Check if we can take a piece diagonally
        for file_offset in (-1..=1).step_by(2) {
            let valid_square = board.validate_square_offset(from_square, file_offset, forward);
            if let Some(new_square) = valid_square {
                if let Some(taken_piece) = board.get(new_square) {
                    if taken_piece.color != self.color {
                        moves.push(move_!(from_square.clone(), new_square));
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
                    moves.push(move_!(from_square.clone(), new_square));
                }
            }
        }
        moves
    }

    // Get all moves for a piece that can move in straight lines,
    // without the ability to move over pieces (e.g. bishop, rook, queen)
    fn get_sliding_moves(
        &self,
        board: &Board,
        from_square: &Square,
        directions: [(i8, i8); 4],
    ) -> Vec<Move> {
        let mut moves = Vec::new();
        for (file_direction, rank_direction) in directions.iter() {
            // Offsets represent the vector from the starting square
            let (mut file_offset, mut rank_offset) = (*file_direction, *rank_direction);
            // Searching is true until we hit another piece or the edge of the board
            let mut searching: bool = true;
            while searching {
                if let Some(new_square) =
                    board.validate_square_offset(from_square, file_offset, rank_offset)
                {
                    // If we hit another piece stop searching,
                    // if that piece is the opposite color, we can take it
                    if let Some(taken_piece) = board.get(new_square) {
                        if taken_piece.color != self.color {
                            moves.push(move_!(from_square.clone(), new_square));
                        }
                        searching = false;
                    } else {
                        moves.push(move_!(from_square.clone(), new_square));
                        // Check the square one step further away in the next iteration
                        file_offset += *file_direction;
                        rank_offset += *rank_direction;
                    }
                } else {
                    searching = false;
                }
            }
        }
        moves
    }

    fn get_bishop_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        const DIRECTIONS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
        self.get_sliding_moves(board, from_square, DIRECTIONS)
    }

    fn get_rook_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        const DIRECTIONS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        self.get_sliding_moves(board, from_square, DIRECTIONS)
    }

    fn get_queen_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        [
            self.get_bishop_moves(board, from_square),
            self.get_rook_moves(board, from_square),
        ]
        .concat()
    }

    fn get_king_moves(&self, board: &Board, from_square: &Square) -> Vec<Move> {
        let mut moves = Vec::new();
        // Check every square in a 3x3 box around the king
        for file_offset in -1i8..=1 {
            for rank_offset in -1i8..=1 {
                if file_offset == 0 && rank_offset == 0 {
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
                    moves.push(move_!(from_square.clone(), new_square));
                }
            }
        }
        moves
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
