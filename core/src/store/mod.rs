mod data;

use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use data::StoreData;

/// Work with data store


pub struct Store {
    pub data: StoreData,
    pub path: PathBuf,
}

impl Store {
    pub fn save(&self) -> Result<(),Box<dyn Error>> {

        let file = File::options()
            .create(true).create_new(true)
            .truncate(true)
            .open(&self.path)?;

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.data)?;
        Ok(())
    }
}

pub fn find_and_load_store() -> Store {
    let path = PathBuf::from("./.git-notify");
    if !path.exists() {
        panic!("couldn't find persistence file. Expected it at {}", path.display());
    }

    load_store(path).unwrap()
}

pub fn load_store(path: PathBuf) -> Result<Store,Box<dyn Error>> {

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let data: StoreData = serde_json::from_reader(reader)?;

    Ok(Store { path, data })
}

