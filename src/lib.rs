#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_assignments
)]
#![allow(unused_assignments, unused_variables)]

//! mpn main lib
extern crate chrono;
extern crate clap;
extern crate filetime;
extern crate mp4parse;

use self::chrono::prelude::TimeZone;
use clap::ArgMatches;
use no_color::is_no_color;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read};

/// Media struct which holds file metadata
pub struct Media {
    /// filename
    pub filename: String,
    /// file creation time
    pub creation_time: i64,
    /// file last accessed time
    pub last_accessed_time: i64,
    /// file last modified time
    pub last_modified_time: i64,
    /// file preview in bytes
    pub preview: [u8; 256],
}

impl Debug for Media {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.preview[0])
    }
}

/// Media implementation
impl Media {
    /// constructor
    pub fn new(filename: String) -> Result<Media, Box<dyn Error>> {
        let preview: [u8; 256] = [0x0; 256];
        // println!("Media.new filename={}", filename);
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
pub fn run(matches: ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut colorize_outout = true;
    if is_no_color() {
        colorize_outout = false;
    }
    if let Some(file) = matches.value_of("MEDIAFILE") {
        println!("[media]");
        println!("uri = \"{}\"", file);
        let mut fd = File::open(file)?;
        let mut buf = Vec::new();
        let size = fd.read_to_end(&mut buf)?;
        let media = Media::new(file.to_string()).unwrap();
        println!("bytes = {}", size);
        println!(
            "creation_time = \"{}\"",
            chrono::Utc.timestamp_opt(media.creation_time, 0).unwrap()
        );
        println!(
            "last_modified_time = \"{}\"",
            chrono::Utc
                .timestamp_opt(media.last_modified_time, 0)
                .unwrap()
        );
        println!(
            "last_accessed_time = \"{}\"",
            chrono::Utc
                .timestamp_opt(media.last_accessed_time, 0)
                .unwrap()
        );
        let mut c = Cursor::new(buf);
        let mut context = mp4parse::MediaContext::new();
        mp4parse::read_mp4(&mut c, &mut context).expect("read_mp4 failed");
        for track in context.tracks {
            match track.data {
                // Some(mp4parse::SampleEntry::Video(_v)) => {
                Some(mp4parse::SampleEntry::Video(_v)) => {
                    println!("[media.track.video]");
                    println!("track_id = {:?}", track.track_id.unwrap());
                    println!("duration = {:?}", track.duration.unwrap());
                    println!("empty_duration = \"{:?}\"", track.empty_duration.unwrap());
                    println!("media_time = \"{:?}\"", track.media_time.unwrap()); // 1 = 64 bit creation and modification times. 0 = 64 bit creation and modification times.
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
                    println!("track_id = {:?}", track.track_id.unwrap());
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
                                esds.audio_sample_rate.unwrap(),
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
                Some(mp4parse::SampleEntry::Unknown) | None => {}
            }
            //
        }
    }
    println!();
    Ok(())
}

/// bit array for testing
//  pub const TESTS_SMALL: [u8; 8] = [0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70];
/// @see (https://doc.rust-lang.org/book/second-edition/ch11-03-test-organization.html)
#[cfg(test)]
mod tests {
    extern crate assert_cmd;
    extern crate tempfile;

    #[test]
    fn unit_args() {
        let filename = String::from("tests/files/test-bokeh-au-0t-vd-30f-854x480.mp4");
        let args: Vec<String> = vec![String::from("mpn"), filename.clone()];
        assert_eq!(args.len(), 2);
    }
}
