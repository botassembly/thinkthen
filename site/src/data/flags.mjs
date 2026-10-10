// The command's flags, as each function's reference page lists them.
//
// Every flag is { spec, takes, default, what }. `spec` is the flag as a
// reader types it. `what` marks code with backticks. Each default and range
// comes from specification/settings.md through setting().
//
// GLOBAL_FLAGS holds exactly the flags every function's --help shows.
// SHARED_FLAGS holds the flags several functions take but not all. A
// function names the shared flags it takes in its `shared` list in
// catalog.mjs, and its own flags in `options`. scripts/check-flags.mjs
// compares all three with `thinkthen <fn> --help` and fails on any gap.

import { setting } from '../lib/settings-table.mjs';
import { backends } from '../lib/backends-table.mjs';

export const flag = (spec, takes, dflt, what, extra = {}) => ({ spec, takes, default: dflt, what, ...extra });

export const lower = (s) => s.charAt(0).toLowerCase() + s.slice(1);
// The default cut on one function, and the ends a cut may take.
export const cutOn = (fn) => setting('Threshold').defaultOn(fn);
export const CUT = setting('Threshold').bounds;
// A list typed beside @FILE replaces the question file's whole list.
export const LIST_RULE = 'A list typed beside `@FILE` replaces the question file\'s whole list. The two never merge.';
// A flag that a function refuses on one document says so only on a
// function that reads one document.
const ON_ONE_DOCUMENT = 'On one document it is a usage error.';
const dflt = (name) => lower(setting(name).default);
const THROTTLE = setting('Throttle');

export const GLOBAL_FLAGS = [
  flag('--context FILE', 'a readable UTF-8 file that is not blank', dflt('Context'), 'Shares the exact contents of FILE as evidence for every batch of records.'),
  flag('--max-request-bytes N', 'a whole number of 1 or more', dflt('Request size'), 'Closes or splits a request before it passes N bytes. A single record or question still goes alone. It beats `THINKTHEN_MAX_REQUEST_BYTES`.'),
  flag('--media text or image', 'text or image', 'text', 'Selects whole-file media. Image media requires --input and --unit file without record framing; only decide, choose and score admit it. Text files keep their existing framing.'),
  flag('--window N', lower(setting('Text-line window').allowed), dflt('Text-line window'), 'Joins N physical text lines per item within each file. A final short window remains one item. Blank lines stay inside each window; a window containing only white space is skipped. It takes no JSON or table framing, evidence pointer, or saved `on`.'),
  flag('--input FILE_OR_FOLDER', lower(setting('Input file').allowed), dflt('Input file'), 'Selects files or folders explicitly on all ten functions. Repeat it in argument order. Folder contents follow sorted relative paths. See Files and folders for located output.'),
  flag('--unit line or file', lower(setting('Reader unit').allowed), setting('Reader unit').default, 'Selects physical lines or complete original files. Requires --input and refuses JSON/table framing and pointers. Existing file calls without reader options keep their framing.'),
  flag('--lines', 'nothing', 'off', 'Takes each line as one text record. Input below gives the framing this function reads by default.'),
  flag('--jsonl', 'nothing', 'off', 'Takes each line as one JSON record.'),
  flag('--field POINTER', 'an RFC 6901 pointer, and it may repeat', dflt('Evidence pointer'), 'Sends only the part of each record the pointer names. Several pointers send one object of the parts. Input below gives every rule.'),
  flag('--plan', 'nothing', dflt('Plan preview'), 'Checks the whole input, prints the first request it would send and a summary line, and sends nothing. It needs no key, and it opens no connection.'),
  flag('--details', 'nothing', dflt('Details'), 'Prints the full result object in place of the bare value. In record mode the object also carries the whole record under `input`.'),
  flag('--facts', 'nothing', dflt('Run facts'), 'Prints one `thinkthen.run/1` line last on standard error. It holds the run\'s counts, and on a failure it names the cause.'),
  flag('--max-requests-total N', 'a whole number of 0 or more', dflt('Process request total'), 'Refuses a live attempt once this process has sent N attempts. An answer from the cache or a replay uses no attempt. 0 allows no live send.'),
  flag('--max-estimated-input-tokens-total N', 'a whole number of 0 or more', dflt('Estimated input admission total'), 'Refuses a live request once the estimated input tokens of this process would pass N. Each request body counts 0.908 tokens a byte. The estimate is not the backend\'s bill.'),
  flag('--url BASE', 'an `https` address, or `http` on `localhost`, `127.0.0.1` or `[::1]`', setting('Address').default, 'The server\'s base address. The tool posts to BASE/systemone. It outranks `THINKTHEN_BASE_URL` and the configuration file.'),
  flag('--backend NAME', `${backends().map((b) => `\`${b.name}\``).join(', ')}, or a name the configuration file adds`, 'none', 'Names a backend. A backend is a base address with its own key variables and model. It outranks `THINKTHEN_BACKEND`.'),
  flag('--profile FILE', 'a readable profile file', dflt('Backend profile'), 'Applies local byte and question limits before a request leaves, and names the calibration profile in use. It never picks an address, a model, a key or a cache.'),
  flag('--model NAME', 'a model name', setting('Model').default, 'The model the request carries. Name a version to pin a run.'),
  flag('--record DIR', 'a folder', dflt('Recording'), 'Calls the backend for every request and writes each exchange into DIR. The folder is created when absent.'),
  flag('--replay DIR', 'a folder', dflt('Recording'), 'Answers from DIR alone. No connection opens, and no key is needed. A missing answer exits 5.'),
  flag('--cache DIR', 'a folder', 'the default answer cache', 'Replays DIR and records into it. It is exactly `--record DIR --replay DIR`. Beside either of those it is a usage error.'),
  flag('--no-cache', 'nothing', 'off', 'Turns off the answer cache for one run, both lookup and writing. It never turns off usage counting.'),
  flag('--refresh-cache', 'nothing', 'off', 'Sends each request live and replaces its saved answer in the cache.'),
  flag('--timeout SECONDS', lower(setting('Timeout').allowed.split(';')[0].replace(/^command: /i, '')), setting('Timeout').number, 'Bounds one attempt from connect to last byte, and each wait before a retry. Another value exits 2.'),
  flag('--max-retries N', lower(setting('Retries').allowed), setting('Retries').default, 'How many times a retried status is sent again. A transport failure is never sent again.'),
];

export const SHARED_FLAGS = {
  '--image': flag('--image FILE', 'an original JPEG or PNG file, and it may repeat', 'off', 'Attaches ordered images to decide, choose or score on an admitted image backend. Other functions refuse images. Captions may use --input, record framing and pointers. Cannot accompany media, unit or window controls.'),
  '--csv': flag('--csv', 'nothing', 'off', 'Reads a comma-separated table with a required header row. Every cell is a string, and every result is JSON Lines.'),
  '--tsv': flag('--tsv', 'nothing', 'off', 'Reads a tab-separated table with a required header row. The CSV rules apply.'),
  '--image-media': flag('--image-media MIME', 'image/png or image/jpeg', 'infer from bytes', 'Declares the format of every repeated --image attachment. Requires --image; a mismatch refuses before sending.'),
  '--context-field': flag('--context-field POINTER', 'an RFC 6901 pointer', 'none', 'Selects separate context from each JSON record. Empty text suppresses shared context. It stays associated with its original record.'),
  '--batch': flag('--batch N or max', lower(setting('Batch').allowed), setting('Batch').default, '`max` fills each request to the backend\'s limits. `--batch 1` asks one record a request. Records that share one request can affect each other\'s answers. It beats `THINKTHEN_BATCH`, which beats a question file\'s `batch`.', { oneDocument: ON_ONE_DOCUMENT }),
  '--jobs': flag('--jobs N', `${THROTTLE.range.min} to ${THROTTLE.range.max}`, THROTTLE.default, 'The most requests in flight at once. A run opens up to one connection for each. It sets no limit on requests a minute.'),
};

// The flag names a spec holds: "--lines, --jsonl" holds two.
export const namesOf = (spec) => spec.match(/--[a-z][a-z0-9-]*/g) || [];

// One function's own and shared flags, in the order its page lists them.
export function flagsOf(fn) {
  const shared = (fn.shared || []).map((name) => {
    if (!SHARED_FLAGS[name]) throw new Error(`flags: ${fn.name} names no shared flag ${name}`);
    const f = SHARED_FLAGS[name];
    return f.oneDocument && !fn.streamOnly ? { ...f, what: `${f.what} ${f.oneDocument}` } : f;
  });
  return [...fn.options, ...shared];
}
