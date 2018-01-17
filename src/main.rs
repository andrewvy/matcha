extern crate clap;
extern crate bytes;
extern crate byteorder;
extern crate futures;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_timer;
extern crate rust_sodium;
extern crate hex;
extern crate serde;
extern crate bincode;
extern crate uuid;
extern crate rand;

#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

mod config;
mod wallet;
mod protocol;
mod codec;
mod node;
mod transaction;

use clap::{App, Arg, SubCommand, AppSettings};

fn main() {
    config::create_config_dir();

    let app = App::new("matcha")
        .version("0.1")
        .author("Andrew V. <andrew@andrewvy.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("daemon")
            .about("Starts the daemon process")
            .arg(
                Arg::with_name("port")
                .short("p")
                .takes_value(true)
                .required(true)
            )
            .arg(
                Arg::with_name("bootstrap")
                .short("b")
                .takes_value(true)
            )
        )
        .subcommand(
            SubCommand::with_name("wallet")
            .about("Commands around wallet functionality")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                SubCommand::with_name("create")
                .about("Creates a wallet")
            )
            .subcommand(
                SubCommand::with_name("list")
                .about("Lists your wallet")
            )
        );
 
    let matches = app.get_matches();

    match matches.subcommand() {
        ("daemon", Some(daemon_matches)) => {
            let port =
                if let Some(port) = daemon_matches.value_of("port") {
                   port
                } else {
                   "12345"
                };

            let server_addr = format!("127.0.0.1:{}", port);

            let mut bootstrap_nodes = Vec::new();

            if let Some(bootstrap_node) = daemon_matches.value_of("bootstrap") {
                bootstrap_nodes.push(bootstrap_node)
            }

            node::boot(server_addr, bootstrap_nodes);
        },
        ("wallet", Some(wallet_matches)) => {
            match wallet_matches.subcommand() {
                ("create", _) => wallet::create_wallet(),
                ("list", _) => wallet::list_wallet(),
                _ => unreachable!("No command specified!")
            }
        },
        ("", None) | _ => {
            println!("No command specified")
        }
    }
}
