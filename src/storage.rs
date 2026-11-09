use clap::ValueHint::FilePath;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::cli::parser::Command;
use crate::model::item::Item;

pub struct Storage {}

impl Storage {
    pub const QUALIFIER: &str = "";
    pub const ORGANIZATION: &str = "";
    pub const APPLICATION: &str = "cod";
    pub const DATA_FILE_NAME: &str = "tasks.json";

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

        let file_path =
            match Self::get_or_create_file_valid_path(data_dir.join(Self::DATA_FILE_NAME)) {
                Ok(file_path) => file_path,
                Err(e) => return Err(format!("{}", e).into()),
            };
        Ok(file_path)
    }

    fn get_or_create_file_valid_path(file_path: PathBuf) -> Result<PathBuf, String> {
        if !Path::exists(file_path.as_path()) {
            match File::create(file_path.clone()) {
                Ok(_) => (),
                Err(e) => return Err(format!("{}", e).into()),
            };
        }
        Ok(file_path)
    }

    fn add_item(file_path: PathBuf, item: Item) -> Result<String, String> {
        let mut file = Self::open_mut_file(file_path).unwrap();

        match file.write_all(&item.summary().into_bytes()) {
            Ok(_) => Ok(item.summary()),
            Err(e) => Err(format!("{}", e).into()),
        }
    }

    fn open_mut_file(file_path: PathBuf) -> Result<File, String> {
        match OpenOptions::new().write(true).open(file_path) {
            Ok(file) => Ok(file),
            Err(e) => Err(format!("{}", e).into()),
        }
    }
}
