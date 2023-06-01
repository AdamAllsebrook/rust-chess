use crate::chess;

pub fn get_board_string(game: &chess::Game) -> String {
    let mut chars = vec![vec!['\u{2022}'; game.board.get_width() + 1]; game.board.get_height()];
    for (square, piece) in game.board.get_all_pieces() {
        let x = square.get_file_index();
        let y = square.get_rank_index();
        chars[y][x] = get_piece_unicode(Some(piece));
    }
    for y in 0..game.board.get_height() {
        chars[y][game.board.get_width()] = '\n';
    }
    chars
        .iter()
        .map(|row| row.iter().map(|c| format!(" {c}")).collect::<String>())
        .collect::<String>()
}

const fn get_piece_unicode(piece: Option<&chess::Piece>) -> char {
    match piece {
        None => ' ',
        Some(piece) => match piece.color {
            chess::Color::White => match piece.piece_type {
                chess::PieceType::Pawn => '\u{2659}',
                chess::PieceType::Knight => '\u{2658}',
                chess::PieceType::Bishop => '\u{2657}',
                chess::PieceType::Rook => '\u{2656}',
                chess::PieceType::Queen => '\u{2655}',
                chess::PieceType::King => '\u{2654}',
            },
            chess::Color::Black => match piece.piece_type {
                chess::PieceType::Pawn => '\u{265F}',
                chess::PieceType::Knight => '\u{265E}',
                chess::PieceType::Bishop => '\u{265D}',
                chess::PieceType::Rook => '\u{265C}',
                chess::PieceType::Queen => '\u{265B}',
                chess::PieceType::King => '\u{265A}',
            },
        },
    }
}
