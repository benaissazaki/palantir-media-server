import { Link, useParams } from 'react-router-dom';
import VideoPlayer from '../../components/VideoPlayer';
import { getMediaKind } from '../../components/MediaList/helpers';
import { ArrowLeftIcon, AudioIcon } from '../../components/Icons';
import style from './MediaPage.module.css';

type MediaPageParams = {
  mediaFile: string,
};

const MediaPage = () => {
  const { mediaFile = '' } = useParams<MediaPageParams>();

  const pathSeparator = mediaFile.includes('\\') ? '\\' : '/';
  const pathSegments = mediaFile.split(pathSeparator);
  const fileName = pathSegments.pop() ?? mediaFile;
  const parentDirectory = pathSegments.join(pathSeparator) || pathSeparator;

  const mediaUrl = `${import.meta.env.VITE_API_BASE_URL}/api/media/${encodeURIComponent(mediaFile)}`;

  let player;
  switch (getMediaKind(fileName)) {
    case 'video':
      player = <VideoPlayer mediaFile={mediaFile} />;
      break;
    case 'audio':
      player = (
        <div className={style.audio}>
          <AudioIcon width={40} height={40} className={style.audioIcon} />
          <audio src={mediaUrl} controls autoPlay />
        </div>
      );
      break;
    default:
      player = <p className={style.unsupported}>This format is not supported.</p>;
  }

  return (
    <article>
      <Link to="/" className={style.back}>
        <ArrowLeftIcon width={16} height={16} />
        Library
      </Link>
      <div className={style.player}>
        {player}
      </div>
      <h1 className={style.title}>{fileName}</h1>
      <p className={style.path}>{parentDirectory}</p>
    </article>
  );
};

export default MediaPage;
