use std::collections::HashMap;

pub mod board;
pub mod game;
pub mod pieces;
pub mod squares;

pub use board::Board;
pub use game::{Color, Game, Move};
use pieces::piece;
pub use pieces::{Piece, PieceType};
use squares::square;
pub use squares::{File, Rank, Square};

pub fn generate_starting_position() -> HashMap<Square, Piece> {
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
