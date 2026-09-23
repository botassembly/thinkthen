# ADR 0035: Recording folders bind to one backend

- Status: Accepted by ticket 0065
- Date: 2026-09-22

## Decision

The first write-capable use of a new or empty recording folder stores one private `.thinkthen-backend.json` marker before key lookup or network access. The marker holds a closed schema and the SHA-256 of the canonical adapter name and resolved endpoint URL. It excludes the model. Requests for several models may share one folder.

A later backend mismatch fails locally at exit 5 before entry lookup, key lookup, or network access. A nonempty folder written by an older version remains available to exact read-only replay and refuses new writes until the user chooses a new folder. Replay alone never creates a marker. Its miss message says that adapter, address, and request form the entry name.

Marker publication follows the recording entry rule: a private complete temporary file is synced, linked into the final name without replacing a winner, and followed by a directory sync. A strict bounded reader refuses malformed, oversized, replaced, symlinked, and non-regular markers without printing their bytes. Status and prune count only exchange entries, and prune preserves the marker.

Ian can overturn the old-folder compatibility boundary and the fixed marker name. The local preflight, canonical identity, model exclusion, private atomic storage, and secret-safe refusal stand.
