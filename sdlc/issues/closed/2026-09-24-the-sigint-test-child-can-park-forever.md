# The SIGINT test child can park forever

Status: closed by Quick Fix `qf-test-deadlines`. Found 2026-09-24 during Quick Fix `qf-parallel-lock-test`. Owner: Claude.

`cli::interrupt::tests::unix::sigint_child` is an ignored test that `partial_prefixes_and_an_armed_follow_up_sigint_use_the_default` starts as a subprocess. It has two modes. In prefix mode (`THINKTHEN_SIGINT_PREFIX`) it prints `ready` and then runs `loop { std::thread::park(); }`. In the second mode (`THINKTHEN_SIGINT_CHILD`) it prints `armed` and then runs the same loop. Only a SIGINT from the parent ends either one.

If the parent stops before it sends that signal, the child parks with no deadline. The parent can stop early when `await_line` fails, when the test run is killed, or when a load harness stops the run. On 2026-09-24 a load generator for that Quick Fix ran the lib test binary in a loop beside `stress-ng --cpu 16`. One run left a `sigint_child --ignored` process that held on for 68 minutes until the coordinator killed it. The parent's `child.wait()` has no deadline either.

A fix gives both loops a deadline. For example, each parks with a timeout and exits with a failure after about 30 seconds. The parent also kills the child when its own assertion fails.
