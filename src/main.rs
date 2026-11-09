mod app;
mod cli;
mod model;
mod storage;

use crate::app::run::run;
use clap::Parser;
use cli::parser::Cli;

fn main() {
    let cli = Cli::parse();

    match run(cli.command) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("{}", e),
    };
}
