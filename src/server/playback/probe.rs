//! Reads the container and the codecs of a media file without decoding it

use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Container {
    Mp4,
    Webm,
    Matroska,
    Avi,
    Mp3,
    Wav,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codec {
    H264,
    Hevc,
    Vp8,
    Vp9,
    Av1,
    Mpeg4,
    Aac,
    Mp3,
    Opus,
    Vorbis,
    Flac,
    Ac3,
    Eac3,
    Dts,
    TrueHd,
    Pcm,
}

/// The container and the codecs of the first video and audio tracks of a media file.
/// A codec is `None` when the track doesn't exist or its codec isn't recognized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaInfo {
    pub container: Container,
    pub video: Option<Codec>,
    pub audio: Option<Codec>,
}

impl Container {
    /// The name of the container in the capabilities cookie sent by the browser
    pub fn token(&self) -> &'static str {
        match self {
            Container::Mp4 => "mp4",
            Container::Webm => "webm",
            Container::Matroska => "mkv",
            Container::Avi => "avi",
            Container::Mp3 => "mp3",
            Container::Wav => "wav",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Container::Mp4 => "MP4",
            Container::Webm => "WebM",
            Container::Matroska => "MKV",
            Container::Avi => "AVI",
            Container::Mp3 => "MP3",
            Container::Wav => "WAV",
        }
    }
}

impl Codec {
    pub fn is_video(&self) -> bool {
        matches!(
            self,
            Codec::H264 | Codec::Hevc | Codec::Vp8 | Codec::Vp9 | Codec::Av1 | Codec::Mpeg4
        )
    }

    /// The name of the codec in the capabilities cookie sent by the browser
    pub fn token(&self) -> &'static str {
        match self {
            Codec::H264 => "h264",
            Codec::Hevc => "hevc",
            Codec::Vp8 => "vp8",
            Codec::Vp9 => "vp9",
            Codec::Av1 => "av1",
            Codec::Mpeg4 => "mpeg4",
            Codec::Aac => "aac",
            Codec::Mp3 => "mp3",
            Codec::Opus => "opus",
            Codec::Vorbis => "vorbis",
            Codec::Flac => "flac",
            Codec::Ac3 => "ac3",
            Codec::Eac3 => "eac3",
            Codec::Dts => "dts",
            Codec::TrueHd => "truehd",
            Codec::Pcm => "pcm",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Codec::H264 => "H.264",
            Codec::Hevc => "HEVC (H.265)",
            Codec::Vp8 => "VP8",
            Codec::Vp9 => "VP9",
            Codec::Av1 => "AV1",
            Codec::Mpeg4 => "MPEG-4 Part 2 (Xvid/DivX)",
            Codec::Aac => "AAC",
            Codec::Mp3 => "MP3",
            Codec::Opus => "Opus",
            Codec::Vorbis => "Vorbis",
            Codec::Flac => "FLAC",
            Codec::Ac3 => "Dolby Digital (AC-3)",
            Codec::Eac3 => "Dolby Digital Plus (E-AC-3)",
            Codec::Dts => "DTS",
            Codec::TrueHd => "Dolby TrueHD",
            Codec::Pcm => "PCM",
        }
    }
}

/// Reads the container and the codecs of a media file.
/// Returns `None` if the file can't be read or its format isn't recognized.
pub fn probe(path: &Path) -> Option<MediaInfo> {
    let mut file = File::open(path).ok()?;
    let mut magic = [0u8; 12];
    let read = file.read(&mut magic).ok()?;
    let magic = &magic[..read];
    file.seek(SeekFrom::Start(0)).ok()?;

    if magic.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
        return probe_matroska(&mut file).ok();
    }
    if magic.len() >= 8 && &magic[4..8] == b"ftyp" {
        return probe_mp4(&mut file).ok();
    }
    if magic.len() >= 12 && &magic[0..4] == b"RIFF" && &magic[8..12] == b"AVI " {
        return Some(MediaInfo {
            container: Container::Avi,
            video: None,
            audio: None,
        });
    }

    // Audio files are identified by their extension
    let extension = path.extension()?.to_str()?.to_ascii_lowercase();
    let (container, audio) = match extension.as_str() {
        "mp3" => (Container::Mp3, Codec::Mp3),
        "wav" => (Container::Wav, Codec::Pcm),
        _ => return None,
    };
    Some(MediaInfo {
        container,
        video: None,
        audio: Some(audio),
    })
}

fn invalid_data(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.to_string())
}

/// The largest `moov` box that will be read in memory
const MAX_MOOV_SIZE: u64 = 64 * 1024 * 1024;

/// Finds the `moov` box at the top level of an MP4 file.
/// It can be located after the media data, so the boxes in between are skipped.
fn probe_mp4(file: &mut File) -> io::Result<MediaInfo> {
    let file_size = file.metadata()?.len();
    let mut position = 0;

    while position + 8 <= file_size {
        file.seek(SeekFrom::Start(position))?;
        let mut header = [0u8; 8];
        file.read_exact(&mut header)?;
        let mut size = u32::from_be_bytes(header[0..4].try_into().unwrap()) as u64;
        let mut header_size = 8;

        if size == 1 {
            let mut large_size = [0u8; 8];
            file.read_exact(&mut large_size)?;
            size = u64::from_be_bytes(large_size);
            header_size = 16;
        } else if size == 0 {
            size = file_size - position;
        }
        if size < header_size {
            return Err(invalid_data("invalid box size"));
        }

        if &header[4..8] == b"moov" {
            let content_size = size - header_size;
            if content_size > MAX_MOOV_SIZE {
                return Err(invalid_data("moov box too large"));
            }
            let mut moov = vec![0u8; content_size as usize];
            file.read_exact(&mut moov)?;
            return Ok(parse_moov(&moov));
        }

        position += size;
    }

    Err(invalid_data("moov box not found"))
}

/// Iterates over the boxes contained in `data`, yielding their type and content
fn mp4_boxes(data: &[u8]) -> impl Iterator<Item = (&[u8], &[u8])> {
    let mut position = 0;
    std::iter::from_fn(move || {
        let header = data.get(position..position + 8)?;
        let size = u32::from_be_bytes(header[0..4].try_into().unwrap()) as usize;
        let (content_start, end) = match size {
            0 => (position + 8, data.len()),
            1 => {
                let large_size = data.get(position + 8..position + 16)?;
                let large_size = u64::from_be_bytes(large_size.try_into().unwrap()) as usize;
                (position + 16, position.checked_add(large_size)?)
            }
            _ => (position + 8, position.checked_add(size)?),
        };
        if end < content_start || end > data.len() {
            return None;
        }
        position = end;
        Some((&header[4..8], &data[content_start..end]))
    })
}

fn find_mp4_box<'a>(data: &'a [u8], path: &[&[u8]]) -> Option<&'a [u8]> {
    let (first, rest) = path.split_first()?;
    let (_, content) = mp4_boxes(data).find(|(kind, _)| kind == first)?;
    match rest.is_empty() {
        true => Some(content),
        false => find_mp4_box(content, rest),
    }
}

fn parse_moov(moov: &[u8]) -> MediaInfo {
    let mut info = MediaInfo {
        container: Container::Mp4,
        video: None,
        audio: None,
    };

    for (_, trak) in mp4_boxes(moov).filter(|(kind, _)| *kind == b"trak") {
        let Some(stsd) = find_mp4_box(trak, &[b"mdia", b"minf", b"stbl", b"stsd"]) else {
            continue;
        };
        // Skip the version, the flags and the entry count to get to the first sample entry
        let Some((format, entry)) = stsd.get(8..).and_then(|e| mp4_boxes(e).next()) else {
            continue;
        };

        match mp4_codec(format, entry) {
            Some(codec) if codec.is_video() => info.video = info.video.or(Some(codec)),
            Some(codec) => info.audio = info.audio.or(Some(codec)),
            None => {}
        }
    }

    info
}

/// Identifies the codec of an MP4 sample entry from its format
fn mp4_codec(format: &[u8], entry: &[u8]) -> Option<Codec> {
    Some(match format {
        b"avc1" | b"avc3" => Codec::H264,
        b"hvc1" | b"hev1" => Codec::Hevc,
        b"vp08" => Codec::Vp8,
        b"vp09" => Codec::Vp9,
        b"av01" => Codec::Av1,
        b"mp4v" => Codec::Mpeg4,
        b"mp4a" => return mp4a_codec(entry),
        b".mp3" => Codec::Mp3,
        b"Opus" => Codec::Opus,
        b"fLaC" => Codec::Flac,
        b"ac-3" => Codec::Ac3,
        b"ec-3" => Codec::Eac3,
        b"dtsc" | b"dtsh" | b"dtsl" | b"dtse" => Codec::Dts,
        b"mlpa" => Codec::TrueHd,
        b"lpcm" | b"sowt" | b"twos" | b"ipcm" => Codec::Pcm,
        _ => return None,
    })
}

/// `mp4a` sample entries can contain AAC or MP3 audio, which is told apart
/// by the object type in the `esds` box that follows the audio sample entry fields
fn mp4a_codec(entry: &[u8]) -> Option<Codec> {
    const AUDIO_SAMPLE_ENTRY_SIZE: usize = 28;
    let esds = find_mp4_box(entry.get(AUDIO_SAMPLE_ENTRY_SIZE..)?, &[b"esds"])?;
    let object_type = esds_object_type(esds.get(4..)?)?;

    match object_type {
        0x40 | 0x66 | 0x67 | 0x68 => Some(Codec::Aac),
        0x69 | 0x6B => Some(Codec::Mp3),
        0xA5 => Some(Codec::Ac3),
        0xA6 => Some(Codec::Eac3),
        0xA9 => Some(Codec::Dts),
        _ => None,
    }
}

/// Reads the object type from an ES descriptor
fn esds_object_type(descriptors: &[u8]) -> Option<u8> {
    let mut position = 0;
    let (tag, content_start) = read_descriptor_header(descriptors, &mut position)?;
    if tag != 0x03 {
        return None;
    }

    // ES_ID, then flags that tell which optional fields follow
    let flags = *descriptors.get(content_start + 2)?;
    position = content_start + 3;
    if flags & 0x80 != 0 {
        position += 2; // dependsOn_ES_ID
    }
    if flags & 0x40 != 0 {
        position += 1 + *descriptors.get(position)? as usize; // URL
    }
    if flags & 0x20 != 0 {
        position += 2; // OCR_ES_Id
    }

    let (tag, content_start) = read_descriptor_header(descriptors, &mut position)?;
    match tag {
        0x04 => descriptors.get(content_start).copied(),
        _ => None,
    }
}

/// Reads a descriptor tag and its size, which is encoded on up to 4 bytes
fn read_descriptor_header(data: &[u8], position: &mut usize) -> Option<(u8, usize)> {
    let tag = *data.get(*position)?;
    *position += 1;
    for _ in 0..4 {
        let byte = *data.get(*position)?;
        *position += 1;
        if byte & 0x80 == 0 {
            break;
        }
    }
    Some((tag, *position))
}

mod ebml {
    pub const HEADER: u64 = 0x1A45DFA3;
    pub const DOC_TYPE: u64 = 0x4282;
    pub const SEGMENT: u64 = 0x18538067;
    pub const TRACKS: u64 = 0x1654AE6B;
    pub const TRACK_ENTRY: u64 = 0xAE;
    pub const TRACK_TYPE: u64 = 0x83;
    pub const CODEC_ID: u64 = 0x86;
    pub const CLUSTER: u64 = 0x1F43B675;
}

/// The largest element that will be read in memory
const MAX_ELEMENT_SIZE: u64 = 16 * 1024 * 1024;

/// Reads an EBML variable size integer. The ID keeps its length marker,
/// the size doesn't and is `None` when it is unknown.
fn read_vint(file: &mut File, keep_marker: bool) -> io::Result<(u64, bool)> {
    let mut first = [0u8; 1];
    file.read_exact(&mut first)?;
    let length = first[0].leading_zeros() as usize + 1;
    if length > 8 {
        return Err(invalid_data("invalid variable size integer"));
    }

    let mut value = match keep_marker {
        true => first[0] as u64,
        false => (first[0] as u64) & (0xFF >> length),
    };
    let mut all_ones = value == (0xFF >> length);
    let mut rest = [0u8; 7];
    file.read_exact(&mut rest[..length - 1])?;
    for &byte in &rest[..length - 1] {
        value = (value << 8) | byte as u64;
        all_ones &= byte == 0xFF;
    }

    Ok((value, all_ones && !keep_marker))
}

/// Reads an element header, returning its ID and its size (`None` if unknown)
fn read_element_header(file: &mut File) -> io::Result<(u64, Option<u64>)> {
    let (id, _) = read_vint(file, true)?;
    let (size, unknown) = read_vint(file, false)?;
    Ok((id, if unknown { None } else { Some(size) }))
}

fn read_element(file: &mut File, size: Option<u64>) -> io::Result<Vec<u8>> {
    let size = size.ok_or_else(|| invalid_data("element of unknown size"))?;
    if size > MAX_ELEMENT_SIZE {
        return Err(invalid_data("element too large"));
    }
    let mut content = vec![0u8; size as usize];
    file.read_exact(&mut content)?;
    Ok(content)
}

/// Iterates over the EBML elements contained in `data`, yielding their ID and content
fn ebml_elements(data: &[u8]) -> impl Iterator<Item = (u64, &[u8])> {
    let mut position = 0;
    std::iter::from_fn(move || {
        let read_vint = |position: &mut usize, keep_marker: bool| -> Option<u64> {
            let first = *data.get(*position)?;
            let length = first.leading_zeros() as usize + 1;
            if length > 8 {
                return None;
            }
            let mut value = match keep_marker {
                true => first as u64,
                false => (first as u64) & (0xFF >> length),
            };
            for &byte in data.get(*position + 1..*position + length)? {
                value = (value << 8) | byte as u64;
            }
            *position += length;
            Some(value)
        };

        let id = read_vint(&mut position, true)?;
        let size = read_vint(&mut position, false)? as usize;
        let content = data.get(position..position.checked_add(size)?)?;
        position += size;
        Some((id, content))
    })
}

/// Reads the document type and the tracks of a Matroska or WebM file.
/// The tracks are located before the media data (clusters) in practice.
fn probe_matroska(file: &mut File) -> io::Result<MediaInfo> {
    let (id, size) = read_element_header(file)?;
    if id != ebml::HEADER {
        return Err(invalid_data("missing EBML header"));
    }
    let header = read_element(file, size)?;
    let container = match ebml_elements(&header).find(|(id, _)| *id == ebml::DOC_TYPE) {
        Some((_, b"webm")) => Container::Webm,
        _ => Container::Matroska,
    };

    let (id, _) = read_element_header(file)?;
    if id != ebml::SEGMENT {
        return Err(invalid_data("missing segment"));
    }

    // The segment's children are read one by one, its size may be unknown
    loop {
        let (id, size) = read_element_header(file)?;
        match id {
            ebml::TRACKS => {
                let tracks = read_element(file, size)?;
                return Ok(parse_matroska_tracks(container, &tracks));
            }
            ebml::CLUSTER => return Err(invalid_data("tracks not found before the media data")),
            _ => {
                let size = size.ok_or_else(|| invalid_data("element of unknown size"))?;
                file.seek(SeekFrom::Current(size as i64))?;
            }
        }
    }
}

fn parse_matroska_tracks(container: Container, tracks: &[u8]) -> MediaInfo {
    let mut info = MediaInfo {
        container,
        video: None,
        audio: None,
    };

    for (_, entry) in ebml_elements(tracks).filter(|(id, _)| *id == ebml::TRACK_ENTRY) {
        let mut track_type = None;
        let mut codec_id = None;
        for (id, content) in ebml_elements(entry) {
            match id {
                ebml::TRACK_TYPE => track_type = content.last().copied(),
                ebml::CODEC_ID => codec_id = std::str::from_utf8(content).ok(),
                _ => {}
            }
        }

        let codec = codec_id.and_then(matroska_codec);
        match track_type {
            Some(1) => info.video = info.video.or(codec),
            Some(2) => info.audio = info.audio.or(codec),
            _ => {}
        }
    }

    info
}

/// Identifies a codec from a Matroska codec ID
fn matroska_codec(codec_id: &str) -> Option<Codec> {
    let codec_id = codec_id.trim_end_matches('\0');
    Some(match codec_id {
        "V_MPEG4/ISO/AVC" => Codec::H264,
        "V_MPEGH/ISO/HEVC" => Codec::Hevc,
        "V_VP8" => Codec::Vp8,
        "V_VP9" => Codec::Vp9,
        "V_AV1" => Codec::Av1,
        "V_MPEG4/ISO/ASP" | "V_MPEG4/ISO/SP" | "V_MPEG4/ISO/AP" => Codec::Mpeg4,
        "A_MPEG/L3" => Codec::Mp3,
        "A_OPUS" => Codec::Opus,
        "A_VORBIS" => Codec::Vorbis,
        "A_FLAC" => Codec::Flac,
        "A_AC3" => Codec::Ac3,
        "A_EAC3" => Codec::Eac3,
        "A_TRUEHD" => Codec::TrueHd,
        id if id.starts_with("A_AAC") => Codec::Aac,
        id if id.starts_with("A_DTS") => Codec::Dts,
        id if id.starts_with("A_PCM") => Codec::Pcm,
        _ => return None,
    })
}
