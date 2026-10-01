# The relate menu tests share one rule file and fail at random

Status: closed 2026-09-30 by quick fix `52bfaa049`: each relate menu test writes its own folder, named for the test and process. Under 4 parallel nextest runs of 300 repeats each, 820 of 1,200 repeats failed before and 0 of 1,200 after. Filed by ticket 0360's builder.
Kind: bug

Five tests in `crates/thinkthen/tests/backend/relate/menu.rs` call `file("works-for", WORKS_FOR)`. The helper writes `relate-menu/works-for.json` under `CARGO_TARGET_TMPDIR`, so every test writes the same path. Nextest runs them in parallel. `fs::write` truncates the file before it writes, so one test can read an empty or partial rule and exit 5.

Seen on the 0360 branch after merging origin/main at 6a6ede090. `a_failed_menu_keeps_its_source_and_the_run_exits_six` exited 5 in a full run. On a rerun of `relate::menu`, `a_single_rule_asks_one_described_menu_per_source_and_the_plan_shows_it` failed once. Two more reruns passed. Ticket 0360 does not touch relate or these tests.

Asked: give each test its own file name, such as the test's name, so no two tests write one path.
