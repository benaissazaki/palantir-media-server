import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';

type MediaFilesResponse = {
  length: number,
  items: string[]
};

const MediaList = () => {
  const [mediaFiles, setMediaFiles] = useState<string[]>([]);

  const fetchMediaFilesList = async () => {
    try {
      const response = await fetch(`${import.meta.env.VITE_API_BASE_URL}/api/media`);
      const data: MediaFilesResponse = await response.json();
      setMediaFiles(data.items);
    } catch (error) {
      console.error('Error fetching media directories:', error);
    }
  };

  useEffect(() => {
    fetchMediaFilesList();
  }, []);

  return (
    <ul>
      {mediaFiles.map(file => (
        <li key={file}><Link to={`/media/${encodeURIComponent(file)}`}>{file}</Link></li>
      ))}
    </ul>
  );
};

export default MediaList;
