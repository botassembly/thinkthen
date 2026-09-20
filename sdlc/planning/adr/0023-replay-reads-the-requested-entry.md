# ADR 0023: Replay reads the requested entry

- Status: Decided by the agent on 2026-09-20. Ian can overturn any line
- Date: 2026-09-20

`specification/recording.md` says any non-entry file under a replay folder causes exit 5. The implementation does not scan the folder. It derives one filename from the requested exchange digest and reads only that entry. The first hands-on test confirmed that replay succeeds when an unrelated `notes.txt` sits beside the valid requested entry.

Scanning would make one exchange depend on every unrelated file in the folder. It would also inspect files that the request does not authorize replay to read. Digest-directed access keeps replay bounded to the exchange the user requested and avoids reading unrelated evidence or other private contents.

## Decision

- Replay derives the requested entry's digest filename and reads only that path. It does not enumerate or validate other files in the folder.
- An unrelated file, including a file that is not a recording entry, is ignored.
- Missing behavior stays unchanged. Under `--replay` alone, an absent requested entry is exit 5 and names the missing digest entry. In cache mode, an absent requested entry remains a miss that may be fetched and recorded.
- Damaged behavior stays unchanged. A requested entry that cannot be parsed is exit 5. Its diagnostic names the digest entry and safe JSON line and column without repeating file contents.

## Consequences

A stray or damaged unrelated file cannot break an otherwise valid replay. Replay also does not serve as a folder audit: a bad file is discovered only when its digest is requested. Tests pin both the ignored unrelated file and the safe refusal of a damaged requested entry. The recording page states this boundary.
