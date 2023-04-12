use crate::chess::Color;

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
