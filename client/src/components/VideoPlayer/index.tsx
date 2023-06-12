import style from './VideoPlayer.module.css';

type VideoPlayerProps = {
  mediaFile: string,
};

const VideoPlayer = ({ mediaFile }: VideoPlayerProps) => {
  return (
    <video className={style.video} src={`${import.meta.env.VITE_API_BASE_URL}/api/media/${encodeURIComponent(mediaFile)}`} controls />
  );
};


export default VideoPlayer;
