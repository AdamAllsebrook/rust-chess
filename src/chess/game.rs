use crate::chess::{Board, Piece, PieceType, Square};
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

    // Get all possible moves for a piece, ignoring the state of the board after the move
    fn get_possible_moves(&self, square: &Square, piece: &Piece) -> Vec<Move> {
        let mut moves = Vec::<Move>::new();
        match piece.piece_type {
            PieceType::Pawn => {
                // Get the direction the pawn can move in
                let forward = match piece.color {
                    Color::White => 1,
                    Color::Black => -1,
                };

                // Check if we can push the pawn forwards
                let valid_square = self.board.validate_square_offset(&square, 0, forward);
                if let Some(new_square) = valid_square {
                    if self.board.get(new_square).is_none() {
                        moves.push(Move {
                            from: square.clone(),
                            to: new_square,
                        });
                    }
                }

                // Check if we can take a piece diagonally
                for file_offset in (-1..=1).step_by(2) {
                    let valid_square =
                        self.board
                            .validate_square_offset(&square, file_offset, forward);
                    if let Some(new_square) = valid_square {
                        if let Some(taken_piece) = self.board.get(new_square) {
                            if taken_piece.color != piece.color {
                                moves.push(Move {
                                    from: square.clone(),
                                    to: new_square,
                                });
                            }
                        }
                    }
                }
            }
            _ => (),
        }
        moves
    }

    // Get all possible moves for every piece for the current player,
    // ignoring the state of the board after the move
    pub fn get_all_possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::<Move>::new();
        for (square, piece) in self.board.get_all_pieces() {
            if piece.color == self.turn {
                moves.append(&mut self.get_possible_moves(&square, &piece));
            }
        }
        moves
    }
}
