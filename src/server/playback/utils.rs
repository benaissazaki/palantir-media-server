use lazy_static::lazy_static;
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Mutex,
    time::SystemTime,
};

use super::probe::{self, Codec, Container, MediaInfo};

/// The cookie in which the browser stores the containers and codecs it can play
pub const CAPABILITIES_COOKIE: &str = "palantir_caps";

/// The containers and codecs a browser can play, as sent in the capabilities cookie
pub struct Capabilities(HashSet<String>);

impl Capabilities {
    /// Parses the cookie value, a list of tokens separated by '.' such as `mp4.webm.h264.aac`
    pub fn parse(value: &str) -> Self {
        Capabilities(
            value
                .split('.')
                .filter(|token| !token.is_empty())
                .map(|token| token.to_ascii_lowercase())
                .collect(),
        )
    }

    fn supports(&self, token: &str) -> bool {
        self.0.contains(token)
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct PlaybackSupport {
    pub playable: bool,
    pub reason: Option<String>,
}

impl PlaybackSupport {
    pub fn playable() -> Self {
        PlaybackSupport {
            playable: true,
            reason: None,
        }
    }

    fn not_playable(reason: String) -> Self {
        PlaybackSupport {
            playable: false,
            reason: Some(reason),
        }
    }
}

/// Codecs that can be stored in a WebM file, which is a subset of Matroska
const WEBM_CODECS: &[Codec] = &[
    Codec::Vp8,
    Codec::Vp9,
    Codec::Av1,
    Codec::Opus,
    Codec::Vorbis,
];

/// Tells whether a browser with the given capabilities can play a media file.
/// When something is unknown, the file is considered playable so that it isn't blocked
/// when it could have been played.
pub fn check_playback_support(info: &MediaInfo, capabilities: &Capabilities) -> PlaybackSupport {
    let codecs = || info.video.iter().chain(info.audio.iter());

    let container_supported = capabilities.supports(info.container.token())
        // Browsers that play WebM can play Matroska files that only use WebM codecs
        || (info.container == Container::Matroska
            && capabilities.supports(Container::Webm.token())
            && codecs().all(|codec| WEBM_CODECS.contains(codec)));

    if !container_supported {
        return PlaybackSupport::not_playable(format!(
            "{} files are not supported by this browser.",
            info.container.name()
        ));
    }

    if let Some(codec) = info.video.filter(|c| !capabilities.supports(c.token())) {
        return PlaybackSupport::not_playable(format!(
            "{} video is not supported by this browser.",
            codec.name()
        ));
    }

    if let Some(codec) = info.audio.filter(|c| !capabilities.supports(c.token())) {
        return PlaybackSupport::not_playable(format!(
            "{} audio is not supported by this browser.",
            codec.name()
        ));
    }

    PlaybackSupport::playable()
}

lazy_static! {
    /// Probe results by file, along with the file's modification time when it was probed
    static ref PROBE_CACHE: Mutex<HashMap<PathBuf, (SystemTime, Option<MediaInfo>)>> =
        Mutex::new(HashMap::new());
}

/// Probes a media file, reusing the previous result if the file hasn't been modified since
pub fn probe_cached(path: &Path) -> Option<MediaInfo> {
    let modified = std::fs::metadata(path).and_then(|m| m.modified()).ok();

    if let Some(modified) = modified {
        if let Some((cached_modified, info)) = PROBE_CACHE.lock().unwrap().get(path) {
            if *cached_modified == modified {
                return info.clone();
            }
        }
    }

    // The cache isn't locked while reading the file
    let info = probe::probe(path);
    if let Some(modified) = modified {
        PROBE_CACHE
            .lock()
            .unwrap()
            .insert(path.to_path_buf(), (modified, info.clone()));
    }
    info
}
