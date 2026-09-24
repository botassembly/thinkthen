# 0115: Question-form runner guards

Status: landed on `ticket/0115-runner-guards`. A fresh review accepted `7ac7bd81` (`sdlc/records/0115-review.md`).

## Result

- Each question-form case runs in a child of the test binary (`form_child`). The runner removes every `THINKTHEN_` variable whose name holds `KEY` or `URL` from the child environment. The command runs in process, and the crate forbids `unsafe`, so a child process is the only place the runner can remove the key.
- The child passes `--url` for a loopback listener that counts connections, and it requires zero. The runner requires the child's `form-child sees []` line, so a renamed child cannot pass silently.
- `the_runner_hides_a_key_and_an_address_from_its_children` starts the runner with `THINKTHEN_API_KEY=test-key-not-real` and `THINKTHEN_BASE_URL` set to a counting loopback listener. The child asks a valid question.
- A ported mutation drops `evidence` from case 29.
- Ratchet +150 over main: the child harness, the counting listener, and the guard test. After the rebase onto `6eb1303e` it moves from 47007 to 47157.

## Red and green

| Check | Before | After |
| --- | --- | --- |
| Guard test, child lines | `sees [THINKTHEN_API_KEY THINKTHEN_BASE_URL]`, `requests 1`, `says thinkthen: the backend closed the connection before a reply and may have received the request; it was not sent again` | `sees []`, `requests 0`, ``says thinkthen: the environment variable `THINKTHEN_API_KEY` is unset or blank, so no key is sent`` |
| Planted listener named by `THINKTHEN_BASE_URL` | 0 connections | 0 connections |
| Evidence rule removed, without the new mutation | all conformance tests green | |
| Evidence rule removed, with the new mutation | `ported mutation 10 passed`, test fails | rule restored: green |

"Before" is the child spawn without `env_remove`. The one request went to the child's loopback listener. The command never reads `THINKTHEN_BASE_URL` in process, so the planted listener saw nothing in either run.

## Checks

- `install`, `lint`, `test`, and `spec` exit 0 with `THINKTHEN_API_KEY` and `THINKTHEN_BASE_URL` unset.
- One `test` run failed in `annotate::scheduling::one_global_queue_bounds_document_and_stream_requests_at_jobs_1_4_and_32` (`document at 4`, the peak-above-one check) with the one-minute load near 10. It passed 5 of 5 alone, and the rerun of `test` exited 0. This ticket does not touch `tests/backend`.
- `sdlc/scripts/live` did not run.
