use crate::chess::{File, Piece, Rank, Square};
use enum_iterator::{all, cardinality};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    squares: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn new(starting_position: HashMap<Square, Piece>) -> Board {
        let mut board = Board {
            squares: vec![vec![None; cardinality::<File>()]; cardinality::<Rank>()],
        };
        for (i, file) in all::<File>().enumerate() {
            for (j, rank) in all::<Rank>().enumerate() {
                let square = Square::new(file, rank);
                if let Some(piece) = starting_position.get(&square) {
                    board.squares[i][j] = Some(piece.clone());
                }
            }
        }
        board
    }

    pub fn get(&self, square: Square) -> Option<&Piece> {
        self.squares[square.file_index][square.rank_index].as_ref()
    }

    // Check if a square index is without the bounds of the board
    pub fn is_valid_square(&self, file_index: usize, rank_index: usize) -> bool {
        rank_index < self.squares.len() && file_index < self.squares[rank_index].len()
    }

    pub fn validate_square_offset(
        &self,
        square: &Square,
        file_offset: i8,
        rank_offset: i8,
    ) -> Option<Square> {
        let file_index = square.file_index as i8 + file_offset;
        let rank_index = square.rank_index as i8 + rank_offset;
        if file_index < 0 || rank_index < 0 {
            return None;
        }
        if self.is_valid_square(file_index as usize, rank_index as usize) {
            Some(Square::from_index(file_index as usize, rank_index as usize))
        } else {
            None
        }
    }

    pub fn get_all_pieces(&self) -> Vec<(Square, Piece)> {
        let mut pieces = Vec::<(Square, Piece)>::new();
        for file in all::<File>() {
            for rank in all::<Rank>() {
                let square = Square::new(file, rank);
                if let Some(piece) = self.get(square) {
                    pieces.push((square, piece.clone()));
                }
            }
        }
        pieces
    }
}
