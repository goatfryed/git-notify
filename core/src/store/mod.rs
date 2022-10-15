mod data;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use data::StoreData;

/// Work with data store


pub struct Store {
    pub data: StoreData,
    pub path: PathBuf,
}

pub fn load_store(path: PathBuf) -> Result<Store,Box<dyn Error>> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let data: StoreData = serde_json::from_reader(reader)?;

    Ok(Store { path, data })
}

