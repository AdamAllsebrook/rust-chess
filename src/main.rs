use std::rc::Rc;

mod chess;
mod display;

fn main() {
    let game = Rc::new(chess::Game::new(chess::generate_starting_position()));
    let moves = game.get_all_possible_moves();
    for move_ in moves {
        println!("{} to {}", move_.from, move_.to);
    }
    display::run(game);
}
