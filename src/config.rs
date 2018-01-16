use std::env;
use std::fs::{DirBuilder, File};
use std::io::Write;
use std::path::PathBuf;

use failure::Error;
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize_from, Infinite};

pub fn get_config_dir() -> PathBuf {
    env::home_dir().unwrap().join(".config/matcha")
}

pub fn create_config_dir() {
    let base_path = get_config_dir();

    DirBuilder::new()
        .recursive(true)
        .create(base_path)
        .expect("Could not create matcha config dir");
}

pub fn load_bincode_from_file<T>(path: &str) -> Result<T, Error> where for<'de> T: Deserialize<'de> {
    let mut file_in = try!(File::open(&get_config_dir().join(path)));
    let object = deserialize_from(&mut file_in, Infinite)?;
    Ok(object)
}

pub fn save_bincode_to_file<T>(path: &str, msg: &T) -> Result<(), Error> where T: Serialize {
    let mut file_out = try!(File::create(&get_config_dir().join(path)));
    let bytes = serialize(msg, Infinite)?;
    file_out.write_all(&bytes)?;
    Ok(())
}
