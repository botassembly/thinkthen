# Review leftovers from ticket 0008

Status: Closed. Point 1 landed in ticket 0046, and point 2 landed earlier: every row carries `meta.tool`.

Found 2026-09-19 by the independent review of ticket 0008. The review said merge and left two points.

1. Done in ticket 0046. A duplicate case id is refused by `counts`, `score`, `sweep`, `band`, and `calibration`. `trials.jq` averages repeated stored probabilities into one derived row per case before a metric runs. `compare.jq` continues to list repeated ids rather than pairing them.
2. `specification/result.md` says every `meta` field is always present, and ADR 0008 item 3 put `tool` there. The binary does not write `meta.tool` yet, so no committed row has it. No recipe reads it. The next ticket that touches the result object adds it, and the rows are recorded again only if a recipe comes to need it.
