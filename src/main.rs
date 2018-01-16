extern crate clap;
extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_core;
extern crate rust_sodium;
extern crate hex;
extern crate bincode;
extern crate serde;

#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

mod config;
mod wallet;
mod protocol;

use clap::{App, SubCommand, AppSettings};

fn main() {
    config::create_config_dir();

    let app = App::new("matcha")
        .version("0.1")
        .author("Andrew V. <andrew@andrewvy.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("daemon")
            .about("Starts the daemon process")
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
        ("daemon", _) => {
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
