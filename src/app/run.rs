use crate::cli::parser::Command;
use crate::model::item::Item;
use crate::storage::Storage;

pub fn run(command: Command) -> Result<String, String> {
    let ret: String;
    match command {
        Command::Add(add) => {
            ret = match Storage::add(Item::new(add.get_label(), add.get_message())) {
                Ok(summary) => summary,
                Err(e) => return Err(format!("{}", e)),
            };
        }
    }

    Ok(ret)
}
