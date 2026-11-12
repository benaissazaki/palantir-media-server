import { useEffect, useState } from 'react';
import { Link, useParams } from 'react-router-dom';
import VideoPlayer from '../../components/VideoPlayer';
import Loader from '../../components/Loader';
import { getMediaKind } from '../../components/MediaList/helpers';
import { AlertIcon, ArrowLeftIcon, AudioIcon, DownloadIcon } from '../../components/Icons';
import style from './MediaPage.module.css';

type MediaPageParams = {
  mediaFile: string,
};

type PlaybackSupport = {
  playable: boolean,
  reason: string | null,
};

type UnsupportedProps = {
  reason: string,
  mediaUrl: string,
  onTryAnyway?: () => void,
};

const Unsupported = ({ reason, mediaUrl, onTryAnyway }: UnsupportedProps) => (
  <div className={style.unsupported}>
    <AlertIcon width={36} height={36} className={style.unsupportedIcon} />
    <h2 className={style.unsupportedTitle}>This file can&apos;t be played in this browser</h2>
    <p className={style.unsupportedReason}>{reason}</p>
    <div className={style.actions}>
      {onTryAnyway && (
        <button type="button" className={style.secondaryButton} onClick={onTryAnyway}>
          Try anyway
        </button>
      )}
      <a href={mediaUrl} download className={style.primaryButton}>
        <DownloadIcon width={16} height={16} />
        Download
      </a>
    </div>
  </div>
);

const MediaPage = () => {
  const { mediaFile = '' } = useParams<MediaPageParams>();
  const [support, setSupport] = useState<PlaybackSupport | null>(null);
  const [tryAnyway, setTryAnyway] = useState(false);
  const [playbackFailed, setPlaybackFailed] = useState(false);

  const pathSeparator = mediaFile.includes('\\') ? '\\' : '/';
  const pathSegments = mediaFile.split(pathSeparator);
  const fileName = pathSegments.pop() ?? mediaFile;
  const parentDirectory = pathSegments.join(pathSeparator) || pathSeparator;

  const apiUrl = import.meta.env.VITE_API_BASE_URL;
  const mediaUrl = `${apiUrl}/api/media/${encodeURIComponent(mediaFile)}`;
  const mediaKind = getMediaKind(fileName);

  useEffect(() => {
    const checkPlaybackSupport = async () => {
      try {
        const response = await fetch(`${apiUrl}/api/playback/${encodeURIComponent(mediaFile)}`);
        if (!response.ok) {
          throw new Error(`Unexpected status ${response.status}`);
        }
        setSupport(await response.json());
      } catch (error) {
        // Let the browser try when the server can't tell
        console.error('Error checking playback support:', error);
        setSupport({ playable: true, reason: null });
      }
    };

    setSupport(null);
    setTryAnyway(false);
    setPlaybackFailed(false);
    checkPlaybackSupport();
  }, [mediaFile]);

  const onPlaybackError = () => setPlaybackFailed(true);

  let player;
  if (support === null) {
    player = <div className={style.checking}><Loader /></div>;
  } else if (playbackFailed) {
    player = (
      <Unsupported
        reason="The browser couldn't play this file. Its format or codecs may not be supported."
        mediaUrl={mediaUrl}
      />
    );
  } else if (!support.playable && !tryAnyway) {
    player = (
      <Unsupported
        reason={support.reason ?? 'Its format or codecs are not supported.'}
        mediaUrl={mediaUrl}
        onTryAnyway={() => setTryAnyway(true)}
      />
    );
  } else if (mediaKind === 'video') {
    player = <VideoPlayer mediaFile={mediaFile} onError={onPlaybackError} />;
  } else if (mediaKind === 'audio') {
    player = (
      <div className={style.audio}>
        <AudioIcon width={40} height={40} className={style.audioIcon} />
        <audio src={mediaUrl} controls autoPlay onError={onPlaybackError} />
      </div>
    );
  } else {
    player = <p className={style.unknown}>This format is not supported.</p>;
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
