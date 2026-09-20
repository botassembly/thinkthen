# ADR 0020: A recording entry does not change underneath a request

- Status: Decided by the agent on 2026-09-20. Ian can overturn any line
- Date: 2026-09-20

A recording folder keys one response by the exact adapter, URL, and request bytes. The page promises that replay gives the same answer. It also says that recording the same request again replaces the entry and the newest answer wins. Those rules conflict when a backend gives two answers to the same request. In one reproduced run, two identical records answered false and true; one file kept the last response, and replay answered both records with that response.

## Decision

- One digest has one immutable response in a recording folder. The first complete entry installed at its path wins until the user removes that entry or uses another folder.
- Recording the same request and same response again is idempotent. A different response for an existing digest is a local failure, exit 5. The error names the entry and never either response.
- Installation remains atomic and private on a filesystem that supports hard links. A complete closed temporary file in the recording folder is linked into the final name without replacement. If another thread or process wins that name, the loser reads the completed entry and applies the same-response or conflict rule. No reader sees a partial final file.
- `--record` adds missing exchanges. `--replay` reads them. `--cache` reads first and adds only a miss. They share the same version-one entry format.
- A successful recorded run can replay the answers it printed. A run that receives two different answers for one digest fails instead of saving a false history.
- A repeated trial that wants another backend answer uses a fresh folder. Recording folders do not silently refresh evidence.

## Compatibility and consequences

Every existing entry remains readable and keeps its name and schema. It becomes the first answer already present for its digest. A later attempt with the same response succeeds, and a later different response is refused. Request bytes, digests, replay lookup, and committed folders do not change.

This keeps recording and cache storage content-addressed. It does not create an ordered transcript or add occurrence numbers. Exact replay means every successful recorded request resolves to the immutable response stored for its digest. It does not mean a failed run with conflicting duplicate answers becomes replayable as if it had succeeded.

New recording writes now require hard-link support in the recording folder. A filesystem that allowed rename but refuses hard links returns the ordinary local recording failure, exit 5, and leaves the existing entry untouched. Every returned write path attempts to remove its private temporary name. A process crash can leave a complete private dot-prefixed temporary file, as it could before this decision.
