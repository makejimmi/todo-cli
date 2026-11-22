use std::error::Error;

use crate::cli::parser::Command;
use crate::model::item::Item;
use crate::storage::Storage;

pub fn run(command: Command) -> Result<Option<String>, Box<dyn Error>> {
    let ret: Option<String>;
    match command {
        Command::Add(add) => ret = Storage::add(Item::new(add.get_label(), add.get_message()))?,
    };

    Ok(ret)
}
