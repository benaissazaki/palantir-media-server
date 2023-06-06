import { useEffect, useState } from 'react';
import TreeNode from './TreeNode';
import { FileNode, constructFileTree } from './helpers';

type MediaFilesResponse = {
  length: number,
  items: string[]
};

const MediaList = () => {
  const [mediaFilesTree, setMediaFilesTree] = useState<FileNode>({ name: '$root', url: '', children: [] });

  const fetchMediaFilesList = async () => {
    try {
      const response = await fetch(`${import.meta.env.VITE_API_BASE_URL}/api/media`);
      const data: MediaFilesResponse = await response.json();
      setMediaFilesTree(constructFileTree(data.items));
    } catch (error) {
      console.error('Error fetching media directories:', error);
    }
  };

  useEffect(() => {
    fetchMediaFilesList();
  }, []);

  return (
    <TreeNode tree={mediaFilesTree} />
  );
};

export default MediaList;
