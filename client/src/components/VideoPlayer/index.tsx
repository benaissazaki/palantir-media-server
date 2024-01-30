import { useEffect, useState } from 'react';
import style from './VideoPlayer.module.css';

type VideoPlayerProps = {
  mediaFile: string;
};

type SubtitlesResponse = {
  length: number,
  items: string[]
};

const VideoPlayer = ({ mediaFile }: VideoPlayerProps) => {
  const pathSeparator = mediaFile.includes('\\') ? '\\' : '/';

  const parentDirectory = mediaFile.split(pathSeparator).slice(0, -1).join(pathSeparator);

  const [subtitlesList, setSubtitlesList] = useState<string[]>([]);

  const fetchSubtitlesList = async () => {
    console.log(parentDirectory);
    try {
      const response = await fetch(`${import.meta.env.VITE_API_BASE_URL}/api/subtitles/${encodeURIComponent(parentDirectory)}`);
      const data: SubtitlesResponse = await response.json();
      setSubtitlesList(data.items);
    } catch (error) {
      console.error('Error fetching subtitles:', error);
    }
  };

  useEffect(() => {
    fetchSubtitlesList();
  }, []);

  return (
    <video
      className={style.video}
      src={`${import.meta.env.VITE_API_BASE_URL}/api/media/${encodeURIComponent(
        mediaFile,
      )}`}
      controls
    >
      {subtitlesList.map((subtitleTrack) => (
        <track
          key={subtitleTrack}
          label={subtitleTrack.split(pathSeparator).pop()}
          src={`${import.meta.env.VITE_API_BASE_URL}/api/media/${encodeURIComponent(
            subtitleTrack,
          )}`} />
      ))}
    </video>
  );
};

export default VideoPlayer;
