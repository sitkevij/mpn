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
use mp4parse::AudioCodecSpecific;
use mp4parse::SampleEntry;
use mp4parse::TrackType;
use mp4parse::VideoCodecSpecific;
use mp4parse::read_mp4;
use no_color::is_no_color;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read};

/// MEDIAFILE Argument constant
pub const ARG_MEDIAFILE: &str = "MEDIAFILE";

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
    if let Some(file) = matches.get_one::<String>(ARG_MEDIAFILE).map(|s| s.as_str()) {
        println!("[media]");
        println!("uri = \"{}\"", file);
        let mut fd = File::open(file)?;
        let mut buf = Vec::new();
        let size = fd.read_to_end(&mut buf)?;
        let metadata = fs::metadata(file)?;
        println!("bytes = {}", size);
        if let Ok(time) = metadata.modified() {
            println!(
                "modified = {:?}",
                chrono::Utc
                    .timestamp_opt(
                        time.duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                            .try_into()
                            .unwrap(),
                        0
                    )
                    .unwrap()
            );
        } else {
            println!(
                "modified = {:?}",
                "\"error: not supported on this platform.\""
            );
        }
        if let Ok(time) = metadata.created() {
            println!(
                "created = {:?}",
                chrono::Utc
                    .timestamp_opt(
                        time.duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                            .try_into()
                            .unwrap(),
                        0
                    )
                    .unwrap()
            );
        } else {
            println!(
                "created = {:?}",
                "\"error: not supported on this platform.\""
            );
        }
        if let Ok(time) = metadata.accessed() {
            println!(
                "accessed = {:?}",
                chrono::Utc
                    .timestamp_opt(
                        time.duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs()
                            .try_into()
                            .unwrap(),
                        0
                    )
                    .unwrap()
            );
        } else {
            println!(
                "accessed = {:?}",
                "\"error: not supported on this platform.\""
            );
        }
        let mut c = Cursor::new(buf);
        let context = read_mp4(&mut c).expect("read_mp4 failed");
        for track in context.tracks {
            match track.track_type {
                // see https://docs.rs/mp4parse/latest/mp4parse/struct.Track.html
                TrackType::Video => {
                    println!("[media.track.video]");
                    println!("track_id = {:?}", track.track_id.unwrap());
                    println!("duration = {:?}", track.duration.unwrap());
                    println!("empty_duration = \"{:?}\"", track.empty_duration.unwrap());
                    println!("media_time = \"{:?}\"", track.media_time.unwrap()); // 1 = 64 bit creation and modification times. 0 = 64 bit creation and modification times.
                    println!("timescale = \"{:?}\"", track.timescale.unwrap());

                    let thb = track.tkhd.unwrap(); // TrackHeaderBox
                    println!("[media.track.video.header]");
                    println!("disabled = {:?}", thb.disabled);
                    println!("duration = {:?}", thb.duration);
                    println!("width = {:?}", thb.width);
                    println!("height = {:?}", thb.height);

                    let stsd = track
                        .stsd
                        .expect("TrackType::Video missing SampleDescriptionBox");
                    let v = match stsd
                        .descriptions
                        .first()
                        .expect("TrackType::Video missing SampleEntry")
                    {
                        SampleEntry::Video(v) => v,
                        _ => panic!("TrackType::Video missing VideoSampleEntry"),
                    };
                    println!("[media.track.video.sample.entry]");
                    println!("width = {:?}", v.width);
                    println!("height = {:?}", v.height);

                    let mut vcsd = HashMap::new(); // VideoCodecSpecific data
                    let codec = match v.codec_specific {
                        VideoCodecSpecific::AV1Config(ref _av1c) => "AV1",
                        VideoCodecSpecific::AVCConfig(ref avc) => {
                            // vcsd.insert(String::from("avc.bytes_length"), avc.len());
                            "AVC"
                        }
                        VideoCodecSpecific::VPxConfig(ref vpx) => {
                            vcsd.insert(String::from("vpx.bit_depth"), vpx.bit_depth);
                            vcsd.insert(String::from("vpx.colour_primaries"), vpx.colour_primaries);
                            vcsd.insert(
                                String::from("vpx.chroma_subsampling"),
                                vpx.chroma_subsampling,
                            );
                            "VPx"
                        }
                        VideoCodecSpecific::ESDSConfig(ref mp4v) => "MP4V",
                        VideoCodecSpecific::H263Config(ref _h263) => "H263",
                    };
                    println!("[media.track.video.codec]");
                    println!("codec_name = \"{}\"", codec);
                    for (key, value) in &vcsd {
                        println!("{} = {:?}", key, value);
                    }
                }
                TrackType::Audio => {
                    println!("[media.track.audio]");
                    println!("track_id = {:?}", track.track_id.unwrap());
                    println!("duration = \"{:?}\"", track.duration.unwrap());
                    println!("empty_duration = \"{:?}\"", track.empty_duration.unwrap());
                    println!("media_time = \"{:?}\"", track.media_time.unwrap());
                    println!("timescale = \"{:?}\"", track.timescale.unwrap());

                    let thb = track.tkhd.unwrap();
                    println!("[media.track.audio.header]");
                    println!("disabled = {:?}", thb.disabled);
                    println!("duration = {:?}", thb.duration);
                    println!("width = {:?}", thb.width);
                    println!("height = {:?}", thb.height);

                    let stsd = track
                        .stsd
                        .expect("TrackType::Audio missing SampleDescriptionBox");
                    let a = match stsd
                        .descriptions
                        .first()
                        .expect("TrackType::Audio missing SampleEntry")
                    {
                        SampleEntry::Audio(a) => a,
                        _ => panic!("TrackType::Audio missing AudioSampleEntry"),
                    };

                    println!("[media.track.audio.sample.entry]");
                    println!("channelcount = {:?}", a.channelcount);
                    println!("samplesize = {:?}", a.samplesize);
                    println!("samplerate = {:?}", a.samplerate);

                    let mut acsd = HashMap::new(); // AudioCodecSpecific data
                    let codec = match &a.codec_specific {
                        AudioCodecSpecific::ES_Descriptor(esds) => {
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
                        AudioCodecSpecific::FLACSpecificBox(flac) => {
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
                        AudioCodecSpecific::OpusSpecificBox(opus) => {
                            acsd.insert(String::from("opus.version"), opus.version as u32);
                            "Opus"
                        }
                        AudioCodecSpecific::ALACSpecificBox(alac) => {
                            acsd.insert(String::from("alac.data.len()"), alac.data.len() as u32);
                            "ALAC"
                        }
                        AudioCodecSpecific::MP3 => "MP3",
                        AudioCodecSpecific::LPCM => "LPCM",
                    };

                    println!("[media.track.audio.codec]");
                    println!("codec_name = \"{}\"", codec);
                    for (key, value) in &acsd {
                        println!("{} = {:?}", key, value);
                    }
                }
                TrackType::Picture => {
                    println!("[media.track.picture]");
                    println!(
                        "error = {:?}",
                        "TrackType::Picture found, but not supported by this application."
                    );
                }
                TrackType::AuxiliaryVideo => {
                    println!("[media.track.auxiliaryvideo]");
                    println!(
                        "error = {:?}",
                        "TrackType::AuxiliaryVideo found, but not supported by this application."
                    );
                }
                TrackType::Metadata => {
                    println!("[media.track.metadata]");
                    println!(
                        "error = {:?}",
                        "TrackType::Metadata found, but not supported by this application."
                    );
                }
                TrackType::Unknown => {
                    println!("[media.track.unknown]");
                    println!("error = {:?}", "TrackType::Unknown.");
                }
            }
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
