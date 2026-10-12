//! # MPN - MPEG-4 Inspector Binary
//!
//! This is the main entry point for the MPN (MPEG-4 inspector) command-line application.
//! It parses command-line arguments and delegates to the library's `run` function
//! to perform the actual MP4 file inspection.
//!
//! ## Usage
//!
//! The application expects a single required argument: the path to a media file.
//! Container type is sniffed from magic bytes; deep track/codec inspect is
//! MPEG-4 / ISO BMFF only.
//!
//! ```bash
//! mpn path/to/video.mp4
//! ```
//!
//! ## Exit Codes
//!
//! - `0`: Success - file was classified (and inspected when MP4-family)
//! - `1`: Error - An error occurred (file not found, unreadable, etc.)

extern crate clap;
use clap::{Arg, Command};
use std::process;

/// Central application entry point.
///
/// This function:
/// 1. Sets up the command-line argument parser using clap
/// 2. Parses the command-line arguments
/// 3. Calls the library's `run` function with the parsed arguments
/// 4. Handles the result and exits with the appropriate exit code
///
/// # Exit Codes
///
/// - Returns exit code `0` on success
/// - Returns exit code `1` on error, printing the error message to stderr
fn main() {
    // Build the command-line interface using clap
    // The command name, version, and description are pulled from Cargo.toml
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION")) // CARGO_PKG_HOMEPAGE
        .arg(
            // Define the required MEDIAFILE argument
            // This is a positional argument (index 1) that must be provided
            Arg::new(mpn::ARG_MEDIAFILE)
                .help("Path to a media file (sniffed; deep inspect for MPEG-4 / ISO BMFF)")
                .required(true)
                .index(1), // Positional argument at index 1 (after the program name)
        )
        .get_matches(); // Parse the command-line arguments

    // Execute the main inspection logic
    // The `run` function handles all the MP4 parsing and output generation
    match mpn::run(matches) {
        Ok(_) => {
            // Success: exit with code 0
            process::exit(0);
        }
        Err(e) => {
            // Error: print error message to stderr and exit with code 1
            // Error format: error = "error message"
            eprintln!("error = \"{}\"", e);
            process::exit(1);
        }
    }
}
