extern crate assert_cmd;
extern crate mp4parse;
extern crate mpn;
extern crate reqwest;

mod common;

use assert_cmd::prelude::*;
use predicates::prelude::*;

///
/// begin tests
///

#[test]
fn integ_cli_valid_file_path() {
    let mut cmd = std::process::Command::main_binary().unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    cmd.assert();
}

#[test]
#[should_panic]
fn integ_cli_invalid_mp4_file() {
    let mut cmd = std::process::Command::main_binary().unwrap();
    cmd.arg("src/main.rs");
    cmd.assert()
        .failure()
        .stderr("thread 'main' panicked at 'read_mp4 failed: UnexpectedEOF'");
}

/**
 * @see https://docs.rs/predicates/0.9.1/predicates/str/fn.contains.html
 */
#[test]
fn integ_cli_valid_stdout_dimensions() {
    let predicate_fn = predicate::str::contains("854");
    let mut cmd = std::process::Command::main_binary().unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    let output = String::from_utf8(cmd.output().unwrap().stdout);
    assert!(predicate_fn.eval(&output.unwrap()));
}

#[test]
fn integ_cli_valid_stdout_codec() {
    let predicate_fn = predicate::str::contains("AVC");
    let mut cmd = std::process::Command::main_binary().unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    let output = String::from_utf8(cmd.output().unwrap().stdout);
    assert!(predicate_fn.eval(&output.unwrap()));
}

#[test]
fn integ_cli_valid_stdout_media_track_video() {
    let predicate_fn = predicate::str::contains("video");
    let mut cmd = std::process::Command::main_binary().unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    let output = String::from_utf8(cmd.output().unwrap().stdout);
    assert!(predicate_fn.eval(&output.unwrap()));
}

#[test]
fn integ_cli_valid_stdout_media_track_audio() {
    let predicate_fn = predicate::str::contains("48000");
    let mut cmd = std::process::Command::main_binary().unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    let result = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
    assert!(predicate_fn.eval(&result));
}

#[test]
fn integ_cli_invalid_stdout_media_track_audio() {
    let predicate_fn = predicate::str::contains("audio");
    let mut cmd = std::process::Command::main_binary().unwrap();
    cmd.arg(common::TEST_BOKEH_AU_0T_VD_30F_854X480_MP4_FILE);
    let output = String::from_utf8(cmd.output().unwrap().stdout);
    assert!(!predicate_fn.eval(&output.unwrap()));
}

// #[test]
// fn integ_cli_invalid_file_path() {
//     let mut cmd = std::process::Command::main_binary().unwrap();
//     cmd.arg("nonexistent");
//     cmd.assert()
//         .failure()
//         .stderr("error = \"No such file or directory (os error 2)\"\n");
// }

/// if need by, run with: cargo test -- --nocapture
// #[test]
// fn integ_cli_valid_media_creation_time() {
//     common::setup();
//     let file_path: String = "tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4".to_string();
//     let config = mpn::Media::new(file_path.clone()).unwrap();
//     println!("creation_time = {}", config.creation_time);
//     assert!(config.creation_time > 0);
// }

/// thread 'main' panicked at 'read_mp4 failed: InvalidData("unread box content or bad parser sync")', src/libcore/result.rs:906:4
#[test]
#[should_panic]
fn integ_cli_nonexistant_file_2() {
    let args: Vec<String> = vec![
        String::from("mpn"),
        String::from("this_file_does_not.exist"),
    ];
    assert_eq!(args.len(), 2);

    let filename: String = String::from("this_file_does_not.exist");
    let config = mpn::Media::new(filename).unwrap();
    assert_eq!(
        config.filename,
        "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4"
    );
}
