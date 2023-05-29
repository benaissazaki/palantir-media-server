import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';

type MediaFilesResponse = {
  length: number,
  items: string[]
};

type FileNode = {
  name: string,
  url: string,
  children: FileNode[]
};

/**
 * Recursively fills the tree with FileNode objects based on the given path segments.
 * @param tree The current tree node to fill.
 * @param pathSegments An array of path segments representing a file path.
 */
function buildTreeFromPaths(tree: FileNode, pathSegments: string[], isWindowsPathSeparator: boolean) {
  if (pathSegments.length === 0) {
    return; // Terminate when all path segments have been processed.
  }

  const correspondingNode = tree.children.filter(c => c.name === pathSegments[0]);
  if (correspondingNode.length === 0) {
    const pathSeparator = isWindowsPathSeparator ? '\\' : '/';
    // If there is no matching child node, create a new FileNode and add it as a child to the current node.
    const newNode: FileNode = {
      name: pathSegments[0],
      url: `${tree.url && (tree.url + pathSeparator)}${pathSegments[0]}`,
      children: [],
    };
    tree.children.push(newNode);
    buildTreeFromPaths(newNode, pathSegments.slice(1), isWindowsPathSeparator); // Recursively fill the new node with remaining path segments.
  } else {
    buildTreeFromPaths(correspondingNode[0], pathSegments.slice(1), isWindowsPathSeparator);
  }

}

/**
 * Converts an array of file paths into a hierarchical tree structure represented by FileNode objects.
 * @param paths An array of file paths.
 * @returns The root FileNode representing the hierarchical tree structure.
 */
function constructFileTree(paths: string[]): FileNode {
  const tree: FileNode = {
    name: '$root',
    url: '',
    children: [],
  };

  paths.forEach(path => buildTreeFromPaths(tree, path.split(/[\\/]/), path.includes('\\'))); // Build the tree by splitting each path into path segments and filling the tree.

  return tree;
}

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
        <li>{tree.name}</li>
        {tree.children.map((child, index) => (
          <TreeNode key={index} tree={child} />
        ))}
      </ul>
    );
  }
  return (
    <ul>
      <li><Link to={`/media/${encodeURIComponent(tree.url)}`}>{tree.name}</Link></li>
      {tree.children.map((child, index) => (
        <TreeNode key={index} tree={child} />
      ))}
    </ul>
  );
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
