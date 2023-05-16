import { useEffect, useState } from 'react';

type MediaFilesResponse = {
  length: number,
  items: string[]
};

export const HomePage = () => {
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
    <>
      <h1>Home</h1>
      <ul>
        {mediaFiles.map(file => (
          <li key={file}>{file}</li>
        ))}
      </ul>
    </>
  );
}; 
