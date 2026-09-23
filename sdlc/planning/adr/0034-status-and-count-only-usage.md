# ADR 0034: Read-only status and count-only usage

- Status: Accepted by ticket 0063
- Date: 2026-09-22

## Decision

`thinkthen status` reports one fixed human shape, and `status --json` reports the same resolved configuration, cache inspection, and usage totals as `thinkthen.status/1`. It never outputs, formats, or persists the key value, sends nothing, and changes nothing. Missing platform homes use the documented unavailable values.

The engine owns private process counters for HTTP attempts, validated live input and output tokens, and eligible answer-cache hits. The command persists those counts in UTC monthly `thinkthen.usage/1` aggregates under the platform sibling `thinkthen-usage`. One permanent lock and atomic replacement make each update constant cost and safe across processes. Strict readers validate recognized files, private modes, opened-object identity, checked sums, and the cache folder gate.

Persistence is observational. A missing platform path is silent. The first persistence failure disables later attempts in that process and prints one warning after ordered judgment results. It never changes a judgment or exit meaning. A crash can leave a conservative overcount of one request or an undercount of later tokens. The totals enforce no budget and claim no provider billing authority.

Ian can overturn the public field names, UTC period, sibling path, explicit-replay exclusion, and warning policy. The ruled count categories, count-only storage, no budget, and offline status stand.
