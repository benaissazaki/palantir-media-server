# v1.2.0

## What's changed?
### Configure media directories from the app
Media directories can now be added with the "Add directory" button and removed from the list, without editing `app_settings.json` by hand. Changes are saved immediately.

### New look
- The desktop app has been redesigned with a dark theme, a server status indicator and the address to open in the browser.
- The web interface has been redesigned: the library is displayed as a sorted folder tree starting at your media directories, and media pages have a cleaner player with the file name and its folder.
- Videos start playing as soon as they are opened.

### Fixes
- The settings file is now found when the app is launched from another directory.

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
