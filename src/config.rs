use std::env;
use std::fs::{DirBuilder, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use failure::Error;

use bincode::{serialize, deserialize, Infinite};
use protocol;

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

pub fn load_bincode_from_file(path: &str) -> Result<protocol::Wallet, Error> {
    let mut file_in = try!(File::open(&get_config_dir().join(path)));
    let mut bytes = vec![];
    try!(file_in.read_to_end(&mut bytes));
    let object = deserialize(&bytes)?;
    Ok(object)
}

pub fn save_bincode_to_file(path: &str, msg: &protocol::Wallet) -> Result<(), Error> {
    let mut file_out = try!(File::create(&get_config_dir().join(path)));
    let bytes = serialize(msg, Infinite)?;
    file_out.write_all(&bytes)?;
    Ok(())
}
