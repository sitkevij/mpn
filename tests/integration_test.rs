extern crate mpi;
extern crate mp4parse;

mod common;

#[test]
fn filename() {
    common::setup();
    let filename = "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4";
    let args: Vec<String> = vec![
        String::from("mpi"),
        String::from(filename),
    ];
    assert_eq!(args.len(), 2);

    let filename = String::from(filename);
    let config = mpi::Media::new(filename).unwrap();
    assert_eq!(
        config.filename,
        "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4"
    );
}

#[test]
fn noaudio_media() {
    common::setup();
    let filename = String::from("tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4");
    let config = mpi::Media::new(filename).unwrap();
    assert_eq!(
        config.filename,
        "tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4"
    );
}

/// if need by, run with: cargo test -- --nocapture
#[test]
fn media_created() {
    common::setup();
    let filename = String::from("tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4");
    let config = mpi::Media::new(filename).unwrap();
    println!("creation_time = {}", config.creation_time);
    assert_eq!(
        config.creation_time,
        1519926310
    );
}

#[test]
#[should_panic]
fn invalid_mp4() {
    common::setup();
    // common::TESTS_FILES_TEST_BOKEH_AU_2T_VD_30F_854X480_MP4;
    let args: Vec<String> = vec![String::from("mpi"), String::from("src/main.rs")];
    assert_eq!(args.len(), 2);

    let filename: String = String::from("src/main.rs");
    let config = mpi::Media::new(filename).unwrap();
    assert_eq!(
        config.filename,
        "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4"
    );
}

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
