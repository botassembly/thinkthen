# The disk cache is never on unless the user names a folder

Status: Open

Ian asked on 2026-09-21 whether the cache is always there. His worry: a side effect nobody asked for is strange to people. This page states the rule for all ten surfaces, so the ADR 0017 rewrite and step 4 of the engine plan have it. It authorizes nothing.

## What is true today

The command saves nothing by default. `--cache DIR` turns the cache on, and the user names the folder. `specification/recording.md` holds the rule.

The planning pages for the libraries and the databases are less clear. `sdlc/planning/databases/README.md` rule 5 says "Answers are kept on disk" and "Tomorrow's run of the same query costs nothing". A reader takes that as on by default. No page says where the folder would be.

## The rule

Two kinds of saving, with two defaults:

| Kind | Where it lives | Default | Why |
| --- | --- | --- | --- |
| Asking once inside one call | Memory, for the length of one bulk call or one query | Always on | Equal pairs of question and text in one batch are one judgment. It writes nothing and outlives nothing. Nobody can observe it except on the bill |
| Answers kept across runs | A folder on disk | Off until the user names the folder | See below |

Three reasons hold the disk cache off by default:

1. `CLAUDE.md` says the tool "never writes a file the user did not name". A cache folder the library picked is such a file.
2. A cache entry holds the text that was judged. A default folder would copy a user's messages or records to a place they never chose.
3. `jev-latest` is an alias. A saved answer outlives a model change, and a user who never asked for a cache would not know to clear one.

Every surface spells the setting its own way: `--cache DIR` in the command, a `cache` setting on the engine value in a library, and a session setting in a database. No surface picks a folder on its own. The section on always-on below adds one variable, and the user sets it.

## What this changes in the database pages

The memory rule alone covers the measured DuckDB case, where one call in `WHERE` and again in `SELECT` cost 8 requests with no cache and 4 with one, if the memory lasts for the query. "Tomorrow's run costs nothing" holds only after the user sets the folder. Rule 5 and `thinkthen_warm` need that sentence. `thinkthen_warm` with no folder set fills the memory for the session, and its page says so.

## How the cache stores and finds an answer, and where it stops scaling

Ian asked on 2026-09-21 how the cache works, how fast it is, how large it grows, and when an entry expires. The answers, from `specification/recording.md` and two measurements:

- **Storage.** One JSON file per request, in one flat folder, named by the SHA-256 of the adapter, the address, and the request bytes. It uses no database and no index file. The file system is the key-value store.
- **Finding.** The tool computes the digest and opens that one path. It never lists the folder.
- **Speed.** 100 whole runs of `decide --replay` on the refund how-to took 0.155 s on 2026-09-21, about 1.5 ms each with the process start included. A live request takes some hundreds of milliseconds.
- **Size.** No limit. A short entry is about 500 bytes and takes one 4 KB block on disk. The accuracy round's folders measure it: 2,940 entries in 12 MB, and 770 entries with 77 options each in 6.2 MB.
- **Expiry.** None. The first answer for a digest stays until the user deletes the file. A changed question, text, model name, or address is a new digest, and the old entry is never read again. `jev-latest` is one model name, and an answer saved under it outlives a change of the model behind it. A user who wants a clean break pins the model or clears the folder.

This design suits the command. An entry diffs in a pull request, a test folder is plain files, and deleting a file is the whole expiry rule.

It stops suiting a database somewhere. A million judged rows is a million files and about 4 GB in one folder. Nothing here has measured that. Before the database extensions promise "answers stay on disk" at that size, the experiment team measures a folder of a million entries: the time to find one, the time to fill, and the disk used. If it fails, the engine gets a second store behind the same setting, one file in place of a folder, and the recording folder stays as it is for tests. No code changes before that number exists.

## Size, age, and always-on

Ian said on 2026-09-21 that a cache with no size rule and no expiry is not tenable, and he asked four things: how entries leave, whether leaving slows a request, how a user turns the cache on for good, and what sits in the XDG folders. A recommendation follows. It needs an ADR before any ticket.

**What sits in the XDG folders today: nothing.** The tool reads two variables, `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL`, and no file. `specification/roadmap.md` records that Ian took the configuration file at `$XDG_CONFIG_HOME/thinkthen/config.json` out of version one on 2026-09-19, because a variable in front of the command already said everything it held.

**Always-on is a third variable, `THINKTHEN_CACHE=DIR`.** The user writes it in a shell profile once, and the user still names the folder. It follows the ruling of 2026-09-19 and brings no file back. `--cache DIR` overrides it, and `--no-cache` turns it off for one run. `--replay` and `--record` ignore it, because a test must never read a personal cache. The engine reads it for the libraries and the databases the way it reads the other two, and an explicit setting on the engine value wins. If a default folder is ever wanted, it is `$XDG_CACHE_HOME/thinkthen`. The state folder is wrong for it, because a cache must be safe to delete.

**No request ever deletes anything.** A request reads one file or writes one file. Removal is its own command, `thinkthen cache prune DIR`, with `--older-than 30d` and `--max-size 1G`. A person, a cron line, or a host program runs it. No request pays for another entry's removal, and there is no background thread to reason about in a forked host.

**The oldest-written entry leaves first.** Least-recently-used needs a write on every read, and it turns the fast path into the slow one. The file's own modification time is the age, and it costs nothing to keep.

**A recording is never pruned by accident.** The cache and a test recording share one format. `prune` runs only on the folder a person names, and the how-tos say never to point it at a recording.

**The model behind an alias.** Each entry already stores the model that answered, `jev-1.13.0` in the how-to recordings, beside the alias that was asked. `prune --answered-by-other-than MODEL` clears what an older model said. Asking the backend which model is current on every cached read would defeat the cache.

**A million entries.** A folder sharded by the first two characters of the digest, the way Git stores objects, is the cheap fix, and it keeps plain files. The experiment team measures the flat folder, the sharded folder, and one SQLite file at a million entries before anything is chosen.

Other concerns the ADR must answer:

- A cache holds the judged text at rest. The folder is private to its owner. The pages say so plainly for anyone with regulated text, and they say that backups copy it.
- A cached answer is frozen. A live model may answer 0.79 today and 0.81 tomorrow. The cache makes a run repeatable and hides that drift. `--details` already marks a replayed answer.
- The threshold is outside the key. Changing a threshold re-reads the cache for free, and this is worth teaching.
- A failure is never saved, so a bad minute at the backend cannot poison the folder.
- The per-digest lock needs a file system with working locks. A network share may lack them, and the page says so.
- A cache folder keeps one lock file per entry under `.locks/`, seen on 2026-09-21: 19 entries and 19 lock files after a `--cache` run. A million entries is then two million files. `prune` clears the lock of every entry it removes, the million-entry measurement counts both, and a user who commits a recording needs the `.locks/` ignore rule this repository already has.
- A prune that runs beside a live job is safe. A reader that loses its file asks again.

## A cache library as a fourth arm, 2026-09-21

Ian asked experiment 211 to add foyer, a Rust cache with a memory tier, a disk tier, a byte capacity, and first-in-first-out eviction, as a fourth arm beside the flat folder, the sharded folder, and SQLite. The run already asks whether it fits a blocking engine and whether it survives a fork. Four more questions decide it as much as speed does:

1. **Many processes on one folder.** The command is a new process on every call, and `xargs`, cron, and PostgreSQL backends put many of them on one cache at once. The plain-files store is built for that: an entry lands by hard link, and a per-digest lock stops a double send. A store that expects one owning process fails here, however fast it is inside that process.
2. **The cost of opening.** One whole `decide --replay` run takes about 1.5 ms today, process start included. A store with a recovery step pays it on every command.
3. **One format or two.** A cache entry and a test recording are the same file today, and `--cache` resumes a record run for that reason. An opaque store splits them: recordings stay plain files for tests and pull requests, and the cache becomes something a person cannot list, read, or delete by entry. Pruning by the model that answered then needs its own index.
4. **The dependency.** It brings an async runtime into a tree that holds zero threads between calls, and every crate passes `deny.toml` and the size ratchet.

A long-lived host may still want answers in memory. A small map in front of the plain files gives it that with no new dependency, and the run can measure that arm too.

## What Ian can overturn

All of it. The wider choice is a disk cache on by default in the databases alone, under a folder the extension documents. It saves a second bill for a user who never read the page, and it breaks the three points above.

## The fourth arm's answers, 2026-09-21

Foyer 0.22.6 ran at both payload sizes beside the other three arms (256 MiB memory tier, `FifoPicker`, a one-worker runtime; every number in `experiments/211-thinkthen-blocking-engine/cache/NOTES.md`). What it wins, measured: fills ten times faster (2.8 s against 30.2 s for a million 457-byte entries), a warm read at 0.31 µs from the memory tier, a hard byte capacity whose FIFO eviction lost no entry at the cap, and a million-entry delete in half a second. What it costs: no blocking API (build, recovery, every cold read, flush, and close are async), two resident threads when quiet and bursts of 14–129 during recovery and flush where the folders hold zero, best-effort writes that silently dropped 59% of an unpaced fill until the submit queue was paced, 428–439 ms of index recovery on every open, a cold read at 22.5 µs against the folder's 5.7 µs, and a fork smoke in which the child hangs on any read the memory tier cannot answer — a written-but-never-read key is a phantom, and the write queue's executor thread did not survive the fork.

The four questions above, answered in order:

1. **Many processes:** foyer expects one owning process; its region files carry no cross-process coordination. The plain-files store keeps the hard-link-and-per-digest-lock answer.
2. **The cost of opening:** 428–439 ms of recovery on every open, against a whole `--replay` run at about 1.5 ms today. Disqualifying for the command's shape.
3. **One format or two:** the regions are opaque. Recordings would stay plain files while the cache becomes unlistable, and pruning by the model that answered would need an index nobody owns. Two formats, as feared.
4. **The dependency:** an async runtime with resident threads inside a tree that holds zero threads between calls, plus exposure to the size ratchet and `deny.toml`.

Verdict: the sharded plain-files store stays the default and SQLite stays the fallback; foyer does not ship for this engine. It is the measured candidate if the extensions ever need a store that enforces its own byte budget in-process, and these numbers say what that costs. The memory tier's warm read is available with no dependency: the engine's always-on per-call memory already gives one, and a plain map in front of the files covers a long-lived host.
