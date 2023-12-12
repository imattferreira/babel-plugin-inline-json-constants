import path from "node:path";
import fs from "node:fs";

const CACHE = new Map();
const AVAILABLE_INJECTION_TYPES = ["str", "number", "number", "bool"];
let entrypointAlreadyChecked = false;

const isEntrypointValid = (entrypoint) =>
  fs.existsSync(entrypoint) && fs.readdirSync(entrypoint).length > 0;

const isObj = (obj) =>
  typeof obj === "object" && !Array.isArray(obj) && obj !== null;

function getJsonFile(entrypoint, filename) {
  if (CACHE.has(filename)) {
    return JSON.parse(CACHE.get(filename));
  }

  const filePath = path.resolve(entrypoint, filename + ".json");
  const fileExists = fs.existsSync(filePath);

  if (!fileExists) {
    CACHE.set(filename, null);

    return null;
  }

  const content = fs.readFileSync(filePath, { encoding: "utf-8" }) || null;

  CACHE.set(filename, content);

  return JSON.parse(content);
}

function wrap(str) {
  return '"' + str + '"';
}

function InlineConstantsPlugin() {
  return {
    visitor: {
      // replace <file.json>.<key1>.<key2> to literal-string constant
      CallExpression(path, { opts }) {
        const entrypoint = opts.constants;

        if (!entrypointAlreadyChecked) {
          if (!isEntrypointValid(entrypoint)) {
            throw new Error("`constants` path is invalid!");
          }

          entrypointAlreadyChecked = true;
        }

        const objectName = path.node.callee.object?.name;
        const propertyName = path.node.callee.property?.name;
        const firstArgument = path.node.arguments[0]?.value;

        if (objectName !== "constant") {
          return;
        }

        if (!AVAILABLE_INJECTION_TYPES.includes(propertyName)) {
          throw new Error("invalid property type!");
        }

        if (!firstArgument || typeof firstArgument !== "string") {
          throw new Error("first argument should be an string!");
        }

        const keys = firstArgument.split(".").map((str) => str.trim());
        const file = keys.shift();
        const availableConstants = getJsonFile(entrypoint, file);

        if (!availableConstants) {
          throw new Error(`file ${file} is empty!`);
        }

        let temp = availableConstants;

        for (const key of keys) {
          console.log(temp);
          temp = temp[key] || null;
        }

        const valueToReplace = temp;

        if (!valueToReplace) {
          throw new Error("constant not found!");
        }

        // TODO fix bugs, only str works
        switch (propertyName) {
          case "str":
            if (typeof valueToReplace !== "string") {
              throw new Error(`invalid typeof ${valueToReplace}, expected str`);
            }

            path.replaceWithSourceString(wrap(valueToReplace));
            break;
          case "enum":
            if (!isObj(valueToReplace)) {
              throw new Error(
                `invalid typeof ${valueToReplace}, expected enum`
              );
            }

            path.replaceWith(valueToReplace);
            break;
          case "number":
            if (typeof valueToReplace !== "number") {
              throw new Error(
                `invalid typeof ${valueToReplace}, expected number`
              );
            }

            console.log({ valueToReplace });
            path.replaceWith(valueToReplace);
            break;
          case "bool":
            if (typeof valueToReplace !== "boolean") {
              throw new Error(
                `invalid typeof ${valueToReplace}, expected bool`
              );
            }

            path.replace(valueToReplace);
            break;

          default:
            // do nothing
            break;
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
