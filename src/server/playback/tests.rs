#[cfg(test)]
mod tests {
    use crate::server::{
        app_settings::AppSettings,
        playback::{
            probe::{probe, Codec, Container, MediaInfo},
            routes::get_playback_support,
            utils::{check_playback_support, Capabilities, PlaybackSupport},
        },
    };
    use actix_web::{cookie::Cookie, test as actix_test, App};
    use std::{fs, path::Path};

    /// What a typical Chromium browser reports
    const CHROMIUM: &str = "mp4.webm.mkv.mp3.wav.h264.vp8.vp9.av1.aac.opus.vorbis.flac.pcm";

    const FIXTURES: &[(&str, &[u8])] = &[
        ("h264_aac.mp4", include_bytes!("fixtures/h264_aac.mp4")),
        ("hevc_aac.mp4", include_bytes!("fixtures/hevc_aac.mp4")),
        ("h264_mp3.mp4", include_bytes!("fixtures/h264_mp3.mp4")),
        ("h264_ac3.mkv", include_bytes!("fixtures/h264_ac3.mkv")),
        ("hevc_eac3.mkv", include_bytes!("fixtures/hevc_eac3.mkv")),
        ("vp9_opus.webm", include_bytes!("fixtures/vp9_opus.webm")),
        ("mpeg4_mp3.avi", include_bytes!("fixtures/mpeg4_mp3.avi")),
    ];

    fn info(container: Container, video: Option<Codec>, audio: Option<Codec>) -> MediaInfo {
        MediaInfo {
            container,
            video,
            audio,
        }
    }

    fn probe_fixture(dir: &str, name: &str) -> Option<MediaInfo> {
        let (_, bytes) = FIXTURES.iter().find(|(n, _)| *n == name).unwrap();
        fs::create_dir_all(dir).unwrap();
        let path = Path::new(dir).join(name);
        fs::write(&path, bytes).unwrap();
        let info = probe(&path);
        fs::remove_dir_all(dir).unwrap();
        info
    }

    /// Reads the codecs of MP4 files, whether the moov box is before or after the media data
    #[test]
    fn probe_reads_mp4_codecs() {
        let dir = "testdirs_probe_mp4";
        assert_eq!(
            probe_fixture(dir, "h264_aac.mp4"),
            Some(info(Container::Mp4, Some(Codec::H264), Some(Codec::Aac)))
        );
        assert_eq!(
            probe_fixture(dir, "hevc_aac.mp4"),
            Some(info(Container::Mp4, Some(Codec::Hevc), Some(Codec::Aac)))
        );
        assert_eq!(
            probe_fixture(dir, "h264_mp3.mp4"),
            Some(info(Container::Mp4, Some(Codec::H264), Some(Codec::Mp3)))
        );
    }

    /// Reads the codecs of Matroska and WebM files
    #[test]
    fn probe_reads_matroska_codecs() {
        let dir = "testdirs_probe_mkv";
        assert_eq!(
            probe_fixture(dir, "h264_ac3.mkv"),
            Some(info(
                Container::Matroska,
                Some(Codec::H264),
                Some(Codec::Ac3)
            ))
        );
        assert_eq!(
            probe_fixture(dir, "hevc_eac3.mkv"),
            Some(info(
                Container::Matroska,
                Some(Codec::Hevc),
                Some(Codec::Eac3)
            ))
        );
        assert_eq!(
            probe_fixture(dir, "vp9_opus.webm"),
            Some(info(Container::Webm, Some(Codec::Vp9), Some(Codec::Opus)))
        );
    }

    /// Identifies AVI files and audio files, and gives up on unknown content
    #[test]
    fn probe_identifies_other_files() {
        let dir = "testdirs_probe_other";
        assert_eq!(
            probe_fixture(dir, "mpeg4_mp3.avi"),
            Some(info(Container::Avi, None, None))
        );

        fs::create_dir_all(dir).unwrap();
        fs::write(Path::new(dir).join("song.MP3"), b"ID3").unwrap();
        fs::write(Path::new(dir).join("broken.mp4"), b"not a video").unwrap();
        let song = probe(&Path::new(dir).join("song.MP3"));
        let broken = probe(&Path::new(dir).join("broken.mp4"));
        let missing = probe(&Path::new(dir).join("missing.mkv"));
        fs::remove_dir_all(dir).unwrap();

        assert_eq!(song, Some(info(Container::Mp3, None, Some(Codec::Mp3))));
        assert_eq!(broken, None);
        assert_eq!(missing, None);
    }

    #[test]
    fn check_accepts_supported_files() {
        let chromium = Capabilities::parse(CHROMIUM);
        let supported = [
            info(Container::Mp4, Some(Codec::H264), Some(Codec::Aac)),
            info(Container::Webm, Some(Codec::Vp9), Some(Codec::Opus)),
            info(Container::Mp3, None, Some(Codec::Mp3)),
            // Unrecognized codecs don't block playback
            info(Container::Mp4, None, None),
        ];
        for media in supported {
            assert_eq!(
                check_playback_support(&media, &chromium),
                PlaybackSupport::playable()
            );
        }
    }

    #[test]
    fn check_rejects_unsupported_codecs_and_containers() {
        let chromium = Capabilities::parse(CHROMIUM);
        let reason = |media: MediaInfo| check_playback_support(&media, &chromium).reason;

        assert_eq!(
            reason(info(Container::Mp4, Some(Codec::Hevc), Some(Codec::Aac))),
            Some("HEVC (H.265) video is not supported by this browser.".to_string())
        );
        assert_eq!(
            reason(info(
                Container::Matroska,
                Some(Codec::H264),
                Some(Codec::Ac3)
            )),
            Some("Dolby Digital (AC-3) audio is not supported by this browser.".to_string())
        );
        assert_eq!(
            reason(info(Container::Avi, None, None)),
            Some("AVI files are not supported by this browser.".to_string())
        );
    }

    /// Browsers that don't report Matroska support can still play Matroska files with WebM codecs
    #[test]
    fn check_plays_matroska_with_webm_codecs() {
        let capabilities = Capabilities::parse("mp4.webm.h264.vp9.aac.opus");

        let webm_codecs = info(Container::Matroska, Some(Codec::Vp9), Some(Codec::Opus));
        let other_codecs = info(Container::Matroska, Some(Codec::H264), Some(Codec::Aac));

        assert!(check_playback_support(&webm_codecs, &capabilities).playable);
        assert!(!check_playback_support(&other_codecs, &capabilities).playable);
    }

    async fn get_support(file: &str, capabilities: Option<&str>) -> (u16, String) {
        let app = actix_test::init_service(App::new().service(get_playback_support)).await;
        let mut req = actix_test::TestRequest::get().uri(format!("/playback/{}", file).as_str());
        if let Some(capabilities) = capabilities {
            req = req.cookie(Cookie::new("palantir_caps", capabilities));
        }
        let res = actix_test::call_service(&app, req.to_request()).await;
        let status = res.status().as_u16();
        let body = actix_test::read_body(res).await;
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    /// Answers based on the capabilities cookie, and lets the browser try without it
    #[actix_web::test]
    async fn get_playback_support_uses_capabilities_cookie() {
        fs::create_dir_all("testdirs_playback/media_dir").unwrap();
        fs::write("testdirs_playback/media_dir/hevc.mp4", FIXTURES[1].1).unwrap();
        fs::write("testdirs_playback/secret.mp4", FIXTURES[1].1).unwrap();
        {
            let mut settings = AppSettings::instance().lock().unwrap();
            settings.media_directories = vec!["testdirs_playback/media_dir".to_string()];
            settings.save().unwrap();
        }

        let file = "testdirs_playback%2Fmedia_dir%2Fhevc.mp4";
        let with_hevc = get_support(file, Some(&format!("{}.hevc", CHROMIUM))).await;
        let without_hevc = get_support(file, Some(CHROMIUM)).await;
        let without_cookie = get_support(file, None).await;
        let outside = get_support("testdirs_playback%2Fsecret.mp4", Some(CHROMIUM)).await;

        fs::remove_dir_all("testdirs_playback").unwrap();

        assert_eq!(
            with_hevc,
            (200, r#"{"playable":true,"reason":null}"#.to_string())
        );
        assert_eq!(
            without_hevc,
            (
                200,
                r#"{"playable":false,"reason":"HEVC (H.265) video is not supported by this browser."}"#
                    .to_string()
            )
        );
        assert_eq!(
            without_cookie,
            (200, r#"{"playable":true,"reason":null}"#.to_string())
        );
        assert_eq!(outside.0, 404);
    }
}
