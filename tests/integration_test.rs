extern crate mpi;
extern crate mp4parse;
extern crate assert_cli;
extern crate reqwest;

mod common;

use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::ffi::OsStr;
use std::fs::File;
// use std::io::prelude::*;

fn get_project_dir() -> PathBuf {
    let bin = env::current_exe().expect("bin path");
    let mut target_dir = PathBuf::from(bin.parent().expect("bin parent"));
    while target_dir.file_name() != Some(OsStr::new("target")) {
        target_dir.pop();
    }
    target_dir.pop();
    target_dir
}

fn get_full_path(path: String) -> PathBuf {
    let mut project_dir: PathBuf = get_project_dir();
    project_dir.push(path);
    println!("get_full_path = {}", project_dir.to_string_lossy());
    project_dir
}

fn get_full_path_as_string(path: String) -> String {
    let full_path = get_full_path(path);
    full_path.into_os_string().into_string().unwrap()
}

fn get_test_file() -> std::ffi::OsString {
    let file_path = "au-0t-vd-30f.mp4";
    let path = env::temp_dir().join(file_path);
    // if Path::new(&path).exists() == false {
    let mut file = File::create(path).expect("Unable to create temporary test mp4 file");
    let mut res = reqwest::get("https://raw.githubusercontent.com/sitkevij/mpi/master/tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4").expect("request failed");

    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());

    assert_eq!(res.status(), reqwest::StatusCode::Ok);

    // let _ = std::io::copy(&mut res, &mut std::io::stdout()).expect("copy stream failed");
    let _ = std::io::copy(&mut res, &mut file).expect("copy stream failed");
    // }
    env::temp_dir().join(file_path).into_os_string()
}
///
/// begin tests
///
#[test]
fn integ_cli_valid_file() {
    assert_cli::Assert::main_binary().with_args(&[get_test_file().to_str().unwrap()]);
}

// #[test]
// fn integ_cli_valid_stdout_dimensions() {
//     assert_cli::Assert::main_binary()
//         // .with_args(&["tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4"])
//         .with_args(&[get_test_file().to_str().unwrap()])
//         .and()
//         .stdout()
//         .contains("width = 854")
//         .unwrap();
// }

// #[test]
// fn integ_cli_valid_stdout_codec() {
//     assert_cli::Assert::main_binary()
//         .with_args(&[get_test_file().to_str().unwrap()])
//         .and()
//         .stdout()
//         .contains("codec_name = \"AVC\"")
//         .unwrap();
// }

#[test]
fn integ_cli_valid_stdout_track() {
    assert_cli::Assert::main_binary()
        .with_args(&[get_test_file().to_str().unwrap()])
        .and()
        .stdout()
        .contains("[media.track.video]")
        .unwrap();
}

// #[test]
// fn integ_cli_valid_stdout_no_audio() {
//     assert_cli::Assert::main_binary()
//         .with_args(&[get_test_file().to_str().unwrap()])
//         .and()
//         .stdout()
//         .not()
//         .contains("[media.track.audio]")
//         .unwrap();
// }

#[test]
fn filename() {
    common::setup();
    let filename = get_test_file().into_string().unwrap();
    let args: Vec<String> = vec![String::from("mpi"), String::from(filename.clone())];

    assert_eq!(args.len(), 2);
    // assert_eq!(Path::new(&String::from(filename.clone())).exists(), true);

    let filename = String::from(filename.clone());
    let config = mpi::Media::new(filename.clone()).unwrap();
    assert_eq!(config.filename, filename.clone());
}

#[test]
fn noaudio_media() {
    common::setup();
    let filename = get_test_file().into_string().unwrap();
    // let file_path = get_full_path_as_string(filename);
    let config = mpi::Media::new(filename.clone()).unwrap();
    assert_eq!(config.filename, filename);
}

/// if need by, run with: cargo test -- --nocapture
#[test]
fn media_created() {
    common::setup();
    let file_path = get_test_file().into_string().unwrap();
    let config = mpi::Media::new(file_path.clone()).unwrap();
    println!("creation_time = {}", config.creation_time);
    assert!(config.creation_time > 0);
}

// #[test]
// #[should_panic]
// fn invalid_mp4() {
//     common::setup();
//     // common::TESTS_FILES_TEST_BOKEH_AU_2T_VD_30F_854X480_MP4;
//     let args: Vec<String> = vec![String::from("mpi"), String::from("src/main.rs")];
//     assert_eq!(args.len(), 2);

//     let filename = String::from("src/main.rs");
//     let file_path = get_full_path_as_string(filename);
//     let config = mpi::Media::new(file_path.clone()).unwrap();
//     assert_eq!(
//         config.filename,
//         file_path
//     );
// }

#[test]
#[should_panic]
fn nonexistant_file() {
    common::setup();
    let args: Vec<String> = vec![
        String::from("mpi"),
        String::from("this_file_does_not.exist"),
    ];
    assert_eq!(args.len(), 2);

    let filename: String = String::from("this_file_does_not.exist");
    let config = mpi::Media::new(filename).unwrap();
    assert_eq!(
        config.filename,
        "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4"
    );
}

/// thread 'main' panicked at 'read_mp4 failed: InvalidData("unread box content or bad parser sync")', src/libcore/result.rs:906:4
#[test]
#[should_panic]
fn nonexistant_file_2() {
    common::setup();
    let args: Vec<String> = vec![
        String::from("mpi"),
        String::from("this_file_does_not.exist"),
    ];
    assert_eq!(args.len(), 2);

    let filename: String = String::from("this_file_does_not.exist");
    let config = mpi::Media::new(filename).unwrap();
    assert_eq!(
        config.filename,
        "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4"
    );
}
