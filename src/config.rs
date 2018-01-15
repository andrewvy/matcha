use std::env;
use std::fs::{DirBuilder, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use protobuf::{self, MessageStatic};

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

pub fn load_proto_from_file<Message: MessageStatic>(path: &str) -> Result<Message, protobuf::ProtobufError> {
    let mut proto_in = try!(File::open(&get_config_dir().join(path)));
    let mut wallet_bytes = vec![];
    try!(proto_in.read_to_end(&mut wallet_bytes));
    Ok(try!(protobuf::parse_from_bytes(&wallet_bytes)))
}

pub fn save_proto_to_file<Message: MessageStatic>(path: &str, msg: &Message) -> Result<(), protobuf::ProtobufError> {
    let mut proto_out = try!(File::create(&get_config_dir().join(path)));
    let wallet_bytes = try!(msg.write_to_bytes());
    Ok(try!(proto_out.write_all(&wallet_bytes)))
}
