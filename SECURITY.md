# Security

Report a vulnerability privately through GitHub: open the repository's Security tab and choose "Report a vulnerability". Do not open a public issue for it.

Include the command or library call, the version from `thinkthen --version`, and what an attacker gains. Never include an API key.

`thinkthen` sends the evidence you give it to the backend address you name, with the key from `THINKTHEN_API_KEY`, or from the selected named backend's own key variable, sent only to that backend's address (ADR 0114). A report about where evidence or the key can travel is in scope. So is a report about a file the tool writes outside the paths [the configuration page](https://thinkthen.dev/install/configuration/#locations) names.

The answer cache is on by default for the command and the libraries. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write a cache or recording folder decides the answers read from it; keep that folder private to the people whose answers it holds.

On Unix, the command warns when an existing named answer folder has another owner or other write permission. A group write bit alone stays quiet, so a group-writable folder is trusted to its whole group; keep a cache that should not trust every member at mode 0700. A quiet folder is not authenticated: ACLs, parent-directory changes and concurrent edits can still change what it serves. The warning is unavailable on non-Unix systems. Library callers receive no automatic warning and must apply the folder trust rule themselves.

The SQL extensions for PostgreSQL, DuckDB and SQLite keep no answer cache unless the operator names a folder, by `THINKTHEN_CACHE` in the host's environment or by the extension's cache setting. A PostgreSQL server runs every role as one operating-system user, so every role whose calls reach a named folder shares its answers. A shared folder takes row text outside the database's access control, row-level security included, and `meta.cached` tells one role that another already judged the same text. Name a shared folder only when every calling role may see every judged row, or give each role its own folder. The SQL extensions refuse, before any request, a named cache, record or replay folder that another user owns or others can write. The same rule as the command's warning applies, so a group write bit alone passes. They create a missing named folder with mode 0700 before that check, so no other user can create it first.
