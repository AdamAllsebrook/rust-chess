use crate::chess::{Board, Piece, Square};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

pub struct Game {
    board: Board,
    turn: Color,
}

impl Game {
    pub fn new(starting_position: HashMap<Square, Piece>) -> Game {
        Game {
            board: Board::new(starting_position),
            turn: Color::White,
        }
    }

    // Get all possible moves for every piece for the current player,
    // ignoring the state of the board after the move
    pub fn get_all_possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::<Move>::new();
        for (square, piece) in self.board.get_all_pieces() {
            if piece.color == self.turn {
                moves.append(&mut piece.get_moves(&self.board, &square));
            }
        }
        moves
    }
}
