use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, Write};
use std::num::NonZeroU16;
use std::path::{Path, PathBuf};
use std::ptr::null;

use crate::model::item::Item;

pub struct Storage {}

impl Storage {
    pub const QUALIFIER: &str = "";
    pub const ORGANIZATION: &str = "";
    pub const APPLICATION: &str = "cod";
    pub const DATA_FILE_NAME: &str = "tasks.jsonl";

    pub fn add(item: Item) -> Result<String, String> {
        let path = Self::init()?;

        let summary = match Self::add_item(path, item) {
            Ok(summary) => summary,
            Err(e) => return Err(format!("{}", e).into()),
        };
        Ok(summary)
    }

    fn init() -> Result<PathBuf, String> {
        let proj_dirs = ProjectDirs::from(Self::QUALIFIER, Self::ORGANIZATION, Self::APPLICATION)
            .ok_or("Could not determine system data directory")?;

        let data_dir = proj_dirs.data_dir();
        match fs::create_dir_all(data_dir) {
            Ok(_) => (),
            Err(e) => return Err(format!("{}", e).into()),
        };

        let file_path = match Self::get_or_create_file_path(data_dir.join(Self::DATA_FILE_NAME)) {
            Ok(file_path) => file_path,
            Err(e) => return Err(format!("{}", e).into()),
        };
        Ok(file_path)
    }

    fn get_or_create_file_path(file_path: PathBuf) -> Result<PathBuf, String> {
        if !Path::exists(file_path.as_path()) {
            match File::create(file_path.clone()) {
                Ok(_) => (),
                Err(e) => return Err(format!("{}", e).into()),
            };
        }
        Ok(file_path)
    }

    fn add_item(file_path: PathBuf, item: Item) -> Option<String> {
        let mut tasks = match Self::load(file_path) {
            Ok(map) => map,
            Err(e) => return format!("{}", e).into(),
        };
        match tasks.insert(item.get_label(), item.get_message()) {
            Some(something) => Some(something),
            None => None,
        };
        None
    }

    fn load(file_path: PathBuf) -> Result<HashMap<String, Option<String>>, String> {
        let file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => return Err(format!("{}", e).into()),
        };

        let reader = BufReader::new(file);
        let map: HashMap<String, Option<String>> = match serde_json::from_reader(reader) {
            Ok(map) => map,
            Err(e) => return Err(format!("{}", e).into()),
        };
        Ok(map)
    }
}
