extern crate assert_cli;
extern crate mp4parse;
extern crate mpn;
extern crate reqwest;

mod common;

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

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

///
/// begin tests
///

#[test]
fn integ_cli_valid_file() {
    assert_cli::Assert::main_binary().with_args(&[common::get_temp_file_blocking(
        "https://raw.githubusercontent.com/sitkevij/mpn/master/tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4".to_string(),
  "au-0t-vd-30f.mp4".to_string())
        .unwrap().to_str().unwrap()
        ]);
}

#[test]
// common::get_file_name(common::get_uri_string_to_url(common::TEST_BOKEH_AU_2T_VD_30F_854X48q0_MP4_URI))
fn integ_cli_valid_stdout_dimensions() {
    common::setup();
    println!(
        "{:?}",
        common::get_uri_file_name(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_URI)
    );
    assert_cli::Assert::main_binary()
        .with_args(&[&common::get_temp_file_path(common::get_uri_file_name(
            common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_URI,
        ))])
        .and()
        .stdout()
        .contains("width = 854")
        .unwrap();
}

#[test]
fn integ_cli_valid_stdout_codec() {
    common::setup();
    assert_cli::Assert::main_binary()
        .with_args(&["tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4"])
        // .with_args(&[get_test_file().to_str().unwrap()])
        .and()
        .stdout()
        .contains("codec_name = \"AVC\"")
        .unwrap();
}

#[test]
fn integ_cli_valid_stdout_track() {
    assert_cli::Assert::main_binary()
        .with_args(&["tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4"])
        // .with_args(&[get_test_file().to_str().unwrap()])
        .and()
        .stdout()
        .contains("[media.track.video]")
        .unwrap();
}

#[test]
fn integ_cli_valid_stdout_no_audio() {
    assert_cli::Assert::main_binary()
        .with_args(&["tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4"])
        // .with_args(&[get_test_file().to_str().unwrap()])
        .and()
        .stdout()
        .not()
        .contains("[media.track.audio]")
        .unwrap();
}

// #[test]
// fn filename() {
//     common::setup();
//     let filename = get_test_file().into_string().unwrap();
//     let args: Vec<String> = vec![String::from("mpn"), String::from(filename.clone())];

//     assert_eq!(args.len(), 2);
//     // assert_eq!(Path::new(&String::from(filename.clone())).exists(), true);

//     let filename = String::from(filename.clone());
//     let config = mpn::Media::new(filename.clone()).unwrap();
//     assert_eq!(config.filename, filename.clone());
// }

// #[test]
// fn noaudio_media() {
//     common::setup();
//     let filename = get_test_file().into_string().unwrap();
//     // let file_path = get_full_path_as_string(filename);
//     let config = mpn::Media::new(filename.clone()).unwrap();
//     assert_eq!(config.filename, filename);
// }

/// if need by, run with: cargo test -- --nocapture
#[test]
fn integ_cli_valid_media_creation_time() {
    common::setup();
    let file_path: String = "tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4".to_string();
    let config = mpn::Media::new(file_path.clone()).unwrap();
    println!("creation_time = {}", config.creation_time);
    assert!(config.creation_time > 0);
}

#[test]
fn integ_cli_missing_params() {
    assert_cli::Assert::main_binary()
        .fails()
        .and()
        .stderr()
        .contains("The following required arguments were not provided")
        .unwrap();
}

#[test]
#[should_panic]
fn integ_cli_invalid_mp4() {
    let args: Vec<String> = vec![String::from("mpn"), String::from("src/main.rs")];
    assert_eq!(args.len(), 2);

    let filename = String::from("src/main.rs");
    let file_path = get_full_path_as_string(filename);
    let config = mpn::Media::new(file_path.clone()).unwrap();
    assert_eq!(config.filename, file_path);

    assert_cli::Assert::main_binary()
        .with_args(&[&file_path])
        .and()
        .stdout()
        .not()
        .contains("[media.track.audio]")
        .unwrap();
}

#[test]
#[should_panic]
fn integ_cli_nonexistant_file() {
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
