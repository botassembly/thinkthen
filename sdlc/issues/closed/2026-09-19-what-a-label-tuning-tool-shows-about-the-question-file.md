# What a label-tuning tool shows about the question file

Status: Closed on 2026-09-25 as reference. No work is owed. A survey with no work owed. Earlier status: Reference. A survey. No work is owed.

Found 2026-09-19. Ian pointed at an open-source tool, new that week, that tunes the wording of a question for the same hosted decider. It samples rows, asks a person to label the unsure ones, and has a second, generating model rewrite the wording until the labels score better. A survey agent read its source. Nothing here is built, and each item names where it would go.

## The wire carries more than `decide` sends

The vendor's yes/no question accepts `criteria` with a `true` text and a `false` text, beside the question. That tool sends both and tunes both. `decide` sends the question alone, and `specification/backends.md` has no row for it. A question file could carry them:

```json
{"decide": "Does this message ask for a refund?", "yes": "The customer asks for money back.", "no": "The customer asks about anything else.", "threshold": "0.2:0.8"}
```

This gives the question file a reason to exist beyond saved typing. It needs a live check first: the same labeled cases, with and without the two texts, through `sdlc/scripts/live`. It would ride with the question file of ADR 0013's amendment.

## The two tools fit end to end

That tool tunes the words and knows no cut: it applies 0.5 everywhere. `thinkthen` tunes the cut with the sweep transform and runs the question in a shell. A converter of about forty lines maps its saved state to a question file: its instructions become the question, its true and false texts become `yes` and `no`, its named criteria become `options`, and its levels become `levels`. Nothing maps to `threshold`, and its run history has no place in a question file. The tool is a day old and marked alpha, so nothing is built against it. The idea belongs in `roadmap.md` when the question file lands.

## Worth borrowing

- **Send a person the unsure rows and a small random share of the sure ones.** The random share catches quiet drift that a band never sees. This is a policy transform for the how-to on watching a pipeline, slice 10b.
- **Write state files by temporary file and rename.** It matters for `--record` when a run is killed. Check what `recording.md` promises before the release pass.

## Mistakes to avoid

- It scores a rewritten question on the same cases it tuned on. The eval how-tos keep a tuning half and a held-out half, as how-to 13 already does.
- It has no retry and no bound on the vendor's rate limit. `backends.md` already has both.
- It fixes the cut at 0.5. The band is the point of `decide`.
