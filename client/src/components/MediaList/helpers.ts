export type FileNode = {
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
export function constructFileTree(paths: string[]): FileNode {
  const tree: FileNode = {
    name: '$root',
    url: '',
    children: [],
  };

  paths.forEach(path => buildTreeFromPaths(tree, path.split(/[\\/]/), path.includes('\\'))); // Build the tree by splitting each path into path segments and filling the tree.

  return tree;
}

