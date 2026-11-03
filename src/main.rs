mod cli;

use clap::Parser;
use cli::parser::Cli;

fn main() {
    let todo = Cli::parse();

    todo.command.go()
}
