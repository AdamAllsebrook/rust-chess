use enum_iterator::{all, cardinality, Sequence};
use std::collections::HashMap;
use std::fmt;

fn main() {
    let board = Board::new(generate_starting_position());
    let moves = board.get_all_possible_moves();
    for move_ in moves {
        println!("{} to {}", move_.from, move_.to);
    }
}

#[derive(Debug)]
struct Board {
    pieces: Vec<Vec<Option<Piece>>>,
    turn: Color,
}

impl Board {
    fn new(starting_position: HashMap<Square, Piece>) -> Board {
        let mut board = Board {
            pieces: vec![vec![None; cardinality::<File>()]; cardinality::<Rank>()],
            turn: Color::White,
        };
        for (i, file) in all::<File>().enumerate() {
            for (j, rank) in all::<Rank>().enumerate() {
                let square = Square::new(file, rank);
                if let Some(piece) = starting_position.get(&square) {
                    board.pieces[i][j] = Some(piece.clone());
                }
            }
        }
        board
    }

    // Check if a square index is without the bounds of the board
    fn is_valid_square(&self, file_index: usize, rank_index: usize) -> bool {
        rank_index < self.pieces.len() && file_index < self.pieces[rank_index].len()
    }

    fn validate_square_offset(
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

    // Check if a piece exists and is the color of the current turn
    fn is_movable_piece(&self, piece: &Option<Piece>) -> bool {
        if let Some(piece) = piece {
            piece.color == self.turn
        } else {
            false
        }
    }

    // Get all possible moves for a piece, ignoring the state of the board after the move
    fn get_possible_moves(&self, piece: &Piece, file_index: usize, rank_index: usize) -> Vec<Move> {
        let mut moves = Vec::<Move>::new();
        let square = Square::from_index(file_index, rank_index);
        match piece.piece_type {
            PieceType::Pawn => {
                // Get the direction the pawn can move in
                let forward = match piece.color {
                    Color::White => 1,
                    Color::Black => -1,
                };

                // Check if we can push the pawn forwards
                let valid_square = self.validate_square_offset(&square, 0, forward);
                if let Some(new_square) = valid_square {
                    if self.pieces[new_square.file_index][new_square.rank_index].is_none() {
                        moves.push(Move {
                            from: square,
                            to: new_square,
                        });
                    }
                }
            }
            _ => (),
        }
        moves
    }

    // Get all possible moves for every piece for the current player,
    // ignoring the state of the board after the move
    fn get_all_possible_moves(&self) -> Vec<Move> {
        let mut moves = Vec::<Move>::new();
        for file_index in 0..self.pieces.len() {
            for rank_index in 0..self.pieces[file_index].len() {
                if self.is_movable_piece(&self.pieces[file_index][rank_index]) {
                    let piece = self.pieces[file_index][rank_index].unwrap();
                    moves.append(&mut self.get_possible_moves(&piece, file_index, rank_index))
                }
            }
        }
        moves
    }
}

#[derive(Debug)]
struct Move {
    from: Square,
    to: Square,
}

#[derive(Debug, Copy, Clone)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
struct Piece {
    piece_type: PieceType,
    color: Color,
}

#[derive(Debug, Sequence, Hash, Eq, PartialEq, Copy, Clone)]
enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Sequence, Hash, Eq, PartialEq, Copy, Clone)]
enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            all::<Rank>().position(|r| r == *self).unwrap() + 1
        )
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Square {
    rank: Rank,
    file: File,
    file_index: usize,
    rank_index: usize,
}

impl Square {
    fn new(file: File, rank: Rank) -> Square {
        Square {
            file,
            rank,
            file_index: all::<File>().position(|f| f == file).unwrap(),
            rank_index: all::<Rank>().position(|r| r == rank).unwrap(),
        }
    }
    fn from_index(file_index: usize, rank_index: usize) -> Square {
        if file_index >= cardinality::<File>() || rank_index >= cardinality::<Rank>() {
            panic!("Tried to create a Square with an out of bounds index")
        }
        return Square {
            file: all::<File>().collect::<Vec<_>>()[file_index],
            rank: all::<Rank>().collect::<Vec<_>>()[rank_index],
            file_index,
            rank_index,
        };
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

#[macro_export]
macro_rules! square {
    ($file:ident, $rank_num:literal) => {
        Square::new(
            File::$file,
            match ($rank_num) {
                1 => Rank::One,
                2 => Rank::Two,
                3 => Rank::Three,
                4 => Rank::Four,
                5 => Rank::Five,
                6 => Rank::Six,
                7 => Rank::Seven,
                8 => Rank::Eight,
                _ => panic!("Invalid rank number"),
            },
        )
    };
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

fn generate_starting_position() -> HashMap<Square, Piece> {
    return HashMap::from([
        (square!(A, 1), piece!(White, Rook)),
        (square!(B, 1), piece!(White, Knight)),
        (square!(C, 1), piece!(White, Bishop)),
        (square!(D, 1), piece!(White, Queen)),
        (square!(E, 1), piece!(White, King)),
        (square!(F, 1), piece!(White, Bishop)),
        (square!(G, 1), piece!(White, Knight)),
        (square!(H, 1), piece!(White, Rook)),
        (square!(A, 2), piece!(White, Pawn)),
        (square!(B, 2), piece!(White, Pawn)),
        (square!(C, 2), piece!(White, Pawn)),
        (square!(D, 2), piece!(White, Pawn)),
        (square!(E, 2), piece!(White, Pawn)),
        (square!(F, 2), piece!(White, Pawn)),
        (square!(G, 2), piece!(White, Pawn)),
        (square!(H, 2), piece!(White, Pawn)),
        (square!(A, 8), piece!(Black, Rook)),
        (square!(B, 8), piece!(Black, Knight)),
        (square!(C, 8), piece!(Black, Bishop)),
        (square!(D, 8), piece!(Black, Queen)),
        (square!(E, 8), piece!(Black, King)),
        (square!(F, 8), piece!(Black, Bishop)),
        (square!(G, 8), piece!(Black, Knight)),
        (square!(H, 8), piece!(Black, Rook)),
        (square!(A, 7), piece!(Black, Pawn)),
        (square!(B, 7), piece!(Black, Pawn)),
        (square!(C, 7), piece!(Black, Pawn)),
        (square!(D, 7), piece!(Black, Pawn)),
        (square!(E, 7), piece!(Black, Pawn)),
        (square!(F, 7), piece!(Black, Pawn)),
        (square!(G, 7), piece!(Black, Pawn)),
        (square!(H, 7), piece!(Black, Pawn)),
    ]);
}
