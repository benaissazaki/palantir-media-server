import { Link } from 'react-router-dom';
import { FileNode, getNodeIcon } from './helpers';
import style from './TreeNode.module.css';

interface TreeProps {
  tree: FileNode;
}

const TreeNode = ({ tree }: TreeProps) => {
  // Don't display root node
  if (tree.name === '$root') {
    return (
      <>
        {tree.children.map((child, index) => (
          <TreeNode key={index} tree={child} />
        ))}
      </>
    );
  }

  // Don't display links for directories
  if (tree.children.length !== 0) {
    return (
      <ul className={style.list}>
        <li>
          <details>
            <summary>
              <div className={style.file}>
                <img src={getNodeIcon(tree)} />
                {tree.name}
              </div>
            </summary>
            {tree.children.map((child, index) => (
              <TreeNode key={index} tree={child} />
            ))}
          </details>
        </li>
      </ul>
    );
  }
  return (
    <ul className={style.list}>
      <li>
        <div className={style.file}>
          <img src={getNodeIcon(tree)} />
          <Link to={`/media/${encodeURIComponent(tree.url)}`}>{tree.name}</Link>
        </div>
      </li>
    </ul>
  );
};

export default TreeNode;
