/**
 * Containers and codecs the server can find in media files, along with the type
 * used to ask the browser whether it can play them.
 * The names must match the tokens used by the server (src/server/playback/probe.rs).
 */
const MEDIA_TYPES: Record<string, string> = {
  // Containers
  mp4: 'video/mp4',
  webm: 'video/webm',
  mkv: 'video/x-matroska',
  avi: 'video/x-msvideo',
  mp3: 'audio/mpeg',
  wav: 'audio/wav',
  // Video codecs
  h264: 'video/mp4; codecs="avc1.640028"',
  hevc: 'video/mp4; codecs="hvc1.1.6.L120.90"',
  vp8: 'video/webm; codecs="vp8"',
  vp9: 'video/webm; codecs="vp9"',
  av1: 'video/mp4; codecs="av01.0.08M.08"',
  mpeg4: 'video/mp4; codecs="mp4v.20.9"',
  // Audio codecs
  aac: 'audio/mp4; codecs="mp4a.40.2"',
  opus: 'audio/webm; codecs="opus"',
  vorbis: 'audio/webm; codecs="vorbis"',
  flac: 'audio/flac',
  ac3: 'audio/mp4; codecs="ac-3"',
  eac3: 'audio/mp4; codecs="ec-3"',
  dts: 'audio/mp4; codecs="dtsc"',
  truehd: 'audio/mp4; codecs="mlpa"',
  pcm: 'audio/wav; codecs="1"',
};

const CAPABILITIES_COOKIE = 'palantir_caps';

/**
 * Lists the containers and codecs this browser can play.
 */
export function detectCapabilities(): string[] {
  const video = document.createElement('video');
  return Object.entries(MEDIA_TYPES)
    .filter(([, type]) => video.canPlayType(type) !== '')
    .map(([name]) => name);
}

/**
 * Stores the browser's capabilities in a cookie so the server can tell which media files
 * it can play. They are detected on every visit, so they stay accurate after a browser update.
 */
export function saveCapabilities() {
  const capabilities = detectCapabilities().join('.');
  document.cookie = `${CAPABILITIES_COOKIE}=${capabilities}; path=/; max-age=31536000; SameSite=Strict`;
}
