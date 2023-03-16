import Mocha from "mocha";
import { getPathFiles } from "../path";
import { Reporter } from "./reporter";

const mocha = new Mocha();

export async function runTests(path: string) {
  const files = await getPathFiles(path, [".js"]);
  for (let i = 0; i < files.length; i++) {
    mocha.addFile(files[i]);
  }

  return mocha.reporter(Reporter).run();
}
