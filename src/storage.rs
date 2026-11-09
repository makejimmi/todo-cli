use clap::error::ErrorKind::MissingRequiredArgument;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::format;
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
    pub const DATA_FILE_NAME: &str = "tasks.json";

    pub fn add(item: Item) -> Result<Option<String>, String> {
        let path = Self::init()?;
        Self::add_item(path, item)
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
        if !file_path.exists() {
            let mut file = File::create(&file_path).map_err(|e| e.to_string())?;
            file.write_all(b"{}").map_err(|e| e.to_string())?;
        }
        Ok(file_path)
    }

    fn add_item(file_path: PathBuf, item: Item) -> Result<Option<String>, String> {
        let mut tasks = Self::load(&file_path)?;
        tasks.insert(item.get_label(), item.get_message());
        let file = File::create(&file_path).map_err(|e| e.to_string())?;
        serde_json::to_writer_pretty(file, &tasks).map_err(|e| e.to_string())?;
        Ok(Some(item.summary()))
    }

    fn load(file_path: &PathBuf) -> Result<HashMap<String, Option<String>>, String> {
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
