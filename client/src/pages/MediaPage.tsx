/* eslint-disable @typescript-eslint/no-non-null-assertion */
import { useParams } from 'react-router-dom';
import VideoPlayer from '../components/VideoPlayer';

type MediaPageParams = {
  mediaFile: string,
};

const MediaPage = () => {
  const params = useParams<MediaPageParams>();

  const fileExtension = params.mediaFile?.split('.').pop();
  if (['mp4', 'mkv', 'avi'].includes(fileExtension!)) {
    return (
      <VideoPlayer mediaFile={params.mediaFile!} />
    );
  }

  if (['mp3', 'wav'].includes(fileExtension!)) {
    return (
      <audio src={`${import.meta.env.VITE_API_BASE_URL}/api/media/${encodeURIComponent(params.mediaFile!)}`} controls/>
    );
  }

  return (
    <div>Unsupported format</div>
  );
};

export default MediaPage;
