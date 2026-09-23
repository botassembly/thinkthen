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
// (surfaces-review-4 item 16, surfaces-review-7 rows R5-28 and R7-4, and
// the wave-7 verifier's attacks).
//
// Every commit after BASE whose ceiling file differs from each of its
// parents is a mover. The walk reads every commit, side branches too, so
// history simplification cannot hide a raise behind an identical one.
// A mover that lowers max below every parent's and keeps directories,
// extension, and exclude unchanged needs only a body: the count must
// equal the ceiling, so such a lower cannot hide growth. Every other
// mover is a raise. A raise passes in one of two ways:
//   1. A review record committed at HEAD (a REVIEW-*.md file under
//      sdlc/records, read from git, never from the working tree) accepts
//      it in a structured line: either "Verdict: ACCEPT <sha>" or
//      "Verdict: ACCEPT WITH FOLLOW-UP <sha>", or a table row whose first
//      cell is the backticked SHA and whose last cell is exactly ACCEPT
//      or ACCEPT WITH FOLLOW-UP. Prose that names the SHA does not count.
//   2. sdlc/surfaces-ratchet-reviews.json grandfathers it by SHA. Only an
//      ancestor of GRANDFATHER_UNTIL can be listed, so the list cannot
//      excuse a new raise.
// BASE and GRANDFATHER_UNTIL live here, in code, not in the JSON file, so
// an edit to the JSON cannot move them.
// In a tree without .git (a frozen export) the check says so and skips
// only itself; the count check above never skips. A shallow clone fails
// by name: the walk needs the whole history back to BASE.
const BASE = "64133279d3e17fd8a7054e3a5f870a2229cc8bc7";
const GRANDFATHER_UNTIL = "db270d2113e6d3e51d0db35ead6d6a781a97dfc8";
const FILE = "sdlc/surfaces-ratchet.json";
const REVIEWS = "sdlc/surfaces-ratchet-reviews.json";

const git = (args, input) =>
  execFileSync("git", args, {
    cwd: REPO,
    encoding: "utf8",
    input,
    maxBuffer: 1 << 28,
    stdio: ["pipe", "pipe", "ignore"],
  }).trim();
const gitOk = (args) => {
  try {
    git(args);
    return true;
  } catch {
    return false;
  }
};
const fail = (message) => {
  console.error(`surfaces-ratchet: ${message}`);
  process.exit(1);
};

// The accept verdicts every committed review record gives, as SHA prefixes.
function acceptedPrefixes() {
  const paths = git(["ls-tree", "-r", "--name-only", "HEAD", "--", "sdlc/records"])
    .split("\n")
    .filter((path) => /(^|\/)REVIEW-[^/]*\.md$/.test(path));
  const accepted = [];
  const verdict = /^Verdict: ACCEPT(?: WITH FOLLOW-UP)? ([0-9a-f]{7,40})$/;
  const row = /^\| `([0-9a-f]{7,40})` \|.*\| (?:ACCEPT|ACCEPT WITH FOLLOW-UP) \|$/;
  for (const path of paths) {
    for (const line of git(["show", `HEAD:${path}`]).split("\n")) {
      const hit = line.trim().match(verdict) ?? line.trim().match(row);
      if (hit) accepted.push(hit[1]);
    }
  }
  return accepted;
}

if (!gitOk(["rev-parse", "--git-dir"])) {
  console.error("surfaces-ratchet: no git history here; the raise-body check cannot run");
} else {
  if (git(["rev-parse", "--is-shallow-repository"]) === "true") {
    fail(
      "the history is too shallow to find the commit that moved sdlc/surfaces-ratchet.json; fetch the full history (git fetch --unshallow, or fetch-depth: 0 in the checkout step)",
    );
  }
  for (const anchor of [BASE, GRANDFATHER_UNTIL]) {
    if (!gitOk(["cat-file", "-e", `${anchor}^{commit}`]) || !gitOk(["merge-base", "--is-ancestor", anchor, "HEAD"])) {
      fail(`${anchor} is not in this history; the review walk starts there`);
    }
  }
  const { grandfathered } = JSON.parse(git(["show", `HEAD:${REVIEWS}`]));
  for (const sha of Object.keys(grandfathered)) {
    if (!gitOk(["merge-base", "--is-ancestor", sha, GRANDFATHER_UNTIL])) {
      fail(`${REVIEWS} grandfathers ${sha}, which is not in the history up to ${GRANDFATHER_UNTIL}`);
    }
  }
  // Every commit after BASE with its parents, and the ceiling blob of each.
  const commits = git(["rev-list", "--parents", `${BASE}..HEAD`]).split("\n").filter(Boolean)
    .map((line) => line.split(" "));
  const wanted = [...new Set(commits.flat())];
  const blobs = new Map();
  git(["cat-file", "--batch-check=%(objectname) %(objecttype) %(rest)"],
    wanted.map((sha) => `${sha}:${FILE} ${sha}`).join("\n") + "\n")
    .split("\n")
    .forEach((line, i) => blobs.set(wanted[i], line.includes(" blob ") ? line.split(" ")[0] : ""));
  const config = (sha) => (blobs.get(sha) ? JSON.parse(git(["cat-file", "-p", blobs.get(sha)])) : undefined);
  const shape = (c) => JSON.stringify([c.directories, c.extension, c.exclude]);
  const accepted = acceptedPrefixes();
  const unreviewed = [];
  for (const [sha, ...parents] of commits) {
    const mine = blobs.get(sha);
    if (parents.length > 0 && parents.every((p) => blobs.get(p) === mine)) continue;
    if (parents.some((p) => blobs.get(p) === mine)) continue;
    if (!mine) continue;
    const now = config(sha);
    const before = parents.map(config);
    const lower = before.length > 0 && before.every((c) => c !== undefined && now.max < c.max && shape(c) === shape(now));
    if (lower) {
      if (git(["log", "-1", "--format=%b", sha]) === "") unreviewed.push(sha);
      continue;
    }
    if (Object.keys(grandfathered).some((g) => sha.startsWith(g))) continue;
    if (accepted.some((prefix) => sha.startsWith(prefix))) continue;
    unreviewed.push(sha);
  }
  if (unreviewed.length > 0) {
    fail(
      `these commits move sdlc/surfaces-ratchet.json without a second-agent review: ${unreviewed.map((s) => s.slice(0, 7)).join(", ")}. A raise needs a REVIEW-*.md record under sdlc/records, committed, with "Verdict: ACCEPT <sha>" (or ACCEPT WITH FOLLOW-UP), and a lower needs a body (CLAUDE.md, surfaces-review-7).`,
    );
  }
}
console.log(`surfaces-ratchet: ${directories.join(", ")} ${total}/${max}`);
