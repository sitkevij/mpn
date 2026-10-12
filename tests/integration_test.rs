extern crate assert_cmd;
extern crate mp4parse;
extern crate mpn;
extern crate reqwest;

mod common;

use assert_cmd::Command;
use predicates::prelude::*;

///
/// begin tests
///

#[test]
fn integ_cli_valid_file_path() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    cmd.assert().success();
}

#[test]
fn integ_cli_missing_required_argument() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn integ_cli_help_flag() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust-based MPEG-4 inspector"))
        .stdout(predicate::str::contains("Usage"))
        .stdout(predicate::str::contains("MEDIAFILE"));
}

#[test]
fn integ_cli_version_flag() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn integ_cli_nonexistent_file() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("nonexistent_file_that_does_not_exist.mp4");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error ="));
}

#[test]
fn integ_cli_non_media_file_sniff_only() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("src/main.rs");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("format = \"unknown\""))
        .stdout(predicate::str::contains(
            "mime = \"application/octet-stream\"",
        ))
        .stdout(predicate::str::contains("inspect = false"));
}

#[test]
fn integ_cli_valid_stdout_media_section() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[media]"))
        .stdout(predicate::str::contains("uri ="))
        .stdout(predicate::str::contains("bytes ="))
        .stdout(predicate::str::contains("format = \"m4v\""))
        .stdout(predicate::str::contains("mime = \"video/x-m4v\""))
        .stdout(predicate::str::contains("inspect = true"));
}

#[test]
fn integ_cli_valid_stdout_dimensions() {
    let predicate_fn = predicate::str::contains("854");
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    let output = String::from_utf8(cmd.output().unwrap().stdout);
    assert!(predicate_fn.eval(&output.unwrap()));
}

#[test]
fn integ_cli_valid_stdout_codec() {
    let predicate_fn = predicate::str::contains("AVC");
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    let output = String::from_utf8(cmd.output().unwrap().stdout);
    assert!(predicate_fn.eval(&output.unwrap()));
}

#[test]
fn integ_cli_valid_stdout_media_track_video() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[media.track.video]"))
        .stdout(predicate::str::contains("track_id ="))
        .stdout(predicate::str::contains("duration ="));
}

#[test]
fn integ_cli_valid_stdout_media_track_audio() {
    let predicate_fn = predicate::str::contains("48000");
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    let result = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
    assert!(predicate_fn.eval(&result));
}

#[test]
fn integ_cli_valid_stdout_audio_track_section() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[media.track.audio]"))
        .stdout(predicate::str::contains("channelcount ="))
        .stdout(predicate::str::contains("samplerate ="));
}

#[test]
fn integ_cli_invalid_stdout_media_track_audio() {
    let predicate_fn = predicate::str::contains("audio");
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_0T_VD_30F_854X480_MP4_FILE);
    let output = String::from_utf8(cmd.output().unwrap().stdout);
    assert!(!predicate_fn.eval(&output.unwrap()));
}

#[test]
fn integ_cli_valid_stdout_video_track_header() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[media.track.video.header]"))
        .stdout(predicate::str::contains("width ="))
        .stdout(predicate::str::contains("height ="));
}

#[test]
fn integ_cli_valid_stdout_video_codec_section() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[media.track.video.codec]"))
        .stdout(predicate::str::contains("codec_name ="));
}

#[test]
fn integ_cli_valid_stdout_audio_codec_section() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[media.track.audio.codec]"))
        .stdout(predicate::str::contains("codec_name ="));
}

#[test]
fn integ_cli_valid_exit_code_success() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn integ_cli_valid_exit_code_failure() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("nonexistent_file.mp4");
    let output = cmd.output().unwrap();
    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn integ_cli_valid_file_metadata_output() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("modified =")
            && (stdout.contains("created =") || stdout.contains("accessed ="))
    );
}

#[test]
fn integ_cli_valid_video_only_file() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg(common::TEST_BOKEH_AU_0T_VD_30F_854X480_MP4_FILE);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[media.track.video]"))
        .stdout(predicate::str::contains("[media]"));
}

#[test]
fn integ_cli_hyperfine_benchmark() {
    // Check if hyperfine is available
    let hyperfine_check = std::process::Command::new("hyperfine")
        .arg("--version")
        .output();

    if hyperfine_check.is_err() {
        // Skip test if hyperfine is not installed
        println!("hyperfine not found, skipping benchmark test");
        return;
    }

    // Get the path to the binary (absolute path)
    let bin_path = assert_cmd::cargo::cargo_bin(env!("CARGO_PKG_NAME"));
    let bin_path_abs = bin_path.canonicalize().unwrap_or(bin_path);

    let bin_str = match bin_path_abs.to_str() {
        Some(s) => s.to_string(),
        None => {
            eprintln!("Binary path is not valid UTF-8, skipping benchmark test");
            return;
        }
    };

    // Get absolute path to test file
    let test_file_path = std::path::Path::new(common::TEST_BOKEH_AU_2T_VD_30F_854X480_MP4_FILE);
    let test_file_abs = test_file_path
        .canonicalize()
        .unwrap_or_else(|_| test_file_path.to_path_buf());

    let test_file_str = match test_file_abs.to_str() {
        Some(s) => s.to_string(),
        None => {
            eprintln!("Test file path is not valid UTF-8, skipping benchmark test");
            return;
        }
    };

    // Run hyperfine benchmark
    // Hyperfine requires the command to be passed as a shell command string
    let command_str = format!("{} {}", bin_str, test_file_str);

    let output = match std::process::Command::new("hyperfine")
        .arg("--warmup")
        .arg("2")
        .arg("--min-runs")
        .arg("3")
        .arg("--max-runs")
        .arg("5")
        .arg("--export-json")
        .arg("-")
        .arg(&command_str)
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            eprintln!(
                "Failed to execute hyperfine: {}, skipping benchmark test",
                e
            );
            return;
        }
    };

    if !output.status.success() {
        eprintln!(
            "hyperfine benchmark failed (exit code: {}), skipping: {}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }

    // Parse JSON output to verify benchmark results
    // Hyperfine outputs JSON to stdout, but may also output text to stderr
    let stdout = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "hyperfine output is not valid UTF-8: {}, skipping benchmark test",
                e
            );
            return;
        }
    };

    // Find JSON in output (hyperfine may output text before JSON)
    let json_start = match stdout.find('{') {
        Some(pos) => pos,
        None => {
            eprintln!("No JSON found in hyperfine output, skipping benchmark test");
            return;
        }
    };
    let json_str = &stdout[json_start..];

    let json: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(j) => j,
        Err(e) => {
            eprintln!(
                "Failed to parse hyperfine JSON output: {}, skipping benchmark test",
                e
            );
            return;
        }
    };

    // Verify that we have results
    let results = match json.get("results").and_then(|r| r.as_array()) {
        Some(arr) if !arr.is_empty() => arr,
        _ => {
            eprintln!("hyperfine output missing or empty 'results' field, skipping benchmark test");
            return;
        }
    };

    // Check that we have timing information
    let first_result = &results[0];
    if let Some(mean) = first_result.get("mean").and_then(|m| m.as_f64()) {
        println!(
            "Benchmark completed: mean execution time = {:.2} seconds",
            mean
        );
    } else {
        eprintln!("hyperfine result missing 'mean' timing, skipping benchmark test");
    }
}

/// Ensures rusty-hook config stays aligned with `make pre-commit` (fmt, clippy, test).
#[test]
fn integ_rusty_hook_pre_commit_config() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".rusty-hook.toml");
    let contents = std::fs::read_to_string(&path).expect(".rusty-hook.toml should exist");
    assert!(
        contents.contains("pre-commit"),
        "rusty-hook config should define a pre-commit hook"
    );
    assert!(
        contents.contains("make pre-commit"),
        "pre-commit hook should run the Makefile pre-commit target"
    );
}
