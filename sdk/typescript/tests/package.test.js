import test from "node:test";
import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { pathToFileURL } from "node:url";
import { cp, mkdtemp, readFile, readdir, rm, stat } from "node:fs/promises";
import { tmpdir } from "node:os";
import { Worker } from "node:worker_threads";
import { once } from "node:events";
import { spawn } from "node:child_process";
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

// Feature: the release package installs all platform engines without a fetch.
test("one package carries exactly the release metadata's native platform engines", async () => {
  const source = await readFile(new URL("../../../scripts/release_registry.py", import.meta.url), "utf8");
  const table = source.match(/NODE_TARGETS = \{([\s\S]*?)\}/);
  assert(table, "release metadata must identify npm platform mappings");
  const targets = [...table[1].matchAll(/'[^']+':\s*'([^']+)'/g)].map(match => `${match[1]}.node`).sort();
  assert(targets.length > 0);
  if ((await stat(join(root, "thinkthen.node"))).isFile()) {
    assert(targets.includes(`${process.platform}-${process.arch}.node`));
    return;
  }
  const files = (await readdir(join(root, "thinkthen.node"))).filter(name => name.endsWith(".node")).sort();
  assert.deepEqual(files, targets);
  const manifest = JSON.parse(await readFile(join(root, "package.json"), "utf8"));
  assert.equal(Object.keys(manifest.optionalDependencies ?? {}).length, 0);
  for (const name of ["preinstall", "install", "postinstall"]) assert.equal(manifest.scripts?.[name], undefined);
});

// Feature: an addon can serve independent worker contexts through its public API.
test("worker_threads loads the installed package and returns a typed row", async () => {
  const entry = pathToFileURL(require.resolve("thinkthen")).href;
  const code = `import { parentPort } from 'node:worker_threads';
    import { ThinkThen } from ${JSON.stringify(entry)};
    const client = new ThinkThen({backend:'stub'});
    try { parentPort.postMessage(await client.decide('Ready?', 'evidence')); }
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
