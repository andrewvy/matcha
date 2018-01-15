extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate rust_sodium;
extern crate hex;
extern crate protobuf;

mod config;
mod wallet;
mod matcha_pb;

fn main() {
    config::create_config_dir();
    wallet::create_wallet();
}
