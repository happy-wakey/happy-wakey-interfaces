import { createRequire } from "node:module";
import { readFile } from "node:fs/promises";
import { resolve } from "node:path";

const require = createRequire(import.meta.url);
const root = resolve(import.meta.dirname, "..");
const fixture = JSON.parse(await readFile(resolve(root, "examples/morning-briefing.json"), "utf8"));
const jsonSchemaValidator = require(resolve(root, "generated/json-schema/validator.cjs"));
const typeSpecValidator = require(resolve(root, "generated/typespec/validator.cjs"));

for (const [authority, validate] of [
  ["JSON Schema", jsonSchemaValidator],
  ["TypeSpec", typeSpecValidator],
]) {
  if (!validate(fixture)) {
    throw new Error(`${authority} generated validator rejected the canonical fixture: ${JSON.stringify(validate.errors)}`);
  }
}

const unsafe = structuredClone(fixture);
unsafe.cards[1].deepLink.feedFallbackAllowed = true;
if (jsonSchemaValidator(unsafe)) {
  throw new Error("JSON Schema validator allowed a social-feed fallback");
}

const missingTenant = structuredClone(fixture);
delete missingTenant.account.tenantId;
if (jsonSchemaValidator(missingTenant) || typeSpecValidator(missingTenant)) {
  throw new Error("generated validators accepted a briefing without tenant isolation");
}

console.log("verified independent TypeSpec and JSON Schema generated validators");
