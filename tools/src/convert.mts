import { convertJsonSchemaToJTD } from "./lib/convertJsonSchemaToJTD.js";
import path from "path";

console.log("hi");

convertJsonSchemaToJTD({
  jsonschemaRelativePath: "../specs/access-control.jsonschema.json",
  outputDir: "../specs",
});
