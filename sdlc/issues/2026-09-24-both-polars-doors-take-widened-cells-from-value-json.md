# Both Polars doors take widened cells from value_json

Status: Open. Found 2026-09-24 by the design review of ticket 0120 (the Rust Polars surface). Owner: Claude.

Ticket 0106 (Python Polars, design accepted) moves the tag's frame writer from `libraries/python/src/arrow.rs`. When a question fails for any row, that writer widens the question's column to text. It wrote a widened score with Rust's `f64` display, so 1.0 became `1`. It wrote the failed marker through the stand-in's serializer for `Failed`. The real `thinkthen::Failed` has no serializer, so 0106's builder must pick a new source for the marker.

Ticket 0120 takes every widened cell's text unchanged from `AnnotatedRecord::value_json` (0095), lifted as a `serde_json` `RawValue`. That is the engine's one serializer. If 0106 ports the tag's code, the two Polars doors write different number text, and each keeps its own marker code.

The fix: 0106's builder takes widened cells from `value_json` the same way. The column table both doors follow:

| Question | Column | Widened cell when any row failed |
| --- | --- | --- |
| decide | `Boolean`, null for not sure | `true` or `false` from `value_json`, null for not sure |
| choose | `String`, null for nothing fits | the plain label, null for nothing fits |
| score | `Float64` | the number text from `value_json` |
| tag | `String` holding the JSON array text | the array text from `value_json` |
| failed | | the marker from `value_json`, such as `{"failed":{"kind":"backend","cause":"missing_probability"}}` |

Whichever Polars door lands first writes this table as the Polars item of ADR 0047, and the other cites it. Ian can overturn the table.
