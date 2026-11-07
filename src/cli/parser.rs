use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo", version, about = "A simply CLI todo app.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Add(Add),
}
#[derive(Args, Debug)]
pub struct Add {
    pub label: String,

    #[arg(short, long)]
    pub message: Option<String>,
}

impl Add {
    pub fn get_label(&self) -> String {
        self.label.clone()
    }
    pub fn get_message(&self) -> Option<String> {
        self.message.clone()
    }
}
