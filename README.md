# Palantir Media Server
Palantir Media Server is a lightweight and efficient media server that allows you to stream and share your media files across devices. It provides a simple and intuitive web interface for accessing your media library.

## Targeted features
- **Media streaming**: Stream your media files (videos, music, and photos) directly from the server to any supported device.
- **Media indexing**: Automatically index and categorize your media files for quick and efficient searching.
- **Web interface**: Access your media library through an easy-to-use web interface.
- **User authentication**: Secure your media server by enabling user authentication to control access to your media files.

## How to use

1. Download the latest release [here](https://github.com/benaissazaki/palantir-media-server/releases/latest).
2. Specify the directories containing the media files that you want to share in the `app_settings.json` file. E.g: 
```
{
  "media_directories": ["D:\\path\\to\\my\\media_directory"]
}
```

3. Launch `palantir-media-server.exe` (`palantir-media-server` on Linux):
4. Specify the host and port then click on 'Start server':

![image](https://github.com/benaissazaki/palantir-media-server/assets/33980130/5865d513-4cf6-4ae8-b8e8-1bd1a3a8c4e9)

6. You can now access the application via browser at the displayed address:

![image](https://github.com/benaissazaki/palantir-media-server/assets/33980130/55cc3460-00d3-412b-997c-9384b21d4dc0)


## Advancements

Currently, the program crawls the media directories specified in `app_settings.json` in search of video and audio files so that the user can access them via a simple web interface.

In the future, the following points will be addressed:

- **Compatibility:** Since the program currently serves the media file as it is, if your browser does not support its format, you won't be able to access it.
- **User authentication**.
- **A more user friendly way to configure the server.**
- **Subtitles for videos**
