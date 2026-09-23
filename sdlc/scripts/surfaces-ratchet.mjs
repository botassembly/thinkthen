#!/usr/bin/env node
// The surfaces ratchet — the same rule as sdlc/scripts/ratchet.mjs, for the
// trees that reader cannot see. The contract, the stand-in, and every
// surface build through their own cargo workspaces, so the crates/ ceiling
// leaves their growth invisible; this reader counts non-blank lines of one
// extension across the listed directories and applies the same rule the
// sealed gate applies: the ceiling must EQUAL the measured total (ticket
// 0060 — the ceiling lands at actual). Slack cannot accumulate, so a raise
// is always a deliberate edit.
//
// Raising max requires the same two things in that commit's message as the
// crates ceiling: the justification (what grew, and why it earns its
// lines) and the confirmation that duplication and bloat were searched for
// first, naming what was checked. Tickets do not grant raise allowances
// (ruling 2026-08-13, landed as ticket 0085).
//
// Excluded from the count: build and vendor directories (target, build,
// dist, .cargo, .runtimes, node_modules, .venv). The number lives in
// sdlc/surfaces-ratchet.json and nowhere else.

import { existsSync, readdirSync, readFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const REPO = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const CONFIG = join(REPO, "sdlc", "surfaces-ratchet.json");

if (!existsSync(CONFIG)) process.exit(0);
const { directories, extension, exclude, max } = JSON.parse(readFileSync(CONFIG, "utf8"));
const skip = new Set(exclude);

// Non-blank lines only (botassembly ticket 0032): counting every line makes
// deleting blank lines currency for adding code.
function nonBlank(source) {
  return source.split("\n").filter((line) => line.trim().length > 0).length;
}

function loc(dir) {
  let n = 0;
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, entry.name);
    if (entry.isDirectory()) {
      if (!skip.has(entry.name)) n += loc(p);
    } else if (entry.name.endsWith(extension)) n += nonBlank(readFileSync(p, "utf8"));
  }
  return n;
}

let total = 0;
for (const directory of directories) {
  const path = join(REPO, directory);
  if (existsSync(path)) total += loc(path);
}
if (total !== max) {
  const remedy = total < max ? `lower it to ${total}` : `raise it to ${total}`;
  console.error(`surfaces-ratchet: ${directories.join(", ")} is ${total} non-blank ${extension} lines, ceiling is ${max}. The ceiling must equal the total; ${remedy} in sdlc/surfaces-ratchet.json, in a commit that says why.`);
  process.exit(1);
}

// The raise discipline, checked at the rung instead of remembered
// (surfaces-review-4, item 16): every commit that moves this ceiling
// carries a body naming what grew and why it earns its lines, plus the
// second-agent review line CLAUDE.md demands. The two empty-body raises
// the fourth review found (e11bb57, 4238ea5) fail this check until a
// compliant commit moves the ceiling again. In a tree without .git (a
// frozen export) the check says so and skips only itself; the count
// check above never skips.
// In a shallow clone (actions/checkout fetches depth 1 by default) the
// grafted root looks like the last mover of every file, and its message
// is whatever commit was checked out (surfaces-review-5). The check then
// fails for the wrong reason, so it names the real one.
const git = (args) =>
  execFileSync("git", args, { cwd: REPO, encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }).trim();
try {
  const mover = git(["log", "-1", "--format=%H", "--", "sdlc/surfaces-ratchet.json"]);
  const grafted = mover !== "" && git(["rev-parse", "--is-shallow-repository"]) === "true" &&
    git(["log", "-1", "--format=%P", mover]) === "";
  if (grafted) {
    console.error(
      "surfaces-ratchet: the history is too shallow to find the commit that moved sdlc/surfaces-ratchet.json; fetch the full history (git fetch --unshallow, or fetch-depth: 0 in the checkout step)",
    );
    process.exit(1);
  }
  const commit = mover === "" ? "" : git(["log", "-1", "--format=%B", mover]);
  if (commit !== "") {
    const body = commit.split("\n\n").slice(1).join("\n\n").trim();
    if (body === "" || !/second agent/i.test(body)) {
      console.error(
        "surfaces-ratchet: the last commit moving sdlc/surfaces-ratchet.json carries no body naming the growth and the second-agent review (CLAUDE.md). Raise the ceiling in a commit that does.",
      );
      process.exit(1);
    }
  }
} catch {
  console.error("surfaces-ratchet: no git history here; the raise-body check cannot run");
}
console.log(`surfaces-ratchet: ${directories.join(", ")} ${total}/${max}`);
