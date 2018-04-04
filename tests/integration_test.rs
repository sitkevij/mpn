extern crate mpi;
extern crate mp4parse;
extern crate assert_cli;

mod common;

use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::ffi::OsStr;

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

#[test]
fn filename() {
    common::setup();

    // We assume that we are in a valid directory.
    // let p = env::current_dir().unwrap();
    // let bin = env::current_exe().unwrap();
    // println!("bin: {}, current directory: {}", bin.display(), p.display());
    // let mut project_dir: PathBuf = get_project_dir();
    // project_dir.push("tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4");
    // let filename = get_full_path_as_string( String::from("tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4") );

    let project_dir = get_full_path(String::from(
        "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4",
    ));
    let filename = project_dir.into_os_string().into_string().unwrap(); // project_dir.to_string_lossy();

    let args: Vec<String> = vec![String::from("mpi"), String::from(filename.clone())];
    assert_eq!(args.len(), 2);

    // assert_eq!(project_dir.clone().exists(), true);

    assert_eq!(Path::new(&String::from(filename.clone())).exists(), true);

    let filename = String::from(filename.clone());
    let config = mpi::Media::new(filename.clone()).unwrap();
    assert_eq!(config.filename, filename.clone());
}

#[test]
fn noaudio_media() {
    common::setup();
    let filename = String::from("tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4");
    let file_path = get_full_path_as_string(filename);
    let config = mpi::Media::new(file_path.clone()).unwrap();
    assert_eq!(config.filename, file_path);
}

/// if need by, run with: cargo test -- --nocapture
#[test]
fn media_created() {
    common::setup();
    let filename = String::from("tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4");
    let file_path = get_full_path_as_string(filename);
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
