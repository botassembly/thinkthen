import test from "node:test";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { pathToFileURL, fileURLToPath } from "node:url";
import { cp, mkdtemp, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { Worker } from "node:worker_threads";
import { once } from "node:events";
import { spawn, spawnSync } from "node:child_process";
import { osEnvironment } from "./support.js";

const root = dirname(process.env.THINKTHEN_NODE_PACKAGE);
const require = createRequire(join(root, "package.json"));

// Feature: one ESM implementation gives import and require identical classes.
test("installed exports share constructors across ESM and CommonJS", async () => {
  const required = require("thinkthen");
  const imported = await import(pathToFileURL(require.resolve("thinkthen")).href);
  assert.equal(required.ThinkThen, imported.ThinkThen);
  assert.equal(required.UsageError, imported.UsageError);
  assert.equal(required.CancelledError, imported.CancelledError);
  const error = new required.UsageError({ code: "usage.question", message: "Invalid" });
  assert(error instanceof imported.ThinkThenError);
  const manifest = JSON.parse(await readFile(join(root, "package.json"), "utf8"));
  assert.equal(manifest.engines.node, ">=22.12");
  assert.equal(manifest.sideEffects, false);
  assert.equal(typeof manifest.exports["."].types, "string");
  assert.equal(typeof manifest.exports["."].default, "string");
  assert(!Object.hasOwn(manifest.exports["."], "require"));
});

// Feature: importing types and functions never loads native code eagerly.
test("import works without an engine and first loads preserve the loader error", async () => {
  const temporary = await mkdtemp(join(tmpdir(), "thinkthen-import-"));
  const copy = join(temporary, "package");
  try {
    await cp(root, copy, { recursive: true });
    await rm(join(copy, "thinkthen.node"), { recursive: true, force: true });
    const sdk = await import(pathToFileURL(join(copy, "runtime.js")).href);
    assert.equal(typeof sdk.decide, "function");
    for (const load of [() => new sdk.ThinkThen({backend:"stub"}),
      () => sdk.parseToolbox("{}"), () => sdk.serializeToolbox({})]) {
      assert.throws(load, error => error instanceof sdk.ThinkThenError
        && error.code === "local.io" && /native engine/.test(error.message));
    }
  } finally { await rm(temporary, { recursive: true, force: true }); }
});

// Feature: an addon can serve independent worker contexts through its public API.
test("worker_threads loads the installed package and returns a typed row", async () => {
  const entry = pathToFileURL(require.resolve("thinkthen")).href;
  const code = `import { parentPort } from 'node:worker_threads';
    import { ThinkThen } from ${JSON.stringify(entry)};
    const client = new ThinkThen({backend:'stub'});
    try { parentPort.postMessage(await client.decide('Ready?', 'evidence').row()); }
    finally { await client.close(); }`;
  const worker = new Worker(new URL(`data:text/javascript,${encodeURIComponent(code)}`), { type: "module" });
  const exited = once(worker, "exit");
  try {
    const [row] = await once(worker, "message");
    assert.equal(row.function, "decide");
    assert.equal(row.position, 1);
    assert.equal(row.status, "answered");
    assert.equal(typeof row.answer, "boolean");
    const [code] = await exited;
    assert.equal(code, 0);
  } finally { await worker.terminate(); }
});

// Feature: completed and abandoned streams release the process event loop.
for (const state of ["completed", "after-row", "pending-input", "pending-after-row"]) {
  test(`a ${state} stream releases the process event loop`, {timeout:10000}, async t => {
    const entry = pathToFileURL(require.resolve("thinkthen")).href;
    const code = `import {ThinkThen} from ${JSON.stringify(entry)};
      const client = new ThinkThen({backend:"stub",batch:1});
      const state = ${JSON.stringify(state)};
      const records = state === "pending-input"
        ? {[Symbol.asyncIterator](){return this;},next(){return new Promise(()=>{});}}
        : state === "pending-after-row"
          ? {[Symbol.asyncIterator](){return this;},index:0,next(){return this.index++ === 0 ? Promise.resolve({done:false,value:"first"}) : new Promise(()=>{});}}
          : ["first","second"];
      const stream = client.decideStream("Ready?",records);
      const pending = stream.next();
      if (state === "completed") { await pending; await stream.toArray(); await stream.next(); await client.close(); }
      else if (state === "after-row") await pending;
      else if (state === "pending-after-row") { console.log((await pending).value.position); stream.next(); }
      globalThis.retained = {stream,client};`;
    const child = spawn(process.execPath,["--input-type=module","-e",code],{env:osEnvironment(),stdio:["ignore","pipe","pipe"]});
    const output = [], rows = [];
    child.stderr.on("data",data => output.push(data));
    child.stdout.on("data",data => rows.push(data));
    try {
      const [code,signal] = await once(child,"exit",{signal:t.signal});
      assert.equal(signal,null,Buffer.concat(output).toString());
      assert.equal(code,0,Buffer.concat(output).toString());
      if (state === "pending-after-row") assert.equal(Buffer.concat(rows).toString(),"1\n");
    } finally { child.kill(); }
  });
}

// Feature: pending caller input releases Node after native input was admitted.
for (const name of ["decide", "rank"]) {
  test(`${name} releases an idle input after a submitted record`, { timeout: 10000 }, async t => {
    const entry = pathToFileURL(require.resolve("thinkthen")).href;
    const program = `import {ThinkThen} from ${JSON.stringify(entry)};
      const client = new ThinkThen({backend:"stub",batch:1});
      let pulled = 0;
      const input = {[Symbol.asyncIterator](){return this;},next(){
        if (++pulled === 1) return Promise.resolve({done:false,value:"record"});
        console.log("pending input"); return new Promise(()=>{});
      }};
      const name = ${JSON.stringify(name)};
      const stream = name === "rank" ? client.rankStream("Priority?",input,{levels:["low","high"]})
        : client.decideStream("Ready?",input);
      stream.next().then(() => stream.next());
      globalThis.retained = {client,stream};`;
    const child = spawn(process.execPath, ["--input-type=module", "-e", program], { env: osEnvironment(), stdio: ["ignore", "pipe", "pipe"] });
    const output = [], errors = [];
    child.stdout.on("data", data => output.push(data));
    child.stderr.on("data", data => errors.push(data));
    try {
      const [code, signal] = await once(child, "exit", { signal: t.signal });
      assert.equal(signal, null, Buffer.concat(errors).toString());
      assert.equal(code, 0, Buffer.concat(errors).toString());
      assert.equal(Buffer.concat(output).toString(), "pending input\n");
    } finally { child.kill(); }
  });
}

// Feature: closing lazy rank input retains Node through all ranked output.
test("rank input completion delivers every row before idle exit", { timeout: 10000 }, async t => {
  const entry = pathToFileURL(require.resolve("thinkthen")).href;
  const program = `import {ThinkThen} from ${JSON.stringify(entry)};
    const client = new ThinkThen({backend:"stub",batch:1});
    async function* input(){yield "one";yield "two";yield "three";}
    const rows = await client.rankStream("Priority?",input(),{levels:["low","high"]}).toArray();
    console.log(JSON.stringify(rows.map(row=>({position:row.position,status:row.status,answer:row.answer}))));
    await client.close();`;
  const child = spawn(process.execPath, ["--input-type=module", "-e", program], { env: osEnvironment(), stdio: ["ignore", "pipe", "pipe"] });
  const output = [], errors = [];
  child.stdout.on("data", data => output.push(data));
  child.stderr.on("data", data => errors.push(data));
  try {
    const [code, signal] = await once(child, "exit", { signal: t.signal });
    assert.equal(signal, null, Buffer.concat(errors).toString());
    assert.equal(code, 0, Buffer.concat(errors).toString());
    const rows = JSON.parse(Buffer.concat(output).toString());
    assert.deepEqual(rows.map(row => row.position).sort(), [1, 2, 3]);
    assert.deepEqual(rows.map(row => row.answer), [1, 2, 3]);
    assert(rows.every(row => row.status === "answered"));
  } finally { child.kill(); }
});

// Feature: unsupported OS/CPU pairs fail during the package's installation hook.
test("installation refuses engines absent from the release package", async () => {
  const script = pathToFileURL(join(root, "platform.js")).href;
  for (const [platform, arch] of [["darwin", "x64"], ["win32", "arm64"]]) {
    const program = `Object.defineProperty(process, "platform", {value:${JSON.stringify(platform)}});
      Object.defineProperty(process, "arch", {value:${JSON.stringify(arch)}});
      await import(${JSON.stringify(script)});`;
    const result = spawnSync(process.execPath, ["--input-type=module", "-e", program], {encoding:"utf8"});
    assert.equal(result.status, 1);
    assert.equal(result.stderr, `thinkthen does not ship a native engine for ${platform}/${arch}\n`);
  }
});

const glibcReport = fileURLToPath(new URL("./glibc-report.cjs", import.meta.url));
/** Run node in the installed package with the runtime reporting this glibc. */
function withGlibc(version, ...command) {
  return spawnSync(process.execPath, ["--require", glibcReport, ...command],
    {cwd: root, encoding: "utf8", env: {...osEnvironment(), THINKTHEN_TEST_GLIBC: version}});
}
const firstLoad = `import { ThinkThen, ThinkThenError } from "./runtime.js";
  try { await new ThinkThen({backend: "stub"}).close(); console.log("loaded"); }
  catch (error) { console.log(JSON.stringify([error instanceof ThinkThenError, error.code, error.message])); }`;

// Feature: Linux below the glibc floor stops at install and at first load with one line of ours.
test("installation and first load refuse Linux below the glibc floor", {skip: process.platform !== "linux"}, () => {
  for (const [version, found] of [["", "none"], ["2.27", "glibc 2.27"], ["2.9", "glibc 2.9"]]) {
    const line = `thinkthen needs glibc 2.28 or later on Linux; this system has ${found}`;
    const install = withGlibc(version, "platform.js");
    assert.equal(install.status, 1, version);
    assert.equal(install.stderr, `${line}\n`);
    const load = withGlibc(version, "--input-type=module", "-e", firstLoad);
    assert.equal(load.status, 0, load.stderr);
    assert.deepEqual(JSON.parse(load.stdout), [true, "local.io", line]);
  }
});

// Feature: Linux at or above the glibc floor installs and loads, whatever the version's text order.
test("installation and first load accept Linux at the glibc floor and above", {skip: process.platform !== "linux"}, () => {
  for (const version of ["2.28", "2.100", "3.1"]) {
    const install = withGlibc(version, "platform.js");
    assert.equal(install.status, 0, install.stderr);
    assert.equal(install.stderr, "");
    const load = withGlibc(version, "--input-type=module", "-e", firstLoad);
    assert.equal(load.status, 0, load.stderr);
    assert.equal(load.stdout, "loaded\n");
  }
});

// Feature: top-N rank never retains all caller objects in the JavaScript adapter.
test("top one ranks a large lazy input within a bounded JavaScript heap", async () => {
  const script = fileURLToPath(new URL("./rank-memory.mjs", import.meta.url));
  const child = spawn(process.execPath, ["--expose-gc", "--max-old-space-size=64", script, process.env.THINKTHEN_NODE_PACKAGE],
    {env:osEnvironment(),stdio:["ignore","pipe","pipe"]});
  const output = [], errors = [];
  child.stdout.on("data", data => output.push(data));
  child.stderr.on("data", data => errors.push(data));
  try {
    const [code, signal] = await once(child, "exit");
    assert.equal(signal, null, Buffer.concat(errors).toString());
    assert.equal(code, 0, Buffer.concat(errors).toString());
    console.log(Buffer.concat(output).toString().trim());
  } finally {child.kill();}
});
