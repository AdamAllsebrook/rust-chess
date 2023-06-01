use std::rc::Rc;

mod chess;
mod cli;

fn main() {
    let mut cli = cli::Cli::new();
    cli.run();
}
