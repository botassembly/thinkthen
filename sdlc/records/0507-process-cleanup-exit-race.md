# 0507: Handle process disappearance during cleanup checks

This build starts from `1ba45888c901384a47ac8f291f69ef3cb26d7c9e`. Ticket review accepted revision `4ffc11e39`. The source change is commit `cdf368ec2`.

The existing `alive` helper now treats `FileNotFoundError` and `ProcessLookupError` from its Linux process-state read as an exited process. Its zombie check and portable signal check remain intact. The existing portable runner exercises both read exceptions and checks that its real owned sleeper is alive before testing release-wrapper cleanup. Owned descendant, unrelated-process survival, retained-output and Windows handle assertions remain intact. No runtime process supervision changes.

The added regression exited 1 before the fix with `ProcessLookupError` at the existing read boundary. After the fix, `python3 sdlc/scripts/release-process-cleanup-test.py` exited 0 under an explicit minimal environment. `python3 sdlc/scripts/workflows` exited 0 under that environment. `CARGO_NET_OFFLINE=true python3 sdlc/scripts/policy.py` exited 0 with existing size warnings in unchanged files. The committed diff check and tracked path/content private-name checks passed.

Two broader workflow self-test trials exposed runner setup mistakes. The first omitted `HOME` and exited 1 with 111 of 113 cases passing; existing packaging fixtures reported `HOME: parameter not set`. The second supplied an empty owned home but omitted installed Swift from `PATH`; it exited 1 with 112 of 113 cases passing at the archive fixture's Swift prerequisite. That trial also installed Rust into the owned home because the environment omitted the existing Rust cache paths. It did not satisfy the requested offline execution boundary. The final trial supplied the installed tool paths, existing `RUSTUP_HOME` and `CARGO_HOME`, the owned `HOME`, the UTF-8 locale and `CARGO_NET_OFFLINE=true`; `python3 sdlc/scripts/workflows --self-test` exited 0 with all 113 cases passing. The owned test home was removed afterward.

Native Windows execution and full Rust build gates were outside this Python test repair. The Windows assertions exercised source oracles, as the existing runner states. No hosted workflow, release action or paid call ran.

Fresh read-only source review accepted `e309af8cb329e131522ed1908017ea13570e9146`, checking the helper and its callers, retained cleanup assertions, regression coverage and honest platform limits. The landing retains the reviewed Python source unchanged.

## What the build taught us

A Linux process can disappear after its process-state path exists and before that path is read. The two observed disappearance exceptions belong at the same existing liveness boundary. The added live-child assertion prevents this repair from treating every process as exited.

An offline Cargo flag alone does not isolate toolchain setup. Broad fixture runners also need their declared home, installed tool paths and existing toolchain caches. Their environment mistakes did not require a source change or a larger verification framework.
