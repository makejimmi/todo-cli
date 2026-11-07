use crate::cli::parser::Command;
use crate::model::item::Item;

pub fn run(command: Command) -> Item {
    match command {
        Command::Add(add) => {
            let item = Item::new(add.get_label(), add.get_message());
            println!("{}", item.summary());
            item
        }
    }
}
