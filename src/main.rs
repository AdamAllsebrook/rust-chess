use std::rc::Rc;

mod chess;
mod display;

fn main() {
    let game = Rc::new(chess::Game::new(chess::generate_starting_position()));
    let moves = game.get_all_possible_moves();
    println!("{}", display::get_board_string(&game));
}
