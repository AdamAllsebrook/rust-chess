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
                    board.squares[i][j] = Some(*piece);
                }
            }
        }
        board
    }

    pub fn get_width(&self) -> usize {
        self.squares[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.squares.len()
    }

    // Index is guaranteed to be in bounds due to the coupling between
    // the Square/File/Rank and the Board initialisation
    pub fn get(&self, square: &Square) -> Option<&Piece> {
        self.squares[square.get_file_index()][square.get_rank_index()].as_ref()
    }

    // Get the square and piece some given offset from a valid square
    // Validates that the new square is in bounds
    pub fn get_offset(
        &self,
        square: &Square,
        file_offset: i8,
        rank_offset: i8,
    ) -> Option<(Square, Option<&Piece>)> {
        let file_index = square.get_file_index() as i8 + file_offset;
        let rank_index = square.get_rank_index() as i8 + rank_offset;
        if file_index < 0 || rank_index < 0 {
            return None;
        }
        let square = self.validate_square(file_index as usize, rank_index as usize);
        square.map(|square| (square, self.get(&square)))
    }

    // Check if a square index is without the bounds of the board
    pub fn validate_square(&self, file_index: usize, rank_index: usize) -> Option<Square> {
        if rank_index < self.squares.len() && file_index < self.squares[rank_index].len() {
            Some(Square::from_index(file_index, rank_index))
        } else {
            None
        }
    }

    pub fn get_all_squares(&self) -> Vec<(Square, Option<&Piece>)> {
        let mut pieces = Vec::new();
        for file in all::<File>() {
            for rank in all::<Rank>() {
                let square = Square::new(file, rank);
                pieces.push((square, self.get(&square)));
            }
        }
        pieces
    }

    pub fn get_all_pieces(&self) -> Vec<(Square, &Piece)> {
        self.get_all_squares()
            .iter()
            .filter(|(_, piece)| piece.is_some())
            .map(|(square, piece)| (*square, piece.unwrap()))
            .collect()
    }
}
