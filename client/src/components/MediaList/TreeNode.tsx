import { Link } from 'react-router-dom';
import { FileNode, getMediaKind, isDirectory } from './helpers';
import { AudioIcon, ChevronIcon, FileIcon, FolderIcon, VideoIcon } from '../Icons';
import style from './TreeNode.module.css';

interface TreeProps {
  tree: FileNode;
  depth?: number;
}

const FileKindIcon = ({ name }: { name: string }) => {
  switch (getMediaKind(name)) {
    case 'video':
      return <VideoIcon className={style.icon} />;
    case 'audio':
      return <AudioIcon className={style.icon} />;
    default:
      return <FileIcon className={style.icon} />;
  }
};

const TreeNode = ({ tree, depth = 0 }: TreeProps) => {
  // Don't display root node
  if (tree.name === '$root') {
    return (
      <ul className={style.list}>
        {tree.children.map((child) => (
          <TreeNode key={child.url} tree={child} depth={depth} />
        ))}
      </ul>
    );
  }

  if (isDirectory(tree)) {
    return (
      <li>
        <details className={style.directory} open={depth === 0}>
          <summary className={style.row}>
            <ChevronIcon className={style.chevron} width={16} height={16} />
            <FolderIcon className={style.icon} />
            <span className={style.name}>{tree.name}</span>
          </summary>
          <ul className={`${style.list} ${style.children}`}>
            {tree.children.map((child) => (
              <TreeNode key={child.url} tree={child} depth={depth + 1} />
            ))}
          </ul>
        </details>
      </li>
    );
  }

  return (
    <li>
      <Link className={`${style.row} ${style.file}`} to={`/media/${encodeURIComponent(tree.url)}`}>
        <FileKindIcon name={tree.name} />
        <span className={style.name}>{tree.name}</span>
      </Link>
    </li>
  );
};

export default TreeNode;
