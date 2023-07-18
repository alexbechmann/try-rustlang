import { convertJsonSchemaToJTD } from "./lib/convertJsonSchemaToJTD.js";
import path from "path";

convertJsonSchemaToJTD({
  jsonschemaRelativePath: "../specs/access-control.jsonschema.json",
  outputDir: "../specs",
});
