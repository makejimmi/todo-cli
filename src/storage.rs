// use directories::ProjectDirs;
// use serde::{Deserialize, Serialize};
// use std::fs::{self, File};
// use std::io::{BufReader, BufWriter};
// use std::path::PathBuf;

// pub struct Storage {
//     file_path: PathBuf,
// }

// impl Storage {
//     pub const QUALIFIER: &str = "";
//     pub const ORGANIZATION: &str = "";
//     pub const APPLICATION: &str = "todo-cli";
//     pub const DATA_FILE_NAME: &str = "tasks.json";

//     pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
//         let proj_dirs = ProjectDirs::from(Self::QUALIFIER, Self::ORGANIZATION, Self::APPLICATION)
//             .ok_or("Could not determine system data directory")?;

//         let data_dir = proj_dirs.data_dir();
//         fs::create_dir_all(data_dir)?;

//         Ok(Self {
//             file_path: data_dir.join(Self::DATA_FILE_NAME),
//         })
//     }
// }
