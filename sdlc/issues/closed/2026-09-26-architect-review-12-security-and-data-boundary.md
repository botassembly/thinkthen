Status: closed 2026-09-30. Item 1: ticket 0163 added the README disclosures, and ticket 0318 keeps SQL hosts off the platform cache. Item 2: ticket 0303's folder warning and `SECURITY.md` fix it for the command, and ticket 0318 refuses open SQL answer folders.

# Architect review 12: security and the data boundary

A fresh reviewer tested where the key and the judged text go, what lands on disk, and how planted text moves answers, as an architect who runs a security review. The review ran against main `9d652bed` and the release binary. It rated the topic fair and found 0 severity 1, 3 severity 2 and 7 severity 3 issues. The full detail sits in the architect review report 273, 12. Work file names in that report refer to its local work folder, which stays unpushed.

Severity 1 means a wrong answer, data loss, a security problem or a hang. Severity 2 means a broken guarantee or a misleading document. Severity 3 means a sharp edge or a missing feature an integrator needs.

## 1. Every surface writes judged text to disk by default, and the library and SQL READMEs do not say so (severity 2)

Evidence. The cache is on by default for the command, the libraries and the SQL extensions. It stores the full evidence in plain text with no expiry (`recording.md:32`). The reviewer verified entry content in a test cache folder. The main README warns about this. A `grep` of every `libraries/*/README.md` and `databases/*/README.md` for any statement that entries hold the judged text found none. In PostgreSQL the folder belongs to the server's operating system user and is shared by every role that can call the functions. That places row text outside the database's own access control, row-level security included. The folder marker binds only the adapter and the URL (`recording.md:36`), so two keys or roles share answers, and `meta.cached` tells a caller that someone else already judged the same text.

What an integrator hits. A database administrator adds `thinkthen_decide(body)` to a query over customer rows and later finds a growing folder of those rows under the postgres home folder. A Python service writes every message it judges under its user's cache.

Direction. State the default and its content on every library and SQL README and in the settings table. Consider defaulting the cache off for SQL extensions, or on only when an administrator names a folder. Consider an expiry setting.

Settled. Ticket 0163 put the disclosure on every README and the settings table. Ticket 0318 turns the SQL extensions' platform cache off, so a SQL cache runs only in a folder an operator names, and documents that every PostgreSQL role shares a named folder. Expiry waits for the clearing ticket ADR 0111 names.

## 2. Anyone who can write a named cache or recording folder decides the answers (severity 2)

Evidence. Explicit `--cache`, `--record` and `--replay` folders at mode `0777` were accepted. Only the default folder is checked for `0700` (`recorder.rs:137-139`). Entries carry no integrity check. Changing `"noul": 0.93` to `0.01` in one entry flipped the next run from `true` to `false`, with `requests_sent: 0` and `cached: true`. `--replay` gave the same result. The spec treats every file field as untrusted text for printing (`recording.md:62`), and it says nothing about the folder deciding answers.

What an integrator hits. A shared `THINKTHEN_CACHE` on a team volume, a CI cache restored across branches, or a committed recording edited in a pull request silently changes gate decisions.

Direction. Say in `recording.md` and `SECURITY.md` that a folder's writers control its answers. Refuse or warn on group- or world-writable folders the tool did not create. Consider an optional keyed check on entries.

Settled. `recording.md` and `SECURITY.md` say that a folder's writers control its answers. Ticket 0303 makes the command warn on a named folder with another owner or the other-write bit. Ticket 0318 makes the SQL extensions refuse such a folder. Library callers apply the rule themselves. Ticket 0318 defers the keyed check and records why.

## 3. ADR 0004 still promises that a key never crosses hosts (severity 2)

Evidence. `sdlc/planning/adr/0004-a-backend-is-a-url-and-a-pure-adapter.md` says "A key never crosses hosts... Overriding the URL on the command line drops the key unless the user also names a key variable." Its status line names only ADR 0007 and says "The rest stands." ADR 0010 and `backends.md` replaced the rule, so the key goes to any address the user names. The reviewer verified that `--url http://127.0.0.1:18731/v1` and a configuration `url` both carried the key.

What an integrator hits. A security reviewer reading the decisions believes an override drops the key, and approves a design that points `THINKTHEN_BASE_URL` at a less trusted host.

Direction. Amend ADR 0004's status to name ADR 0010 as superseding the key rule.

Fixed by Quick Fix qf-review-273 in commit `d1835925`, from branch `ticket/qf-review-273`. ADR 0004's status line names ADR 0010, and its amendment of 2026-09-26 states the current rule. The record is `sdlc/records/qf-review-273.md`.

## Severity 3 titles

- No private TLS roots, and a certificate failure reads as a network failure. The TLS stack uses bundled `webpki-roots` and ignores `SSL_CERT_FILE`.
- Local faults are reported as network or bare-status faults: a key with a line feed reads as "could not be reached", and a 302 gives a bare status.
- The configuration file is trusted whatever its mode: a `0666` `config.json` with a loopback `url` received the key. Quick Fix qf-command-edges-and-prune warns when any user can write the file. Quick Fix qf-config-owner-warning adds the remaining Unix owner warning; independent code and dependency review accepted 53ba4edb. This configuration warning finding is settled. The other findings in this issue retain their own status.
- The planted-text guidance is narrower than a reader will take it. `decide.md:92` does not name the one question and one model behind "0.04 or less". Live, a command moved a different question by 0.16 to 0.18, with a confound. Planted claims flipped `decide` on a second question (0.01 to 0.64) and flipped `choose` from shipping to billing (0.78). The default cut of 0.5 turns the H-08 claim into `true` (0.54 live). Direction: name the question and model, say that the default cut is where claims flip answers, and add a hostile fixture for `choose`. Under the batching design, planted text also steers neighbours; see `2026-09-26-batching-design-review-before-0146.md`.
- Ruby result values print caller text. Closed in `closed/2026-09-26-ruby-result-values-inspect-caller-text.md`.
- No release to verify, and the install check will not catch a replaced release. See `2026-09-25-release-and-install-for-0-1.md`.
- The default destination is a third-party service, and nothing says what it keeps. `README.md`, `SECURITY.md` and `specification/` say nothing on the vendor's retention, training or terms.
