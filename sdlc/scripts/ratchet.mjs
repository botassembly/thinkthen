#!/usr/bin/env node
// Ratchet — the size ceiling, checked at lint time so you hear about it here
// rather than at the sealed verify gate. The number lives in sdlc/ratchet.json
// and nowhere else; this file is the same short reader in every repo that has
// adopted one, and it applies the same rule the sealed gate applies: the
// ceiling must EQUAL the measured total (ticket 0060 — the ceiling lands at
// actual). Slack cannot accumulate, so a raise is always a deliberate edit.
//
// Raising max requires two things in that commit's message, and the second is
// the one that matters:
//   1. the justification — what grew, and why it earns its lines;
//   2. the confirmation — you looked for duplication and bloat to remove
//      first, and name what you checked. "I searched X and Y for code to
//      slim and found none" is the sentence; without it a raise is refused
//      in review. Tickets do not grant raise allowances (ruling 2026-08-13,
//      landed as ticket 0085): an attempt raises the ceiling itself, in the
//      commit that needs it, and defends the number there.
// The ceiling exists because you maintain this codebase — every raise you
// take today is code future-you must carry. The lazy raise is the trap.
//
// A repo with no sdlc/ratchet.json has not adopted a ceiling; that is a
// choice, not a fault, and this exits quiet — same as the sealed gate.

import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

// The repo is this file's own home (sdlc/scripts/), never the working
// directory: `make -C bot ratchet` and a gate both have to reach the same
// number, and cwd resolution is what made that go wrong elsewhere.
const REPO = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const CONFIG = join(REPO, "sdlc", "ratchet.json");

if (!existsSync(CONFIG)) process.exit(0);
const { directory, extension, max } = JSON.parse(readFileSync(CONFIG, "utf8"));

// Non-blank lines only (botassembly ticket 0032): counting every line makes
// deleting blank lines currency for adding code.
function nonBlank(source) {
  return source.split("\n").filter((line) => line.trim().length > 0).length;
}

function loc(dir) {
  let n = 0;
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, entry.name);
    if (entry.isDirectory()) n += loc(p);
    else if (entry.name.endsWith(extension)) n += nonBlank(readFileSync(p, "utf8"));
  }
  return n;
}

const total = loc(join(REPO, directory));
if (total !== max) {
  const remedy = total < max ? `lower it to ${total}` : `raise it to ${total}`;
  console.error(`ratchet: ${directory} is ${total} non-blank lines, ceiling is ${max}. The ceiling must equal the total; ${remedy} in sdlc/ratchet.json, in a commit that says why.`);
  process.exit(1);
}
console.log(`ratchet: ${directory} ${total}/${max}`);
