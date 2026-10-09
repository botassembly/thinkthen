#!/usr/bin/env node
// Check the nonblank source total against sdlc/ratchet.json.
// The ceiling must equal the measured total; spare allowance cannot accumulate.
// A fresh reviewer accepts proposed growth before it lands. Explain the useful
// behavior and check for duplication first. Lower the ceiling when deleting code.
// AGENTS.md defines the current workflow; this reader adds no review gates.
// A repository without a ceiling exits quietly.

import { spawnSync } from "node:child_process";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

// The repo is this file's own home (sdlc/scripts/), never the working
// directory: `make -C bot ratchet` and a gate both have to reach the same
// number, and cwd resolution is what made that go wrong elsewhere.
const REPO = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
// One optional argument names another config, such as a binding's own
// (thinkthen ADR 0047). Its `directory` resolves from the config's own folder,
// and a named config that is missing fails.
const NAMED = process.argv[2];
const CONFIG = NAMED ? resolve(NAMED) : join(REPO, "sdlc", "ratchet.json");
const BASE = NAMED ? dirname(CONFIG) : REPO;

if (!existsSync(CONFIG)) {
  if (NAMED) console.error(`ratchet: ${NAMED} does not exist.`);
  process.exit(NAMED ? 1 : 0);
}
const { directory, extension, max } = JSON.parse(readFileSync(CONFIG, "utf8"));

// Non-blank lines only (botassembly ticket 0032): counting every line makes
// deleting blank lines currency for adding code.
function nonBlank(source) {
  return source.split("\n").filter((line) => line.trim().length > 0).length;
}

function files(dir) {
  const found = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, entry.name);
    // Build output under `target` is never source (ticket 0093).
    if (entry.isDirectory()) { if (entry.name !== "target") found.push(...files(p)); }
    else if (entry.name.endsWith(extension)) found.push(p);
  }
  return found;
}

// A file Git ignores is a build copy, such as the C header a binding check
// copies in, and never source. Outside a Git checkout every file counts.
function unignored(paths) {
  const ignored = spawnSync("git", ["-C", REPO, "check-ignore", "--stdin", "-z"], { input: paths.join("\0"), encoding: "utf8" });
  if (ignored.status !== 0) return paths;
  const skip = new Set(ignored.stdout.split("\0"));
  return paths.filter((p) => !skip.has(p));
}

// `directory` names one folder or a list of them (thinkthen ticket 0092 adds
// the conformance backend beside the crate).
const folders = [directory].flat();
const counted = unignored(folders.flatMap((folder) => files(resolve(BASE, folder))));
const total = counted.reduce((sum, p) => sum + nonBlank(readFileSync(p, "utf8")), 0);
const named = folders.join(" + ");
if (total !== max) {
  const remedy = total < max ? `lower it to ${total}` : `raise it to ${total}`;
  console.error(`ratchet: ${named} is ${total} non-blank lines, ceiling is ${max}. The ceiling must equal the total; ${remedy} in ${NAMED ?? "sdlc/ratchet.json"}, in a commit that says why.`);
  process.exit(1);
}
console.log(`ratchet: ${named} ${total}/${max}`);
