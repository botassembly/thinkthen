# 0085: Build the real engine facade

Status: in progress. Owner: Claude.

## Call-path inventory before the first code change

Tree: the branch after merging main `86f4012e`. Every owner below is private.

| Concern | Owner on this tree |
| --- | --- |
| Question files, typed values, plans | `core` (`question_file`, `question_set`, `plan`, `find`, `recognize`, `relation`, `relate_file`) |
| Answer rules, thresholds, ranking, the find selector | `core` (`answer`, `threshold`, `order`, `find`) |
| Recognition assembly and relation planning and edges | `core` (`recognize::assemble_names`, `relation::{plan_relation, plan_pairs, assemble_edges}`) |
| Request encoding, digests, the backend-limit split | `engine::prepared_request` (`PreparedRequest`, `PreparedRequests`, `SettledRelation`) |
| Replay, recording, cache locks | `engine::recorder`, `engine::cache_lock` |
| Transport, retries, the width gate | `engine::http::Client`, `engine::Widths` |
| One request through replay, transport, and recording | `engine::request::{ask_profile, ask_prepared}` |
| Ordered record scheduling and grouped scheduling | `engine::schedule`, `engine::annotate_schedule`, `engine::workers` |
| Counters | `engine::usage::Counters`, owned by the command's `Environment` |
| Cancellation and deadlines | `engine::Cancel` |

What the command held beside those owners, before this ticket:

| Function | Command path | Direct low-level use |
| --- | --- | --- |
| `decide`, `choose`, `score`, `tag`, `filter`, `rank` | `cli/judge.rs` to `cli/asking.rs` `run` and `Judging::finish_row` | builds `Client` and `Recorder`, calls `asking::ask` to `engine::request::ask_profile`; streams through `cli/schedule.rs` to `engine::schedule::run_cancelled`; the one-document call sends on the calling thread |
| `find` | `cli/find.rs` | builds `Client` and `Recorder`, `PreparedRequest` for the plan, `asking::ask`; sends on the calling thread |
| `annotate` | `cli/annotate.rs`, `cli/annotate_schedule.rs`, `cli/annotate/plan.rs` | builds `Client` and `Recorder`, `PreparedRequests` per group, `asking::ask_prepared`, `engine::annotate_schedule::run` |
| `recognize` | `cli/recognize.rs`, `cli/recognize/relation.rs`, `cli/recognize/dry_run.rs` | the whole staged pipeline (token questions, split, send, token answers, name assembly, relation settle and send, edge assembly) lives in the command; sends on the calling thread |
| `relate` | `cli/relate.rs`, `cli/relate/plan.rs` | relation settle and split, sequential sends, logical-answer bookkeeping in the command; sends on the calling thread |

`cli/asking/request.rs` held the command's two request adapters (`ask`, `ask_prepared`) and the `Asking` chunk loop that `recognize` and `relate` shared.
