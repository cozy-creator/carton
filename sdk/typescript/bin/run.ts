#!/usr/bin/env node

import { parseArgs } from "../src/parser";
import * as tsNode from "ts-node";
import path from "path";

interface Arg {
  file: string;
}

const args = parseArgs<Arg>(process.argv);
if (path.extname(args.file) == ".ts") {
  tsNode.register();
}

require(args.file);
