import { useEffect, useState } from 'react';
import TreeNode from './TreeNode';
import {
  FileNode, compressTree, constructFileTree, countFiles, findLibraryRoot,
} from './helpers';
import Loader from '../Loader';
import EmptyList from './EmptyList';
import style from './MediaList.module.css';

type MediaFilesResponse = {
  length: number,
  items: string[]
};

const MediaList = () => {
  const [mediaFilesTree, setMediaFilesTree] = useState<FileNode>({ name: '$root', url: '', children: [] });
  const [libraryPath, setLibraryPath] = useState<string>('');
  const [isLoading, setIsLoading] = useState<boolean>(true);

  const fetchMediaFilesList = async () => {
    try {
      const response = await fetch(`${import.meta.env.VITE_API_BASE_URL}/api/media`);
      const data: MediaFilesResponse = await response.json();
      const libraryRoot = findLibraryRoot(constructFileTree(data.items));
      setLibraryPath(libraryRoot.url);
      setMediaFilesTree(compressTree({ ...libraryRoot, name: '$root' }));
    } catch (error) {
      console.error('Error fetching media directories:', error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchMediaFilesList();
  }, []);

  if (isLoading) {
    return <Loader />;
  }

  if (mediaFilesTree.children.length === 0) {
    return <EmptyList />;
  }

  const filesCount = countFiles(mediaFilesTree);

  return (
    <section>
      <div className={style.heading}>
        <div className={style.titles}>
          <h1 className={style.title}>Library</h1>
          {libraryPath && <p className={style.path} title={libraryPath}>{libraryPath}</p>}
        </div>
        <p className={style.count}>{filesCount} {filesCount === 1 ? 'file' : 'files'}</p>
      </div>
      <div className={style.card}>
        <TreeNode tree={mediaFilesTree} />
      </div>
    </section>
  );
};

export default MediaList;
