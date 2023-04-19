use crate::chess;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::style::Color;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Cell, Row, Table},
    Frame, Terminal,
};
use std::{io, thread, time::Duration};

pub fn display(game: &chess::Game) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());

        draw_board(f, game, chunks[1]);
    })?;

    thread::sleep(Duration::from_millis(5000));
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

pub fn draw_board<B: Backend>(f: &mut Frame<B>, game: &chess::Game, area: ratatui::layout::Rect) {
    let size = 1;
    // idk why it has to be size * 2
    let widths = vec![Constraint::Length(size * 2); 8];

    let light_square = Style::default().bg(Color::Yellow);
    let dark_square = Style::default().bg(Color::Blue);
    let rows = (0..game.board.get_height()).map(|i| {
        Row::new((0..game.board.get_width()).map(|j| {
            Cell::from(String::from(get_piece_unicode(game.board.get(
                &chess::Square::from_index(
                    j,
                    if game.turn == chess::Color::White {
                        game.board.get_height() - i - 1
                    } else {
                        i
                    },
                ),
            ))))
            .style(if i & 1 == j & 1 {
                light_square
            } else {
                dark_square
            })
        }))
        .height(size)
    });

    let table = Table::new(rows)
        .style(Style::default().fg(Color::Black))
        .block(Block::default().title("Board").borders(Borders::ALL))
        .widths(&widths)
        .column_spacing(0);
    f.render_widget(table, area);
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
