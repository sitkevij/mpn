# mpn

[mpn](https://github.com/sitkevij/mpn) is a rust-based command line application which can inspect MPEG-4 media files,
read box information, and output in variable text formats.

```sh
mpn 0.2.0
author https://github.com/sitkevij
MPEG-4 media file inspector.

USAGE:
    mpn <MEDIAFILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <MEDIAFILE>    Pass a valid mp4 file path as an argument for inspection
```

| branch  | ci status |
|---------|-----------|
| main    | [![ci](https://github.com/sitkevij/mpn/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sitkevij/mpn/actions/workflows/ci.yml)|
| develop | [![ci](https://github.com/sitkevij/mpn/actions/workflows/ci.yml/badge.svg?branch=develop)](https://github.com/sitkevij/mpn/actions/workflows/ci.yml)|

# build & run

This mini-guide expects rust and cargo already installed.

## clone, test, build, run

```sh
$ git clone https://github.com/sitkevij/mpn && \
cd mpn && \
cargo test && \
cargo build --release && \
target/release/mpn tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4
```

## output

```toml
[media]
uri = "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4"
creation_time = "2018-03-14 15:24:40 UTC"
last_modified_time = "2018-03-14 15:24:40 UTC"
last_accessed_time = "2018-03-14 15:28:25 UTC"
[media.track.audio]
track_id = "1"
duration = "TrackScaledTime(50176, 0)"
empty_duration = "MediaScaledTime(0)"
media_time = "TrackScaledTime(0, 0)"
timescale = "TrackTimeScale(48000, 0)"
[media.track.audio.dimension]
channelcount = 2
samplesize = 16
samplerate = 48000.0
[media.track.audio.header]
disabled = true
duration = 30
width = 0
height = 0
[media.track.audio.codec]
codec_name = "ES"
esds.audio_sample_rate = 48000
esds.audio_object_type = 2
[media.track.video]
track_id = "2"
duration = "TrackScaledTime(30, 1)"
empty_duration = "MediaScaledTime(0)"
media_time = "TrackScaledTime(1, 1)"
timescale = "TrackTimeScale(30, 1)"
[media.track.video.dimension]
width = 854
height = 450
[media.track.video.header]
disabled = true
duration = 30
width = 55967744
height = 29491200
[media.track.video.codec]
codec_name = "AVC"
```

## License

MIT
