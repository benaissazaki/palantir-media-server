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

export type MediaKind = 'video' | 'audio' | 'unknown';

/**
 * Returns the kind of media of a file based on its extension.
 */
export function getMediaKind(fileName: string): MediaKind {
  const fileExtension = fileName.split('.').pop()?.toLowerCase() ?? '';

  if (['mp4', 'mkv', 'avi'].includes(fileExtension)) {
    return 'video';
  }

  if (['mp3', 'wav'].includes(fileExtension)) {
    return 'audio';
  }

  return 'unknown';
}

export function isDirectory(node: FileNode): boolean {
  return node.children.length !== 0;
}

/**
 * Sorts directories before files, then by name with numbers in natural order (2 before 10).
 */
function compareNodes(a: FileNode, b: FileNode): number {
  if (isDirectory(a) !== isDirectory(b)) {
    return isDirectory(a) ? -1 : 1;
  }
  return a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' });
}

/**
 * Merges directories that only contain a single directory into one node
 * so that e.g. '/' > 'home' > 'user' > 'Movies' is displayed as '/home/user/Movies'.
 */
export function compressTree(node: FileNode): FileNode {
  let current = node;
  let name = node.name;

  while (current.name !== '$root' && current.children.length === 1 && isDirectory(current.children[0])) {
    const child = current.children[0];
    const pathSeparator = child.url.includes('\\') ? '\\' : '/';
    name = name.endsWith(pathSeparator) ? name + child.name : name + pathSeparator + child.name;
    current = child;
  }

  return {
    name,
    url: current.url,
    children: current.children.map(compressTree).sort(compareNodes),
  };
}

/**
 * Returns the deepest directory containing all the files, so that the tree
 * starts at the media directories instead of the filesystem's root.
 */
export function findLibraryRoot(tree: FileNode): FileNode {
  let current = tree;
  while (current.children.length === 1 && isDirectory(current.children[0])) {
    [current] = current.children;
  }
  return current;
}

/**
 * Counts the files (leaf nodes) in the tree.
 */
export function countFiles(node: FileNode): number {
  if (!isDirectory(node)) {
    return node.name === '$root' ? 0 : 1;
  }
  return node.children.reduce((count, child) => count + countFiles(child), 0);
}
