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
// (surfaces-review-4 item 16, surfaces-review-7 rows R5-28 and R7-4).
// Every commit after the recorded base that touches the ceiling file
// must show a real second-agent review. The last mover is not enough,
// because a later compliant commit would hide an earlier unreviewed one.
// A commit passes in one of three ways:
//   1. Its body has a paragraph that starts "Second-agent review:" and
//      cites an existing sdlc/ record or a commit SHA, with no word that
//      says the review is pending or missing.
//   2. A review record (a REVIEW-*.md file under sdlc/records) names its
//      SHA. A reviewer can cover several raises in one record.
//   3. sdlc/surfaces-ratchet-reviews.json grandfathers it by SHA. Only a
//      commit reachable from that file's grandfather_until can be listed,
//      so a new raise cannot be waved through the list.
// In a tree without .git (a frozen export) the check says so and skips
// only itself; the count check above never skips.
// In a shallow clone (actions/checkout fetches depth 1 by default) the
// grafted root looks like the last mover of every file (surfaces-review-5).
// The check then fails for the wrong reason, so it names the real one.
const git = (args) =>
  execFileSync("git", args, { cwd: REPO, encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }).trim();
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
const FILE = "sdlc/surfaces-ratchet.json";
const REVIEWS = join(REPO, "sdlc", "surfaces-ratchet-reviews.json");
const MISSING = /\b(pending|not yet|not done|unreviewed|not reviewed|nobody|no one|no second|awaiting|waiting|to come|tbd|todo)\b/i;

function reviewLine(body) {
  const at = body.search(/^second[- ]agent review:/im);
  if (at < 0) return "";
  return body.slice(at).split(/\n\s*\n/)[0].replace(/^second[- ]agent review:/i, "").trim();
}

function citesRecord(text) {
  for (const path of text.match(/\bsdlc\/[\w./-]+\.md\b/g) ?? []) {
    if (existsSync(join(REPO, path))) return true;
  }
  for (const sha of text.match(/\b[0-9a-f]{7,40}\b/g) ?? []) {
    if (gitOk(["cat-file", "-e", `${sha}^{commit}`])) return true;
  }
  return false;
}

function reviewRecords(dir = join(REPO, "sdlc", "records")) {
  if (!existsSync(dir)) return [];
  const found = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, entry.name);
    if (entry.isDirectory()) found.push(...reviewRecords(p));
    else if (/^REVIEW-.*\.md$/.test(entry.name)) found.push(readFileSync(p, "utf8"));
  }
  return found;
}

if (!gitOk(["rev-parse", "--git-dir"])) {
  console.error("surfaces-ratchet: no git history here; the raise-body check cannot run");
} else {
  const mover = git(["log", "-1", "--format=%H", "--", FILE]);
  const grafted = mover !== "" && git(["rev-parse", "--is-shallow-repository"]) === "true" &&
    git(["log", "-1", "--format=%P", mover]) === "";
  if (grafted) {
    fail(
      "the history is too shallow to find the commit that moved sdlc/surfaces-ratchet.json; fetch the full history (git fetch --unshallow, or fetch-depth: 0 in the checkout step)",
    );
  }
  const { base, grandfather_until: until, grandfathered } = JSON.parse(readFileSync(REVIEWS, "utf8"));
  for (const sha of Object.keys(grandfathered)) {
    if (!gitOk(["merge-base", "--is-ancestor", sha, until])) {
      fail(`${REVIEWS.slice(REPO.length + 1)} grandfathers ${sha}, which is not in the history up to ${until}`);
    }
  }
  const range = base === "" ? ["HEAD"] : [`${base}..HEAD`];
  const movers = git(["log", "--format=%H", ...range, "--", FILE]).split("\n").filter(Boolean);
  const records = reviewRecords();
  const unreviewed = movers.filter((sha) => {
    if (Object.keys(grandfathered).some((g) => sha.startsWith(g))) return false;
    if (records.some((text) => new RegExp(`\\b${sha.slice(0, 7)}[0-9a-f]*\\b`).test(text))) return false;
    const body = git(["log", "-1", "--format=%B", sha]).split("\n\n").slice(1).join("\n\n");
    const line = reviewLine(body);
    return line === "" || MISSING.test(line) || !citesRecord(line);
  });
  if (unreviewed.length > 0) {
    fail(
      `these commits move sdlc/surfaces-ratchet.json without a second-agent review: ${unreviewed.map((s) => s.slice(0, 7)).join(", ")}. Each needs a "Second-agent review:" paragraph that cites the review record or commit, or a REVIEW-*.md record under sdlc/records that names its SHA (CLAUDE.md, surfaces-review-7).`,
    );
  }
}
console.log(`surfaces-ratchet: ${directories.join(", ")} ${total}/${max}`);
