extern crate clap;

use clap::{App, AppSettings};

fn main() {
    let app = App::new("matcha-cli")
        .version("0.1")
        .author("Andrew V. <andrew@andrewvy.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp);

    let _ = app.get_matches();
}
