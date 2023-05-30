use std::rc::Rc;

use cursive::{
    theme::{
        BaseColor::*, BorderStyle, Color::*, ColorStyle, Palette, PaletteColor, PaletteColor::*,
        Style, Theme,
    },
    views::{Button, Dialog, EditView, FixedLayout, LinearLayout, TextView},
    Cursive, CursiveExt, Rect,
};

use crate::chess;

pub struct TerminalInterface {
    siv: Cursive,
}

impl TerminalInterface {
    pub fn new() -> Self {
        let mut siv = Cursive::new();
        siv.set_theme(generate_theme());
        Self { siv }
    }

    pub fn play_game(&mut self, game: Rc<chess::Game>) {
        let move_input = Dialog::new().title(format!("{} to move", game.turn));
        // .content(EditView::new().on_submit(|s, input| game.send_input(input)));
        let quit_button = Button::new("Quit", |s| s.quit());
        let board_layout = get_board_layout(&game);

        let layout = LinearLayout::vertical()
            .child(board_layout)
            .child(move_input)
            .child(quit_button);

        self.siv.add_layer(layout);
        self.siv.run();
    }
}

fn generate_theme() -> Theme {
    let mut palette = Palette::default();
    let colors = vec![
        (Background, Dark(Black)),
        (Highlight, Dark(Blue)),
        (HighlightInactive, Dark(Yellow)),
        (TitlePrimary, Dark(Black)),
    ];
    palette.extend(colors);
    Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette,
    }
}

fn get_board_layout(game: &Rc<chess::Game>) -> FixedLayout {
    let mut board_layout = FixedLayout::new();
    let light_square = Style::from(ColorStyle::new(
        PaletteColor::Primary,
        PaletteColor::HighlightInactive,
    ));
    let dark_square = Style::from(ColorStyle::new(
        PaletteColor::Primary,
        PaletteColor::Highlight,
    ));

    for (square, piece) in game.board.get_all_squares() {
        // double width to make squares
        let x = square.get_file_index() * 2;
        let y = if game.turn == chess::Color::White {
            game.board.get_height() - square.get_rank_index() - 1
        } else {
            square.get_rank_index()
        };
        let rect = Rect::from_size((x, y), (2, 1));

        let color = if square.is_light_square() {
            light_square
        } else {
            dark_square
        };

        let text = format!("{} ", get_piece_unicode(piece));
        board_layout.add_child(rect, TextView::new(text).style(color));
    }
    board_layout
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
