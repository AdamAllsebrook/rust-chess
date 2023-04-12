mod chess;

fn main() {
    let game = chess::Game::new(chess::generate_starting_position());
    let moves = game.get_all_possible_moves();
    for move_ in moves {
        println!("{} to {}", move_.from, move_.to);
    }
}
