import { Link } from 'react-router-dom';
import { FileNode } from './helpers';

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
      <ul>
        <li>
          <details>
            <summary>{tree.name}</summary>
            {tree.children.map((child, index) => (
              <TreeNode key={index} tree={child} />
            ))}
          </details>
        </li>
      </ul>
    );
  }
  return (
    <ul>
      <li>
        <Link to={`/media/${encodeURIComponent(tree.url)}`}>{tree.name}</Link>
      </li>
    </ul>
  );
};

export default TreeNode;
