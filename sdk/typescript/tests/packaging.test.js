import test from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, cp, rm, mkdir, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

// Contract: the native family selects the public error class, even when its code differs.
test("native family determines the error class independently of the code prefix", async () => {
  const directory = await mkdtemp(join(tmpdir(), "thinkthen-family-"));
  const installed = dirname(resolve(process.env.THINKTHEN_NODE_PACKAGE));
  try {
    const packagePath = join(directory, "package");
    await cp(installed, packagePath, { recursive: true, filter: path => path !== join(installed, "thinkthen.node") });
    const native = join(packagePath, "thinkthen.node");
    await mkdir(native);
    await writeFile(join(native, "package.json"), '{"main":"index.cjs","type":"commonjs"}');
    await writeFile(join(native, "index.cjs"), `
const native = require(${JSON.stringify(join(installed, "thinkthen.node"))});
module.exports = {...native, Client: class extends native.Client {
  constructor(settings) {
    try { super(settings); }
    catch (error) {
      const payload = JSON.parse(error.message);
      if (payload.family !== "usage") throw error;
      throw new Error(JSON.stringify({...payload, family: "backend"}));
    }
  }
}};
`);
    const result = spawnSync(process.execPath, ["--input-type=module", "-e", `
import {ThinkThen, BackendError} from ${JSON.stringify(join(packagePath, "runtime.js"))};
import assert from "node:assert/strict";
assert.throws(() => new ThinkThen({backend:"unknown_backend"}), error => error instanceof BackendError && error.code === "usage.backend");
`], { encoding: "utf8" });
    assert.equal(result.status, 0, result.stderr);
  } finally { await rm(directory, {recursive: true}); }
});

// Contract: release assembly refuses any changed, missing or additional shared file.
test("platform tarball merge checks every shared file", () => {
  const result = spawnSync("python3", [fileURLToPath(new URL("./registry.py", import.meta.url)), dirname(resolve(process.env.THINKTHEN_NODE_PACKAGE))], {encoding:"utf8"});
  assert.equal(result.status, 0, result.stderr);
});
