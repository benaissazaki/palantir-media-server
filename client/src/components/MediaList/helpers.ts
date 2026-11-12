/* eslint-disable @typescript-eslint/no-non-null-assertion */
export type FileNode = {
  name: string,
  url: string,
  children: FileNode[]
};

/**
 * Joins a parent path and a segment without doubling the separator after a root such as '/'.
 */
function joinPath(parent: string, segment: string, pathSeparator: string): string {
  if (!parent) {
    return segment;
  }
  return parent.endsWith(pathSeparator) ? parent + segment : parent + pathSeparator + segment;
}

/**
 * Splits a path into segments, keeping the leading '/' of absolute Unix paths as the root segment.
 */
function splitPath(path: string): string[] {
  const segments = path.split(/[\\/]/);
  const root = segments[0] === '' ? ['/'] : [];
  return [...root, ...segments.filter(segment => segment !== '')];
}

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
      url: joinPath(tree.url, pathSegments[0], pathSeparator),
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

  paths.forEach(path => buildTreeFromPaths(tree, splitPath(path), path.includes('\\'))); // Build the tree by splitting each path into path segments and filling the tree.

  return tree;
}

export function getNodeIcon(node: FileNode): string {
  if (node.children.length != 0) {
    return '/icons/directory.svg';
  }

  const fileExtension = node.name.split('.').pop()?.toLowerCase();

  if (['mp4', 'mkv', 'avi'].includes(fileExtension!)) {
    return '/icons/video.svg';
  }

  if (['mp3', 'wav'].includes(fileExtension!)) {
    return '/icons/audio.svg';
  }

  return '/icons/media-not-found.svg';
}
