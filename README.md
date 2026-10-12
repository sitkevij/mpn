# mpn

[mpn](https://github.com/sitkevij/mpn) is a Rust command-line tool that sniffs common media
containers from magic bytes, deep-inspects MPEG-4 / ISO BMFF (MP4-family) files with
[mp4parse](https://github.com/mozilla/mp4parse-rust), and prints structured TOML-like text to
stdout.

[![Crates.io](https://img.shields.io/crates/v/mpn?style=flat-square)](https://crates.io/crates/mpn)
[![Crates.io](https://img.shields.io/crates/d/mpn?style=flat-square)](https://crates.io/crates/mpn)
[![GitHub Repo stars](https://img.shields.io/github/stars/sitkevij/mpn)](https://github.com/sitkevij/mpn)
[![GitHub repo size](https://img.shields.io/github/repo-size/sitkevij/mpn)](https://github.com/sitkevij/mpn)
[![main](https://github.com/sitkevij/mpn/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sitkevij/mpn/actions/workflows/ci.yml?branch=main)
[![docs.rs](https://img.shields.io/docsrs/mpn)](https://docs.rs/mpn)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/sitkevij/mpn/blob/main/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/sitkevij/mpn/blob/main/LICENSE-MIT)

## Usage

```sh
mpn path/to/media.file
mpn --help
```

Every run emits a `[media]` block with sniffed `format`, `mime`, and `inspect`. When
`inspect = true`, track and codec sections from `mp4parse` are included. Other recognized
containers exit successfully with `inspect = false` and no track dump.

Exit status is `0` on success (including sniff-only), `1` on I/O or parse errors (`error = "…"`
on stderr).

## Sniffing vs inspect

| Stage | What happens |
| --- | --- |
| Sniff | Read leading bytes; set `format`, `mime`, and whether the file is MP4-family. |
| Inspect | Only when MP4-family (`inspect = true`): parse boxes, tracks, codecs. |

Sniffed containers include MP4 / M4V / MOV / M4A / 3GP (inspectable), plus Matroska / WebM,
WAV, AVI, Ogg, FLAC, AIFF, FLV, ASF, MPEG-TS, MP3, and unknown (`application/octet-stream`).

## Output

MP4-family file (inspect runs; track sections truncated):

```toml
$ mpn tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4

[media]
uri = "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4"
bytes = 441551
format = "m4v"
mime = "video/x-m4v"
inspect = true
modified = …
[media.track.audio]
track_id = 1
…
[media.track.video.codec]
codec_name = "AVC"
```

Non-media or unrecognized content (sniff only):

```toml
$ mpn src/main.rs

[media]
uri = "src/main.rs"
bytes = …
format = "unknown"
mime = "application/octet-stream"
inspect = false
```

## Installation

### crates.io

If Rust and Cargo are installed, this is the quickest way to get a release binary:

```sh
cargo install mpn
```

### From source

```sh
git clone https://github.com/sitkevij/mpn && \
cd mpn && \
cargo test && \
cargo build --release && \
target/release/mpn tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4
```

### Packaging

Distro packages and other channels:

[![Packaging status](https://repology.org/badge/vertical-allrepos/mpn.svg)](https://repology.org/project/mpn/versions)

## Development

```sh
make ci          # format check, clippy, tests (matches GitHub CI core)
make pre-commit  # format, clippy, tests (local commits)
```

The first `cargo test` builds dev-dependencies and installs [rusty-hook](https://github.com/swellaby/rusty-hook)
from `.rusty-hook.toml` (pre-commit runs `make pre-commit`).

## License

MIT OR Apache-2.0

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
