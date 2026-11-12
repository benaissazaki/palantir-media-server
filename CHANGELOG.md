# v1.1.1

## What's changed?
### Security fixes
- Files outside of the media directories can no longer be accessed using `..` in their path.
- Other websites can no longer call the server's API from your browser.
- The settings can only be changed from the machine running the server.
- The settings file and the executable are no longer served. The web interface is now located in a `client` directory next to the executable, so replace the whole folder when upgrading.

### Fixes
- Media files can now be played on Linux.
- SRT subtitles are converted to WebVTT so they can be displayed by the browser. Only `.srt` and `.vtt` subtitles are listed.
- Media files with uppercase extensions (e.g. `.MP4`) are now found.
- Entering an invalid port displays an error message instead of crashing the app.
- Audio files are displayed with an audio icon.

# v1.1.0

## What's changed?
### Add subtitles support
When playing a video, any subtitle file located in its parent directory can be used.

# v1.0.1

## What's changed?
Display an error message when the server fails to start instead of crashing the app 
