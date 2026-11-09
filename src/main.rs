mod app;
mod cli;
mod model;
mod storage;

use crate::app::run::run;
use clap::Parser;
use cli::parser::Cli;

fn main() {
    let cli = Cli::parse();

    if let Err(msg) = run(cli.command) {
        eprintln!("error: {msg}");
        std::process::exit(1);
    };
}
