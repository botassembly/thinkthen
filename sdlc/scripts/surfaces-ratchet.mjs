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

// A missing or unreadable ceiling file fails: deleting it must not delete
// the check (surfaces-review-7 verifier).
if (!existsSync(CONFIG)) {
  console.error("surfaces-ratchet: sdlc/surfaces-ratchet.json is missing. The ceiling file must exist; restore it from git.");
  process.exit(1);
}
let ceiling;
try {
  ceiling = JSON.parse(readFileSync(CONFIG, "utf8"));
} catch {
  console.error("surfaces-ratchet: sdlc/surfaces-ratchet.json is not valid JSON.");
  process.exit(1);
}
const { directories, extension, exclude, max } = ceiling;
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
// the wave-7 verifiers' attacks).
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
//      or ACCEPT WITH FOLLOW-UP. The SHA is the full 40 characters: a
//      short prefix can be ground onto a forged commit in seconds.
//      Prose that names the SHA does not count, and neither does a line
//      inside a fenced code block or an HTML comment.
//   2. sdlc/surfaces-ratchet-reviews.json grandfathers it by full SHA.
//      Only an ancestor of GRANDFATHER_UNTIL can be listed, so the list
//      cannot excuse a new raise.
// A commit after SCRIPT_FROM that changes this script or its self-test
// needs a structured verdict too, the same as a raise: whoever can edit
// the rule can switch it off. An uncommitted raise in the working tree
// fails, since it has no SHA a review could name.
// BASE, GRANDFATHER_UNTIL, and SCRIPT_FROM live here, in code, not in the
// JSON file, so an edit to the JSON cannot move them.
// In a tree without .git (a frozen export) the check says so and skips
// only itself; the count check above never skips. A shallow clone fails
// by name: the walk needs the whole history back to BASE.
const BASE = "64133279d3e17fd8a7054e3a5f870a2229cc8bc7";
const GRANDFATHER_UNTIL = "db270d2113e6d3e51d0db35ead6d6a781a97dfc8";
const SCRIPT_FROM = "f6a7faea8384e715b4059fa224d06122773cc600";
const FILE = "sdlc/surfaces-ratchet.json";
const REVIEWS = "sdlc/surfaces-ratchet-reviews.json";
const GUARDED = ["sdlc/scripts/surfaces-ratchet.mjs", "sdlc/scripts/surfaces-ratchet-self-test"];
const FULL = /^[0-9a-f]{40}$/;

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

// The lines a reader sees: HTML comments and fenced code blocks removed.
// An unclosed comment or fence hides the rest of the file.
function readable(text) {
  const lines = [];
  let fence = null;
  for (const line of text.replace(/<!--[\s\S]*?(?:-->|$)/g, "").split("\n")) {
    const mark = line.match(/^ {0,3}(`{3,}|~{3,})/);
    if (fence) {
      if (mark && mark[1][0] === fence[0] && mark[1].length >= fence.length) fence = null;
    } else if (mark) fence = mark[1];
    else lines.push(line.trim());
  }
  return lines;
}

// The full SHAs every committed review record accepts.
function acceptedShas() {
  const paths = git(["ls-tree", "-r", "--name-only", "HEAD", "--", "sdlc/records"])
    .split("\n")
    .filter((path) => /(^|\/)REVIEW-[^/]*\.md$/.test(path));
  const accepted = new Set();
  const verdict = /^Verdict: ACCEPT(?: WITH FOLLOW-UP)? ([0-9a-f]{40})$/;
  const row = /^\| `([0-9a-f]{40})` \|.*\| (?:ACCEPT|ACCEPT WITH FOLLOW-UP) \|$/;
  for (const path of paths) {
    for (const line of readable(git(["show", `HEAD:${path}`]))) {
      const hit = line.match(verdict) ?? line.match(row);
      if (hit) accepted.add(hit[1]);
    }
  }
  return accepted;
}

// Each commit in range with its parents, and the blob of path in each.
function walk(range, path) {
  const commits = git(["rev-list", "--parents", range]).split("\n").filter(Boolean)
    .map((line) => line.split(" "));
  const wanted = [...new Set(commits.flat())];
  const blobs = new Map();
  if (wanted.length > 0) {
    git(["cat-file", "--batch-check=%(objectname) %(objecttype) %(rest)"],
      wanted.map((sha) => `${sha}:${path} ${sha}`).join("\n") + "\n")
      .split("\n")
      .forEach((line, i) => blobs.set(wanted[i], line.includes(" blob ") ? line.split(" ")[0] : ""));
  }
  // A mover differs from every parent; a root commit that holds the file moves it.
  const movers = commits.filter(([sha, ...parents]) =>
    parents.length > 0 ? parents.every((p) => blobs.get(p) !== blobs.get(sha)) : blobs.get(sha) !== "");
  return { movers, blobs };
}

const shape = (c) => JSON.stringify([c.directories, c.extension, c.exclude]);

if (!gitOk(["rev-parse", "--git-dir"])) {
  console.error("surfaces-ratchet: no git history here; the raise-body check cannot run");
} else {
  if (git(["rev-parse", "--is-shallow-repository"]) === "true") {
    fail(
      "the history is too shallow to find the commit that moved sdlc/surfaces-ratchet.json; fetch the full history (git fetch --unshallow, or fetch-depth: 0 in the checkout step)",
    );
  }
  for (const anchor of [BASE, GRANDFATHER_UNTIL, SCRIPT_FROM]) {
    if (!gitOk(["cat-file", "-e", `${anchor}^{commit}`]) || !gitOk(["merge-base", "--is-ancestor", anchor, "HEAD"])) {
      fail(`${anchor} is not in this history; the review walk starts there`);
    }
  }
  let grandfathered;
  try {
    ({ grandfathered } = JSON.parse(git(["show", `HEAD:${REVIEWS}`])));
  } catch {
    fail(`${REVIEWS} is missing at HEAD or is not valid JSON`);
  }
  if (typeof grandfathered !== "object" || grandfathered === null || Array.isArray(grandfathered)) {
    fail(`${REVIEWS} has no "grandfathered" object mapping full SHAs to reasons`);
  }
  for (const sha of Object.keys(grandfathered)) {
    if (!FULL.test(sha)) fail(`${REVIEWS} grandfathers ${sha}, which is not a full 40-character SHA`);
    if (!gitOk(["merge-base", "--is-ancestor", sha, GRANDFATHER_UNTIL])) {
      fail(`${REVIEWS} grandfathers ${sha}, which is not in the history up to ${GRANDFATHER_UNTIL}`);
    }
  }
  const accepted = acceptedShas();

  const { movers, blobs } = walk(`${BASE}..HEAD`, FILE);
  const config = (sha) => (blobs.get(sha) ? JSON.parse(git(["cat-file", "-p", blobs.get(sha)])) : undefined);
  const unreviewed = [];
  for (const [sha, ...parents] of movers) {
    if (!blobs.get(sha)) continue;
    const now = config(sha);
    const before = parents.map(config);
    const lower = before.length > 0 && before.every((c) => c !== undefined && now.max < c.max && shape(c) === shape(now));
    if (lower) {
      if (git(["log", "-1", "--format=%b", sha]) === "") unreviewed.push(sha);
      continue;
    }
    if (Object.hasOwn(grandfathered, sha) || accepted.has(sha)) continue;
    unreviewed.push(sha);
  }
  if (unreviewed.length > 0) {
    fail(
      `these commits move sdlc/surfaces-ratchet.json without a second-agent review: ${unreviewed.join(", ")}. A raise needs a REVIEW-*.md record under sdlc/records, committed, with "Verdict: ACCEPT <full 40-character sha>" (or ACCEPT WITH FOLLOW-UP), and a lower needs a body (CLAUDE.md, surfaces-review-7).`,
    );
  }

  const unguarded = new Set();
  for (const path of GUARDED) {
    for (const [sha] of walk(`${SCRIPT_FROM}..HEAD`, path).movers) if (!accepted.has(sha)) unguarded.add(sha);
  }
  if (unguarded.size > 0) {
    fail(
      `these commits change the ratchet script or its self-test without a second-agent review: ${[...unguarded].join(", ")}. Each needs a REVIEW-*.md record under sdlc/records, committed, with "Verdict: ACCEPT <full 40-character sha>" (or ACCEPT WITH FOLLOW-UP).`,
    );
  }

  let committed;
  try {
    committed = JSON.parse(git(["show", `HEAD:${FILE}`]));
  } catch {
    committed = undefined;
  }
  if (committed === undefined || shape(committed) !== shape(ceiling) || max > committed.max) {
    fail(
      `sdlc/surfaces-ratchet.json in the working tree raises the committed ceiling. Commit the raise with a body and add a review record; an uncommitted raise has no SHA a review can name.`,
    );
  }
}
console.log(`surfaces-ratchet: ${directories.join(", ")} ${total}/${max}`);
