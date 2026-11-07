mod app;
mod cli;
mod model;

use crate::app::run::run;
use clap::Parser;
use cli::parser::Cli;

fn main() {
    let cli = Cli::parse();

    run(cli.command);
}
