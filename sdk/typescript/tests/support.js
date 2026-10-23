import { spawn } from "node:child_process";
import { createInterface } from "node:readline";
import { once } from "node:events";
import { mkdtemp, writeFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

// A test never consults the caller's backend settings or credentials.
for (const name of ["OPENAI_API_KEY", "THINKTHEN_BASE_URL", "THINKTHEN_BATCH"]) delete process.env[name];
process.env.THINKTHEN_API_KEY = "sk-test-0123456789";
export const sdk = await import(pathToFileURL(process.env.THINKTHEN_NODE_PACKAGE).href);

export async function backend(source) {
  let directory;
  let path = source;
  if (typeof source !== "string") {
    directory = await mkdtemp(join(tmpdir(), "thinkthen-node-"));
    path = join(directory, "case.json");
    await writeFile(path, JSON.stringify(source));
  }
  const child = spawn(process.env.THINKTHEN_LOOPBACK, [path], {
    env: { PATH: process.env.PATH }, stdio: ["pipe", "pipe", "inherit"],
  });
  const exited = once(child, "exit");
  const lines = createInterface({ input: child.stdout })[Symbol.asyncIterator]();
  const port = (await lines.next()).value;
  return {
    url: `http://127.0.0.1:${port}/v1`,
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
