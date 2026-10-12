//! Magic-byte / container signature sniffing for common media formats.
//!
//! Detection is intentionally shallow: identify container kind and MIME, and
//! whether this crate can deep-inspect the file (ISO BMFF / MP4-family only).

/// Result of sniffing a media container from leading file bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DetectedFormat {
    /// Short format id (e.g. `"mp4"`, `"mkv"`, `"wav"`).
    pub id: &'static str,
    /// IANA-style MIME type for the container.
    pub mime: &'static str,
    /// Whether [`crate::run`] will deep-inspect via `mp4parse`.
    pub inspectable: bool,
}

impl DetectedFormat {
    /// Build a detected format descriptor.
    pub const fn new(id: &'static str, mime: &'static str, inspectable: bool) -> Self {
        Self {
            id,
            mime,
            inspectable,
        }
    }
}

/// Placeholder when no known signature matches.
pub const UNKNOWN: DetectedFormat =
    DetectedFormat::new("unknown", "application/octet-stream", false);

/// Sniff `bytes` for a known video/audio container signature.
///
/// Returns [`None`] when the buffer is empty or no signature matches.
pub fn sniff(bytes: &[u8]) -> Option<DetectedFormat> {
    if bytes.is_empty() {
        return None;
    }
    sniff_bmff(bytes)
        .or_else(|| sniff_matroska(bytes))
        .or_else(|| sniff_riff(bytes))
        .or_else(|| sniff_ogg(bytes))
        .or_else(|| sniff_flac(bytes))
        .or_else(|| sniff_aiff(bytes))
        .or_else(|| sniff_flv(bytes))
        .or_else(|| sniff_asf(bytes))
        .or_else(|| sniff_mpeg_ts(bytes))
        .or_else(|| sniff_mp3(bytes))
}

/// Resolve a format for output: known sniff, else [`UNKNOWN`].
pub fn sniff_or_unknown(bytes: &[u8]) -> DetectedFormat {
    sniff(bytes).unwrap_or(UNKNOWN)
}

fn sniff_bmff(bytes: &[u8]) -> Option<DetectedFormat> {
    // ISO BMFF: [size:u32][b"ftyp"][major_brand:4]...
    if bytes.len() < 12 {
        return None;
    }
    if &bytes[4..8] != b"ftyp" {
        return None;
    }
    let brand = &bytes[8..12];
    Some(match brand {
        b"M4A " | b"M4B " | b"M4P " => DetectedFormat::new("m4a", "audio/mp4", true),
        b"M4V " => DetectedFormat::new("m4v", "video/x-m4v", true),
        b"qt  " => DetectedFormat::new("mov", "video/quicktime", true),
        b"3gp4" | b"3gp5" | b"3gp6" | b"3g2a" | b"3g2b" | b"3g2c" | b"3ge6" | b"3gg6" | b"3gs6" => {
            DetectedFormat::new("3gp", "video/3gpp", true)
        }
        // isom / iso2 / mp41 / mp42 / avc1 / dash / etc. → mp4
        _ => DetectedFormat::new("mp4", "video/mp4", true),
    })
}

fn sniff_matroska(bytes: &[u8]) -> Option<DetectedFormat> {
    // EBML header 0x1A45DFA3
    if bytes.len() < 4 {
        return None;
    }
    if bytes[0..4] != [0x1A, 0x45, 0xDF, 0xA3] {
        return None;
    }
    // Prefer WebM when DocType "webm" appears in the early EBML header region.
    let window = &bytes[..bytes.len().min(256)];
    if contains_slice(window, b"webm") {
        return Some(DetectedFormat::new("webm", "video/webm", false));
    }
    Some(DetectedFormat::new("mkv", "video/x-matroska", false))
}

fn sniff_riff(bytes: &[u8]) -> Option<DetectedFormat> {
    // RIFF....WAVE / AVI / WEBP (webp excluded — images out of scope)
    if bytes.len() < 12 {
        return None;
    }
    if &bytes[0..4] != b"RIFF" {
        return None;
    }
    match &bytes[8..12] {
        b"WAVE" => Some(DetectedFormat::new("wav", "audio/wav", false)),
        b"AVI " => Some(DetectedFormat::new("avi", "video/x-msvideo", false)),
        _ => None,
    }
}

fn sniff_ogg(bytes: &[u8]) -> Option<DetectedFormat> {
    if bytes.len() >= 4 && &bytes[0..4] == b"OggS" {
        Some(DetectedFormat::new("ogg", "application/ogg", false))
    } else {
        None
    }
}

fn sniff_flac(bytes: &[u8]) -> Option<DetectedFormat> {
    if bytes.len() >= 4 && &bytes[0..4] == b"fLaC" {
        Some(DetectedFormat::new("flac", "audio/flac", false))
    } else {
        None
    }
}

fn sniff_aiff(bytes: &[u8]) -> Option<DetectedFormat> {
    // FORM....AIFF or AIFC
    if bytes.len() < 12 {
        return None;
    }
    if &bytes[0..4] != b"FORM" {
        return None;
    }
    match &bytes[8..12] {
        b"AIFF" | b"AIFC" => Some(DetectedFormat::new("aiff", "audio/aiff", false)),
        _ => None,
    }
}

fn sniff_flv(bytes: &[u8]) -> Option<DetectedFormat> {
    if bytes.len() >= 3 && &bytes[0..3] == b"FLV" {
        Some(DetectedFormat::new("flv", "video/x-flv", false))
    } else {
        None
    }
}

fn sniff_asf(bytes: &[u8]) -> Option<DetectedFormat> {
    // ASF header GUID 3026B275-8E66-CF11-A6D9-00AA0062CE6C
    const ASF_HEADER: [u8; 16] = [
        0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE,
        0x6C,
    ];
    if bytes.len() >= 16 && bytes[0..16] == ASF_HEADER {
        Some(DetectedFormat::new("asf", "video/x-ms-asf", false))
    } else {
        None
    }
}

fn sniff_mpeg_ts(bytes: &[u8]) -> Option<DetectedFormat> {
    // 188-byte packets starting with sync byte 0x47
    const PACKET: usize = 188;
    if bytes.len() < PACKET * 2 {
        return None;
    }
    if bytes[0] == 0x47 && bytes[PACKET] == 0x47 {
        Some(DetectedFormat::new("mpegts", "video/mp2t", false))
    } else {
        None
    }
}

fn sniff_mp3(bytes: &[u8]) -> Option<DetectedFormat> {
    if bytes.len() >= 3 && &bytes[0..3] == b"ID3" {
        return Some(DetectedFormat::new("mp3", "audio/mpeg", false));
    }
    // MPEG frame sync: 11 set bits then layer/bitrate sanity
    if bytes.len() >= 2 && bytes[0] == 0xFF && (bytes[1] & 0xE0) == 0xE0 {
        let layer = (bytes[1] >> 1) & 0x03;
        // Layer III = 0b01 in the 2-bit field (MPEG numbering is inverted in the header)
        if layer == 0b01 {
            return Some(DetectedFormat::new("mp3", "audio/mpeg", false));
        }
    }
    None
}

fn contains_slice(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ftyp(brand: &[u8; 4]) -> Vec<u8> {
        let mut v = vec![0, 0, 0, 20];
        v.extend_from_slice(b"ftyp");
        v.extend_from_slice(brand);
        v.extend_from_slice(b"isom"); // compatible brand filler
        v
    }

    #[test]
    fn sniff_empty_is_none() {
        assert!(sniff(&[]).is_none());
        assert_eq!(sniff_or_unknown(&[]), UNKNOWN);
    }

    #[test]
    fn sniff_mp4_isom() {
        let d = sniff(&ftyp(b"isom")).unwrap();
        assert_eq!(d.id, "mp4");
        assert_eq!(d.mime, "video/mp4");
        assert!(d.inspectable);
    }

    #[test]
    fn sniff_m4a_brand() {
        let d = sniff(&ftyp(b"M4A ")).unwrap();
        assert_eq!(d.id, "m4a");
        assert!(d.inspectable);
    }

    #[test]
    fn sniff_mov_brand() {
        let d = sniff(&ftyp(b"qt  ")).unwrap();
        assert_eq!(d.id, "mov");
        assert!(d.inspectable);
    }

    #[test]
    fn sniff_3gp_brand() {
        let d = sniff(&ftyp(b"3gp4")).unwrap();
        assert_eq!(d.id, "3gp");
        assert!(d.inspectable);
    }

    #[test]
    fn sniff_mkv_ebml() {
        let mut buf = vec![0x1A, 0x45, 0xDF, 0xA3];
        buf.extend_from_slice(&[0; 32]);
        let d = sniff(&buf).unwrap();
        assert_eq!(d.id, "mkv");
        assert!(!d.inspectable);
    }

    #[test]
    fn sniff_webm_doctype() {
        let mut buf = vec![0x1A, 0x45, 0xDF, 0xA3];
        buf.extend_from_slice(b"....webm....");
        let d = sniff(&buf).unwrap();
        assert_eq!(d.id, "webm");
    }

    #[test]
    fn sniff_wav_riff() {
        let mut buf = b"RIFF".to_vec();
        buf.extend_from_slice(&[0, 0, 0, 0]);
        buf.extend_from_slice(b"WAVE");
        let d = sniff(&buf).unwrap();
        assert_eq!(d.id, "wav");
        assert!(!d.inspectable);
    }

    #[test]
    fn sniff_avi_riff() {
        let mut buf = b"RIFF".to_vec();
        buf.extend_from_slice(&[0, 0, 0, 0]);
        buf.extend_from_slice(b"AVI ");
        assert_eq!(sniff(&buf).unwrap().id, "avi");
    }

    #[test]
    fn sniff_ogg() {
        assert_eq!(sniff(b"OggS........").unwrap().id, "ogg");
    }

    #[test]
    fn sniff_flac() {
        assert_eq!(sniff(b"fLaC........").unwrap().id, "flac");
    }

    #[test]
    fn sniff_aiff() {
        let mut buf = b"FORM".to_vec();
        buf.extend_from_slice(&[0, 0, 0, 0]);
        buf.extend_from_slice(b"AIFF");
        assert_eq!(sniff(&buf).unwrap().id, "aiff");
    }

    #[test]
    fn sniff_flv() {
        assert_eq!(sniff(b"FLV\x01......").unwrap().id, "flv");
    }

    #[test]
    fn sniff_asf() {
        let mut buf = vec![
            0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62,
            0xCE, 0x6C,
        ];
        buf.extend_from_slice(&[0; 8]);
        assert_eq!(sniff(&buf).unwrap().id, "asf");
    }

    #[test]
    fn sniff_mpeg_ts_sync() {
        let mut buf = vec![0u8; 188 * 2];
        buf[0] = 0x47;
        buf[188] = 0x47;
        assert_eq!(sniff(&buf).unwrap().id, "mpegts");
    }

    #[test]
    fn sniff_mp3_id3() {
        assert_eq!(sniff(b"ID3\x03\x00........").unwrap().id, "mp3");
    }

    #[test]
    fn sniff_mp3_frame_sync() {
        // Frame sync + Layer III (bits ...xx where layer field is 01)
        // byte1: 1110_0001 → sync continuation + MPEG1 + Layer3
        assert_eq!(sniff(&[0xFF, 0xFB, 0x90, 0x00]).unwrap().id, "mp3");
    }

    #[test]
    fn sniff_unknown_text() {
        assert!(sniff(b"fn main() {}").is_none());
        assert_eq!(sniff_or_unknown(b"fn main() {}").id, "unknown");
    }
}
