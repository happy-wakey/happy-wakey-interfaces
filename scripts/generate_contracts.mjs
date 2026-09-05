import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";

import Ajv2020 from "ajv/dist/2020.js";
import standaloneCode from "ajv/dist/standalone/index.js";
import addFormats from "ajv-formats";
import openapiTS, { astToString } from "openapi-typescript";
import { parse as parseYaml } from "yaml";

const root = resolve(import.meta.dirname, "..");
const typeSpecPath = resolve(root, "typespec/main.tsp");
const jsonSchemaPath = resolve(root, "schemas/morning-briefing.schema.json");
const openApiPath = resolve(root, "generated/typespec/openapi/openapi.yaml");
const typeSpecOutput = resolve(root, "generated/typespec");
const jsonSchemaOutput = resolve(root, "generated/json-schema");

await Promise.all([mkdir(typeSpecOutput, { recursive: true }), mkdir(jsonSchemaOutput, { recursive: true })]);

const [typeSpecSource, jsonSchemaSource, openApiSource] = await Promise.all([
  readFile(typeSpecPath, "utf8"),
  readFile(jsonSchemaPath, "utf8"),
  readFile(openApiPath, "utf8"),
]);
const jsonSchema = JSON.parse(jsonSchemaSource);
const openApi = parseYaml(openApiSource);

function rewriteOpenApiRefs(value) {
  if (Array.isArray(value)) return value.map(rewriteOpenApiRefs);
  if (value === null || typeof value !== "object") return value;
  const rewritten = Object.fromEntries(
    Object.entries(value).map(([key, child]) => {
      if (key === "$ref" && typeof child === "string") {
        return [key, child.replace(/^#\/components\/schemas\//, "#/$defs/")];
      }
      return [key, rewriteOpenApiRefs(child)];
    }),
  );
  if (rewritten.format === "uint32") {
    delete rewritten.format;
    rewritten.minimum ??= 0;
    rewritten.maximum ??= 4_294_967_295;
  } else if (rewritten.format === "uint64") {
    delete rewritten.format;
    rewritten.minimum ??= 0;
    // JSON consumers must not accept integers that JavaScript cannot represent exactly.
    rewritten.maximum ??= Number.MAX_SAFE_INTEGER;
  }
  return rewritten;
}

function createValidator(schema) {
  const ajv = new Ajv2020({
    allErrors: true,
    code: { source: true, esm: false },
    strict: false,
  });
  addFormats(ajv);
  return standaloneCode(ajv, ajv.compile(schema));
}

const typeSpecSchemas = rewriteOpenApiRefs(openApi.components.schemas);
const typeSpecRoot = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  $id: "https://interfaces.hawky.pro/generated/typespec/morning-briefing.schema.json",
  ...typeSpecSchemas.MorningBriefing,
  $defs: typeSpecSchemas,
};

const typeSpecAst = await openapiTS(openApi, { alphabetize: true });

function schemaType(schema) {
  if (schema.$ref) return schema.$ref.split("/").at(-1);
  if (Object.hasOwn(schema, "const")) return JSON.stringify(schema.const);
  if (schema.enum) return schema.enum.map((value) => JSON.stringify(value)).join(" | ");
  if (schema.type === "array") return `Array<${schemaType(schema.items)}>`;
  if (schema.type === "integer" || schema.type === "number") return "number";
  if (schema.type === "boolean") return "boolean";
  if (schema.type === "string") return "string";
  return "unknown";
}

function renderJsonSchemaTypes(definitions) {
  const lines = ["/* Generated from the independent JSON Schema authority. Do not edit. */", ""];
  for (const [name, schema] of Object.entries(definitions)) {
    if (schema.type !== "object") {
      lines.push(`export type ${name} = ${schemaType(schema)};`, "");
      continue;
    }
    const required = new Set(schema.required ?? []);
    lines.push(`export interface ${name} {`);
    for (const [property, propertySchema] of Object.entries(schema.properties ?? {})) {
      lines.push(`  ${property}${required.has(property) ? "" : "?"}: ${schemaType(propertySchema)};`);
    }
    lines.push("}", "");
  }
  lines.push("export type MorningBriefingDocument = MorningBriefing;", "");
  return lines.join("\n");
}

const jsonSchemaTypes = renderJsonSchemaTypes(jsonSchema.$defs);

const generated = [
  [resolve(typeSpecOutput, "types.ts"), astToString(typeSpecAst)],
  [resolve(typeSpecOutput, "validator.cjs"), createValidator(typeSpecRoot)],
  [resolve(jsonSchemaOutput, "types.d.ts"), jsonSchemaTypes],
  [resolve(jsonSchemaOutput, "validator.cjs"), createValidator(jsonSchema)],
];
await Promise.all(generated.map(async ([path, content]) => {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, content, "utf8");
}));

const sha256 = (content) => createHash("sha256").update(content).digest("hex");
await writeFile(
  resolve(root, "generated/provenance.json"),
  `${JSON.stringify({
    schema: "happy-wakey.contract-generation.v1",
    authorities: {
      jsonSchema: { path: "schemas/morning-briefing.schema.json", sha256: sha256(jsonSchemaSource) },
      typeSpec: { path: "typespec/main.tsp", sha256: sha256(typeSpecSource) },
    },
    intermediates: {
      typeSpecOpenApi: { path: "generated/typespec/openapi/openapi.yaml", sha256: sha256(openApiSource) },
    },
    outputs: [
      "generated/json-schema/types.d.ts",
      "generated/json-schema/validator.cjs",
      "generated/typespec/types.ts",
      "generated/typespec/validator.cjs"
    ],
  }, null, 2)}\n`,
  "utf8",
);
