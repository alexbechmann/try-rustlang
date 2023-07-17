const schema = require("./specs/access-control.jsonschema.json");
const cloudEventSchema = require("./specs/base_cloudevent_v1.0.1.json");
const path = require("path");
const fs = require("fs");

// for each key,value in schema.definitions

const cloudEventProperties = {
  id: {
    type: "string",
  },
  source: {
    type: "string",
  },
  specversion: {
    type: "string",
  },
  datacontenttype: {
    type: "string",
  },
  dataschema: {
    type: "string",
  },
  subject: {
    type: "string",
  },
  time: {
    type: "string",
  },
  data_base64: {
    type: "string",
  },
};

const jtdSchema = {
  discriminator: "type",
  mapping: {},
};

for (const [key, value] of Object.entries(schema.definitions)) {
  jtdSchema.mapping[key] = {
    properties: {
      //   type: { enum: [key] },
      ...cloudEventProperties,
      data: {
        properties: value.properties.data.properties,
      },
    },
  };

  fixProperties(value.properties.data.properties);
}

function fixProperties(properties) {
  for (const [key2, value2] of Object.entries(properties)) {
    if (value2.properties) {
      fixProperties(value2.properties);
    }
    if (value2.$ref) {
      if (value2.$ref.indexOf("timedef") > -1) {
        // value2.type = "timestamp";
        value2.type = "string";
      }
      delete value2.$ref;
    }
    if (value2.enum) {
      delete value2.type;
    }
  }
}

fs.writeFileSync(path.resolve(__dirname, "specs", "access-control.jtd.json"), JSON.stringify(jtdSchema, null, 2));
