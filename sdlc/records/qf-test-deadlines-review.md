# Review of Quick Fix qf-test-deadlines

Reviewer: a fresh read-only Opus session that did not write the work. It reviewed `5dbf9649` and returned ACCEPT. The next commit applies findings 1 to 3.

Findings:

1. The standard library's `wait` closes the child's input before it waits, and `finish` did not. Every current caller drops the input first. A future caller that left it open would hang for 60 seconds. `finish` now closes it first.
2. The follow-up issue told a test that uses `output()` to switch to `spawn()`. `spawn()` passes on the parent's input, and `output()` gives an empty one. The issue now names `.stdin(Stdio::null())`.
3. A trailing "which" clause in the record broke the workspace writing rule. It is now two sentences.
4. Note only: the 60-second limit covers the child's exit and not the join of its output readers. A grandchild that kept a pipe open would still block. No current call site starts one.

Checks:

- `finish` reads stdout and stderr on their own threads before it waits, so a full pipe cannot deadlock. A missing pipe gives empty output. On expiry it kills and reaps the child and returns `TimedOut`. `park_for_signal` survives a stray wake-up.
- Each call site keeps its exit code, signal, output fields, and drop order. The 2-second loops and the waits after a kill are unchanged.
- The 60-second child limit is twice the tool's 30-second request timeout. The backend binary took 37.8 seconds at a load of 3.4. The 30-second signal limit is far above the parent's reaction time.
- The `#[path]` share is sound. The file uses only the standard library and `pub(crate)` items.
- The error text names test arguments. The key reaches children only through the environment, and test arguments carry only fake values.
- `--lib`: 330 passed, 8 ignored. `--test backend`: 358 passed. The ratchet read 50377/50377.
- The reviewer compared the planted-bug results with the code paths and did not repeat them.
