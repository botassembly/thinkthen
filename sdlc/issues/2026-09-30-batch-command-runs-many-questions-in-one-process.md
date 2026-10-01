# A batch command that runs many questions in one process, after 0.1

Status: open. Filed 2026-09-30 on Ian's request. Owner: none until 0.1 ships.
Kind: idea
When: after the 0.1 release.
Milestone: later

Today each verb asks one question of many records. A benchmark of many different questions starts one process per question, so the requests-a-minute pacer, the question cache and request batching each cover only one case, and the processes add their rates together.

Idea: `thinkthen batch jobs.jsonl` reads one job per line and runs them all in one process. A line names the verb, the question text or a question file, the input, and the verb's options, for example `{"verb":"choose","question":"...","input":"...","options":["a","b"]}`. It prints one result per job, in input order, with the result object each verb already prints. The file may also carry `requests_per_minute`.

Ian wants the Beatles Bench and release QA to run their end-to-end capability tests through it. A design should match the bench's case format so no converter is needed, and should keep a small set of direct per-verb and per-surface runs, since `batch` alone would not prove each command and binding.

Limits a design must keep:

- Each job gives the same answer it gives through its own verb, with the same question key, so cached answers carry over.
- One bad job fails only that job, with its own exit reason in its result.
- Key secrecy and the recording rule apply as they do for every verb.

Related: `2026-09-30-proxy-service-for-shared-limits-and-traces.md` covers a limit across machines.
