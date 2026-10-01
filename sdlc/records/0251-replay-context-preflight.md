# 0251 replay miss context preflight

Status: design preparation only on `3918bba242ce2d1da221987d3fabfa35cc33acb9`; no source or public page changed. Original: local experiment 284's `102-replay-miss-does-not-say-why.md`. Its success test asks for **record N, the question set's name when a file named it, and the exact-match digest** so fixture repair takes one read. It does not ask for a breakdown of which digest component changed. The older 0221 ticket and cache-replay preparation inflated that last idea; the current plan row correctly withdraws it.

## Current carriers and unmet literal criterion

`cli/failure/recording.rs` prints the exact entry digest and a closed command/request label. `cli/failure/stopped.rs` prints the stopped record or the full packed request range; `annotate_schedule.rs` carries its input record position and group ordinal/member count into the replay cause. `cli/annotate/batching.rs` attaches the same group context to packed calls. `tests/backend/recordings/replay_context.rs` executes seven distinct request shapes, including a streaming row and packed range, at exit 5 with empty stdout, zero loopback requests and an absent replay folder; its existing-folder miss test compares the folder before and after and checks the exact digest. This is the reviewed 0221 implementation at `4a740fdc`, present on this source revision.

`core/question_set.rs::QuestionSet::parse` accepts top-level `version`, `threshold`, `profile`, `questions` and `batch`; it rejects an unknown top-level `name`. Its `questions` keys name individual members, not the set. `annotate` accepts a caller-controlled file path, and one group may contain several members. The filename, path and member strings are not safe secret-free labels merely because member names have a restricted alphabet. `cli/annotate.rs` knows the file path; the scheduler knows the record and group, but no producer has a declared set name. Public and CLI errors intentionally avoid echoing evidence, credentials and arbitrary paths. An entry digest is a repair key, not a proof that a particular request component differed.

**Coordinator proposal pending High review:** retain 0221's safe group ordinal/count, stopped record or packed range, and exact digest. Explicitly decline printing a caller-controlled question filename, path or member. The original set-name criterion remains unmet; the absence of a top-level name field does not satisfy it. Propose `nonissue / disagreed approach`, not `done` or already fully fixed. Do not manufacture a name from basename, member, key, evidence or URL, or add an opaque path hash. A later materially justified safe naming design may reopen the issue; no new grammar or API is implied now.

No new runtime proof is needed for this disposition. The existing replay-context cases cover first-request failures with empty stdout. A later two-record **streamed** annotate proof, if a behavioral change warrants it, must preserve record 1's completed stdout prefix when record 2 misses; it would pin ordered prefix, record 2, group context, exact digest, exit 5, zero replay sends, secrecy and an unchanged folder. `cli/annotate` uses `Output::Streaming::take` to write completed rows. No current source or test claim is proposed.

## Review risk

The accepted 0221 ticket says component diagnosis remains, while the original issue does not. The first preflight wrongly treated the absent schema name as satisfying a conditional literal criterion and would have asserted empty stdout after a completed streamed row. The corrected disposition records both limits without claiming that the original set-name request was implemented.

## Independent review and ruling

Fresh High review accepted corrected candidate `0327a13d`. The coordinator retains safe record/range, group and digest context and declines caller-controlled filename/member output under the accepted secrecy rule. Register 102 is non-issue by disagreed approach; its literal name criterion remains unmet. The review correction also preserves already completed streaming output on a later replay miss. No runtime change or new check is claimed.
