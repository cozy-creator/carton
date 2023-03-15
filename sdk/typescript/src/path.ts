import path from "path";
import fsp from "fs/promises";
import fs from "fs";

export async function getPathFiles(dirPath: string, ext: string) {
  const dir = await fsp.readdir(dirPath);
  let files: string[] = [];

  for (let i = 0; i < dir.length; i++) {
    let filePath = path.join(dirPath, dir[i]);
    const stat = await fsp.stat(filePath);

    if (stat.isFile()) {
      if (ext == path.extname(filePath)) {
        files.push(filePath);
      }
    } else {
      files = [...files, ...(await getPathFiles(filePath, ext))];
    }
  }

  return files;
}

export function getRootPath() {
  let rootPath = process.cwd();

  while (!fs.existsSync(path.join(rootPath, "Carton.toml"))) {
    let parentPath = path.dirname(rootPath);
    if (rootPath == parentPath) {
      throw new Error();
    }

    rootPath = parentPath;
  }

  if (!rootPath) {
    throw new Error();
  }

  return rootPath;
}
