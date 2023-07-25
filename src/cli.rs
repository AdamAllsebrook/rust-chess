use crate::chess;
use std::{
    collections::HashMap,
    io::{self, Write},
};

enum Player {
    Human,
    Computer,
}

pub struct Cli {
    game: chess::Game,
    players: HashMap<chess::Color, Player>,
}

impl Cli {
    pub fn new() -> Cli {
        Cli {
            game: chess::Game::new(chess::generate_starting_position()),
            players: HashMap::from([
                (chess::Color::White, Player::Human),
                (chess::Color::Black, Player::Computer),
            ]),
        }
    }

    pub fn run(&self) {
        loop {
            match self.players.get(&self.game.turn) {
                Some(Player::Human) => {
                    let move_input = loop {
                        let raw_input = self.get_player_input();
                        let parsed_input = chess::Move::parse(&raw_input);
                        match parsed_input {
                            Ok(move_input) => break move_input,
                            Err(e) => println!("Input Error! {}", e),
                        }
                    };
                    self.game.do_move(move_input);
                }
                Some(Player::Computer) => (),
                None => panic!("No player found for color {}", self.game.turn),
            }
        }
    }

    fn get_player_input(&self) -> String {
        println!("{}\n", get_board_string(&self.game));
        print!("{} to move > ", self.game.turn);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input
    }
}

pub fn get_board_string(game: &chess::Game) -> String {
    let mut chars = vec![vec!['\u{2022}'; game.board.get_width()]; game.board.get_height()];
    for (square, piece) in game.board.get_all_pieces() {
        let x = square.get_file_index();
        let y = square.get_rank_index();
        chars[y][x] = get_piece_unicode(Some(piece));
    }
    let mut ranks = String::from("  ");
    for x in 0..game.board.get_width() {
        ranks.push_str(&format!(
            " {}",
            chess::File::from_index(x).unwrap().to_char()
        ))
    }
    let mut board_output = chars
        .iter()
        .map(|row| row.iter().map(|c| format!(" {c}")).collect::<String>())
        .enumerate()
        .map(|(i, row)| format!("{} {}\n", game.board.get_height() - i, row))
        .collect::<String>();
    board_output.push_str(&ranks);
    board_output
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
