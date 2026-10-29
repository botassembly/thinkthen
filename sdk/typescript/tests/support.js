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

async function deadline(pending, label) {
  let timer;
  try {
    return await Promise.race([pending, new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error(`fixture ${label} deadline expired`)), 30000);
    })]);
  } finally { clearTimeout(timer); }
}

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
  let port;
  try { port = (await deadline(lines.next(), "startup")).value; }
  catch (error) {
    child.kill();
    await deadline(exited, "failed startup cleanup");
    if (directory) await rm(directory, { recursive: true });
    throw error;
  }
  return {
    url: `http://127.0.0.1:${port}/v1`,
    async event(name) { child.stdin.write(`${name}\n`); return (await deadline(lines.next(), name)).value; },
    async count() { child.stdin.write("count\n"); return Number((await deadline(lines.next(), "count")).value); },
    async close() {
      child.stdin.end("shutdown\n");
      try {
        const [code] = await deadline(exited, "shutdown");
        if (code !== 0) throw new Error(`fixture exited with ${code}`);
        await deadline(lines.return(), "pipe close");
      } finally { child.kill(); }
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
