use enum_iterator::{all, Sequence};
use std::collections::HashMap;

fn main() {
    let board = Board::new(generate_starting_position());
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Hash, Eq, PartialEq)]
struct Square {
    file: File,
    rank: Rank,
}

struct Board {
    pieces: HashMap<Square, Option<Piece>>,
}

impl Board {
    fn new(starting_position: HashMap<Square, Piece>) -> Board {
        let mut board = Board {
            pieces: HashMap::new(),
        };
        for file in all::<File>() {
            for rank in all::<Rank>() {
                let square = Square { file, rank };
                if let Some(piece) = starting_position.get(&square) {
                    board.pieces.insert(square, Some(piece.clone()));
                } else {
                    board.pieces.insert(square, None);
                }
            }
        }
        board
    }
}

#[macro_export]
macro_rules! square {
    ($file:ident, $rank_num:literal) => {
        Square {
            file: File::$file,
            rank: match ($rank_num) {
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
        }
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
