use std::env;
use std::fs::{DirBuilder, File};
use std::io::{BufReader};
use std::path::{PathBuf};

use protobuf::{self, MessageStatic};
use failure::Error;

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

pub fn load_proto_from_file<T: MessageStatic>(path: &str) -> Result<T, Error> {
    match File::open(&get_config_dir().join(path)) {
        Ok(file_in) => {
            let mut reader = BufReader::new(file_in);

            protobuf::parse_from_reader(&mut reader)
                .map_err(|_| format_err!("Could not read from {}", path))
        },
        Err(_) => Err(format_err!("Could not open {} for reading", path)),
    }
}

pub fn save_proto_to_file<T: MessageStatic>(path: &str, msg: &T) -> Result<(), Error> {
    match File::create(&get_config_dir().join(path)) {
        Ok(mut file_out) => {
            msg
                .write_to_writer(&mut file_out)
                .map_err(|_| format_err!("Could not write to {}", path))
        },
        Err(_) => Err(format_err!("Could not open {} for writing", path)),
    }
}
