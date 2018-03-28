// #![allow(warnings)]
extern crate mp4parse;
extern crate filetime;
extern crate clap;
extern crate chrono;

use self::chrono::prelude::TimeZone;
use clap::ArgMatches;
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::vec::Vec;

/// Media struct which holds file metadata
pub struct Media {
    pub filename: String,
    pub creation_time: i64,
    pub last_accessed_time: i64,
    pub last_modified_time: i64,
    pub preview: [u8; 256],
}

/// Media implementation
impl Media {
    pub fn new(filename: String) -> Result<Media, Box<::std::error::Error>> {
        let preview: [u8; 256] = [0x0; 256];
        let metadata = fs::metadata(filename.clone()).unwrap();
        let ctime = filetime::FileTime::from_creation_time(&metadata).unwrap();
        let mtime = filetime::FileTime::from_last_modification_time(&metadata);
        let atime = filetime::FileTime::from_last_access_time(&metadata);

        Ok(Media {
            filename,
            creation_time: ctime.seconds_relative_to_1970() as i64,
            last_accessed_time: atime.seconds_relative_to_1970() as i64,
            last_modified_time: mtime.seconds_relative_to_1970() as i64,
            preview,
        })
    }
}

/// Inspect mp4 file and output box metadata.
/// # Arguments
/// * `matches` - Argument matches from the command line input
pub fn run(matches: ArgMatches) -> Result<(), Box<::std::error::Error>> {
    // if let Some(ref file) = matches.value_of("MEDIAFILE") {
    if let Some(file) = matches.value_of("MEDIAFILE") {
        println!("[media]");
        println!("uri = \"{}\"", file);
        let mut fd = try!(File::open(file));
        let mut buf = Vec::new();
        try!(fd.read_to_end(&mut buf));
        let media = Media::new(file.to_string()).unwrap();
        println!(
            "creation_time = \"{}\"",
            chrono::Utc.timestamp(media.creation_time, 0)
        );
        println!(
            "last_modified_time = \"{}\"",
            chrono::Utc.timestamp(media.last_modified_time, 0)
        );
        println!(
            "last_accessed_time = \"{}\"",
            chrono::Utc.timestamp(media.last_accessed_time, 0)
        );
        let mut c = Cursor::new(buf);
        let mut context = mp4parse::MediaContext::new();
        mp4parse::read_mp4(&mut c, &mut context).expect("read_mp4 failed");
        for track in context.tracks {
            match track.data {
                Some(mp4parse::SampleEntry::Video(_v)) => {
                    println!("[media.track.video]");
                    println!("track_id = \"{:?}\"", track.track_id.unwrap());
                    println!("duration = \"{:?}\"", track.duration.unwrap());
                    println!("empty_duration = \"{:?}\"", track.empty_duration.unwrap());
                    println!("media_time = \"{:?}\"", track.media_time.unwrap());
                    println!("timescale = \"{:?}\"", track.timescale.unwrap());
                    println!("[media.track.video.dimension]");
                    println!("width = {:?}", _v.width);
                    println!("height = {:?}", _v.height);

                    let thb = track.tkhd.unwrap(); // TrackHeaderBox
                    println!("[media.track.video.header]");
                    println!("disabled = {:?}", thb.disabled);
                    println!("duration = {:?}", thb.duration);
                    println!("width = {:?}", thb.width);
                    println!("height = {:?}", thb.height);

                    let mut vcsd = HashMap::new(); // VideoCodecSpecific data
                    let codec = match _v.codec_specific {
                        mp4parse::VideoCodecSpecific::AVCConfig(_v) => "AVC",
                        mp4parse::VideoCodecSpecific::VPxConfig(_vpx) => {
                            vcsd.insert(String::from("vpx.bit_depth"), _vpx.bit_depth);
                            vcsd.insert(String::from("vpx.color_space"), _vpx.color_space);
                            vcsd.insert(
                                String::from("vpx.chroma_subsampling"),
                                _vpx.chroma_subsampling,
                            );
                            "VPx"
                        }
                        mp4parse::VideoCodecSpecific::ESDSConfig(_mp4v) => "MP4V",
                    };
                    println!("[media.track.video.codec]");
                    println!("codec_name = \"{}\"", codec);
                    for (key, value) in &vcsd {
                        println!("{} = {:?}", key, value);
                    }
                }
                Some(mp4parse::SampleEntry::Audio(a)) => {
                    println!("[media.track.audio]");
                    println!("track_id = \"{:?}\"", track.track_id.unwrap());
                    println!("duration = \"{:?}\"", track.duration.unwrap());
                    println!("empty_duration = \"{:?}\"", track.empty_duration.unwrap());
                    println!("media_time = \"{:?}\"", track.media_time.unwrap());
                    println!("timescale = \"{:?}\"", track.timescale.unwrap());
                    println!("[media.track.audio.dimension]");
                    println!("channelcount = {:?}", a.channelcount);
                    println!("samplesize = {:?}", a.samplesize);
                    println!("samplerate = {:?}", a.samplerate);

                    let thb = track.tkhd.unwrap();
                    println!("[media.track.audio.header]");
                    println!("disabled = {:?}", thb.disabled);
                    println!("duration = {:?}", thb.duration);
                    println!("width = {:?}", thb.width);
                    println!("height = {:?}", thb.height);

                    let mut acsd = HashMap::new(); // AudioCodecSpecific data
                    let codec = match a.codec_specific {
                        mp4parse::AudioCodecSpecific::ES_Descriptor(esds) => {
                            // @see https://docs.rs/enum_derive/*/enum_derive/index.html
                            // @see https://stackoverflow.com/questions/39146584/how-do-i-create-a-rust-hashmap-where-the-value-can-be-one-of-multiple-types
                            // @see http://siciarz.net/24-days-of-rust-anymap/
                            acsd.insert(
                                String::from("esds.audio_sample_rate"),
                                esds.audio_sample_rate.unwrap() as u32,
                            );
                            acsd.insert(
                                String::from("esds.audio_object_type"),
                                esds.audio_object_type.unwrap() as u32,
                            );
                            "ES"
                        }
                        mp4parse::AudioCodecSpecific::FLACSpecificBox(flac) => {
                            acsd.insert(
                                String::from("flac.blocks[0].block_type"),
                                flac.blocks[0].block_type as u32,
                            );
                            acsd.insert(
                                String::from("flac.blocks[0].data.len()"),
                                flac.blocks[0].data.len() as u32,
                            );
                            "FLAC"
                        }
                        mp4parse::AudioCodecSpecific::OpusSpecificBox(opus) => {
                            acsd.insert(String::from("opus.version"), opus.version as u32);
                            "Opus"
                        }
                        mp4parse::AudioCodecSpecific::ALACSpecificBox(alac) => {
                            acsd.insert(String::from("alac.data.len()"), alac.data.len() as u32);
                            "ALAC"
                        }
                        mp4parse::AudioCodecSpecific::MP3 => "MP3",
                        mp4parse::AudioCodecSpecific::LPCM => "LPCM",
                    };

                    println!("[media.track.audio.codec]");
                    println!("codec_name = \"{}\"", codec);
                    for (key, value) in &acsd {
                        println!("{} = {:?}", key, value);
                    }
                }
                Some(mp4parse::SampleEntry::Unknown) |
                None => {}
            }
            //
        }
    }
    println!("");
    Ok(())
}

pub const TESTS_SMALL: [u8; 8] = [0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70];

/// @see (https://doc.rust-lang.org/book/second-edition/ch11-03-test-organization.html)
#[cfg(test)]
mod tests {
    use super::*;
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
    fn unit_process_file() {
        let filename = String::from("tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4");
        let file_path = get_full_path_as_string(filename.clone());

        // assert_eq!(Path::new(&String::from(file_path.clone())).exists(), true);

        let config = Media::new(file_path.clone()).unwrap();
        assert_eq!(
            config.filename,
            file_path.clone()
        );
    }
    #[test]
    fn unit_args() {
        let filename = String::from("tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4");
        // let file_path = get_full_path_as_string(filename.clone());
        let args: Vec<String> = vec![
            String::from("mpi"),
            filename.clone(),
        ];
        assert_eq!(args.len(), 2);
    }
}
