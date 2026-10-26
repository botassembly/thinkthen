import { spawn } from "node:child_process";
import { createInterface } from "node:readline";
import { once } from "node:events";
import { mkdtemp, writeFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

// A test never consults the caller's backend settings or credentials.
for (const name of ["OPENAI_API_KEY", "THINKTHEN_BASE_URL", "THINKTHEN_BATCH", "HTTPS_PROXY", "HTTP_PROXY", "ALL_PROXY"]) delete process.env[name];
process.env.THINKTHEN_API_KEY = "sk-test-0123456789";
export const sdk = await import(pathToFileURL(process.env.THINKTHEN_NODE_PACKAGE).href);

export async function backend(source) {
  let directory;
  let path = source;
  if (typeof source !== "string" && !Array.isArray(source)) {
    directory = await mkdtemp(join(tmpdir(), "thinkthen-node-"));
    path = join(directory, "case.json");
    await writeFile(path, JSON.stringify(source));
  }
  const env = osEnvironment();
  const child = spawn(process.env.THINKTHEN_LOOPBACK, Array.isArray(source) ? source : [path], {
    env, stdio: ["pipe", "pipe", "inherit"],
  });
  const exited = once(child, "exit");
  const lines = createInterface({ input: child.stdout })[Symbol.asyncIterator]();
  const port = (await lines.next()).value;
  return {
    url: `http://127.0.0.1:${port}/v1`,
    async event(name) { child.stdin.write(`${name}\n`); return (await lines.next()).value; },
    async count() { child.stdin.write("count\n"); return Number((await lines.next()).value); },
    async close() {
      child.stdin.end("shutdown\n");
      await exited;
      await lines.return();
      if (directory) await rm(directory, { recursive: true });
    },
  };
}

export function caseFor(question, records, replies) {
  return { function: "decide", question, records, replies, settings: {}, expected: [], requests: 0 };
}

export function osEnvironment() {
  const names = ["PATH", "SystemRoot", "WINDIR", "TEMP", "TMP"];
  return Object.fromEntries(names.filter(name => process.env[name] !== undefined).map(name => [name, process.env[name]]));
}
