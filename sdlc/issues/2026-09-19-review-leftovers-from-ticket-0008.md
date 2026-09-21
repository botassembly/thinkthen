# Review leftovers from ticket 0008

Status: Open. Point 2 is done: every row carries `meta.tool`. Point 1 waits for the repeated-trials transform in the builder's plan of 2026-09-21.

Found 2026-09-19 by the independent review of ticket 0008. The review said merge and left two points.

1. A duplicate case id is counted twice by `counts`, `score`, `sweep`, `band`, and `calibration`, and nothing warns. Only `compare.jq` notices, and it then lists the id under `only_in_before`, which reads oddly. ADR 0008 says rows that share an id inside one run are repeated trials, and a metric averages within a case first. Slice 10b gives every metric recipe that rule, and until then each recipe should report duplicate ids the way it reports unlabeled cases.
2. `specification/result.md` says every `meta` field is always present, and ADR 0008 item 3 put `tool` there. The binary does not write `meta.tool` yet, so no committed row has it. No recipe reads it. The next ticket that touches the result object adds it, and the rows are recorded again only if a recipe comes to need it.
