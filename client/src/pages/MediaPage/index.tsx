/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { useNavigate, useParams } from 'react-router-dom';
import VideoPlayer from '../../components/VideoPlayer';
import style from './MediaPage.module.css';

type MediaPageParams = {
  mediaFile: string,
};

const MediaPage = () => {
  const params = useParams<MediaPageParams>();
  const navigate = useNavigate();

  const fileExtension = params.mediaFile?.split('.').pop();

  let player = null;
  if (['mp4', 'mkv', 'avi'].includes(fileExtension!)) {
    player = <VideoPlayer mediaFile={params.mediaFile!} />;
  } else if (['mp3', 'wav'].includes(fileExtension!)) {
    player = <audio src={`${import.meta.env.VITE_API_BASE_URL}/api/media/${encodeURIComponent(params.mediaFile!)}`} controls />;
  } else {
    player = <div>Unsupported format</div>;
  }
  return (
    <>
      <button id={style.previous} onClick={() => navigate(-1)}>
        <img src="/icons/arrow-left.svg" alt="Previous" />
      </button>
      <div id={style.player_container}>
        {player}
      </div>

    </>
  );

};

export default MediaPage;
