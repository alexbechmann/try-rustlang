import { $RefParser, dereference } from "@apidevtools/json-schema-ref-parser";
import path from "path";
import fs from "fs";
import type { JSONSchema4, JSONSchema4Object, JSONSchema6, JSONSchema6Object, JSONSchema7, JSONSchema7Object } from "json-schema";
import { cloudEventProperties } from "./cloudEventProperties.js";

// type JSONSchema = JSONSchema4 | JSONSchema6 | JSONSchema7;
type JSONSchema = NonNullable<$RefParser["schema"]>;

// type JSONProperties = {
//     [k: string]: JSONSchema4;
// }

// const x: Required

// type JSONProperties = NonNullable<JSONSchema4Object["properties"] | JSONSchema6Object["properties"] | JSONSchema7Object["properties"]>;

export async function convertJsonSchemaToJTD({ jsonschemaRelativePath, outputDir }: { jsonschemaRelativePath: string; outputDir: string }) {
  const schemaPath = path.resolve(process.cwd(), jsonschemaRelativePath);
  let schema = await $RefParser.dereference(schemaPath);
  const jtdSchema: { discriminator: string; mapping: { [key: string]: any } } = {
    discriminator: "type",
    mapping: {},
  };

  for (const [key, value] of Object.entries(schema.definitions!)) {
    const required: string[] = value.properties.data.required || [];
    jtdSchema.mapping[key] = {
      properties: {
        //   type: { enum: [key] },
        ...cloudEventProperties,
        data: {
          properties:
            Object.entries(value.properties.data.properties)
              .filter(([k, v]) => required.includes(k))
              .reduce((acc, [k, v]) => {
                console.log("ab", { k, v });
                return { ...acc, [k]: v };
              }, {}) || {},
          optionalProperties:
            Object.entries(value.properties.data.properties)
              .filter(([k, v]) => !required.includes(k))
              .reduce((acc, [k, v]) => {
                console.log("abc", { k, v });
                return { ...acc, [k]: v };
              }, {}) || {},
        },
      },
    };

    console.log(JSON.stringify(jtdSchema, null, 2));
    // console.log(
    //   JSON.stringify(
    //     {
    //       properties: {
    //         key,
    //         value: value.properties.data.properties,
    //       },
    //       optionalProperties: {
    //         key,
    //         value: value.properties.data.optionalProperties,
    //       },
    //     },
    //     null,
    //     2
    //   )
    // );
    fixProperties({ properties: jtdSchema.mapping[key].properties.data.properties });
    fixProperties({ properties: jtdSchema.mapping[key].properties.data.optionalProperties });
  }

  const originalFileName = path.basename(schemaPath, ".json");
  const outputPath = path.resolve(process.cwd(), outputDir, `${originalFileName}.jtd.json`);

  fs.writeFileSync(outputPath, JSON.stringify(jtdSchema, null, 2));
}

function fixProperties({ properties }: { properties: any }) {
  console.log({ properties });
  for (const [propertyKey, propertyValue] of Object.entries(properties) as any) {
    if (propertyValue.properties) {
      fixProperties({ properties: propertyValue.properties });
    }
    fixProperty(propertyValue);
  }
}

function fixProperty(propertyValue: any) {
  if (propertyValue.$ref) {
    if (propertyValue.$ref.indexOf("timedef") > -1) {
      propertyValue.type = "timestamp";
    }
    delete propertyValue.$ref;
  }
  if (propertyValue.enum) {
    delete propertyValue.type;
  }
  if (Array.isArray(propertyValue.type)) {
    propertyValue.type = propertyValue.type[0];
  }
  delete propertyValue.$schema;
  delete propertyValue.title;
  const allowedKeys = ["type", "enum"];
  for (const [key, value] of Object.entries(propertyValue) as any) {
    if (!allowedKeys.includes(key)) {
      delete propertyValue[key];
    }
  }
}
