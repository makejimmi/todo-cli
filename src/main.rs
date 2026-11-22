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
        Ok(Some(msg)) => println!("{}", msg),
        Ok(None) => println!("No summary available"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
