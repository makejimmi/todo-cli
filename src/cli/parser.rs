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

impl Command {
    pub fn go(&self) -> () {
        match self {
            Command::Add(add) => println!("{}", add.summary()),
        }
    }
}

impl Add {
    pub fn summary(&self) -> String {
        format!("{}{}", self.label_to_string(), self.message_to_string())
    }

    fn label_to_string(&self) -> String {
        format!("Added \"{}\"", self.label)
    }
    fn message_to_string(&self) -> String {
        format!(
            "\n  └─ note: {:?}",
            self.message.clone().unwrap_or_default()
        )
    }
}
