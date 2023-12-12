import path from "node:path";
import fs from "node:fs";
// import { fileURLToPath } from "node:url";

// const __filename = fileURLToPath(import.meta.url);
// const __dirname = path.dirname(__filename);

const isEntrypointValid = (entrypoint) =>
  fs.existsSync(entrypoint) && fs.readdirSync(entrypoint).length > 0;

function getJsonFile(entrypoint, filename) {
  const filePath = path.resolve(entrypoint, filename + ".json");

  return JSON.parse(fs.readFileSync(filePath, { encoding: "utf-8" }));
}

const isPlaceholderFn = (path) => path.node.callee.name === "constant";

function wrap(str) {
  return '"' + str + '"';
}

function InlineConstantsPlugin() {
  return {
    visitor: {
      // replace <file.json>.<key1>.<key2> to literal-string constant
      CallExpression(path, { opts }) {
        const entrypoint = opts.constants;

        if (!isEntrypointValid(entrypoint)) {
          throw new Error("`constants` path is empty");
        }

        if (isPlaceholderFn(path)) {
          const constantPath = path.node.arguments[0].value;
          const keys = path.node.arguments[0].value
            .split(".")
            .map((str) => str.trim());

          const file = keys.shift();

          const constants = getJsonFile(entrypoint, file);
          let temp = constants;

          for (const key of keys) {
            temp = temp[key];
          }

          const strToReplace = temp;

          if (!strToReplace) {
            throw new Error(`${constantPath} not found!`);
          }

          path.replaceWithSourceString(wrap(strToReplace));
        }
      },
      // purge import of noop function
      ImportDeclaration(path) {
        if (path.node.source.value.includes("plugin/constant")) {
          path.remove();
        }
      },
    },
  };
}

export default InlineConstantsPlugin;
