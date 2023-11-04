extern crate clap;
use clap::{App, Arg};
use std::process;

/// Central application entry point.
fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION")) // CARGO_PKG_HOMEPAGE
        .arg(
            Arg::with_name("MEDIAFILE")
                .help("Pass a valid mp4 file path as an argument for inspection")
                .required(true)
                .index(1),
        )
        .get_matches();

    match mpn::run(matches) {
        Ok(_) => {
            process::exit(0);
        }
        Err(e) => {
            eprintln!("error = \"{}\"", e);
            process::exit(1);
        }
    }
}
