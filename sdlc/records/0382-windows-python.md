# 0382: Windows Python wheel

Adds the win_amd64 abi3 .pyd wheel, its installed-package gate and five-wheel release inventories. Existing Unix API/wheels remain covered. A narrow Windows readable-memory adapter uses the existing pinned windows-sys dependency; production-wheel smoke checks package imports, panic-hook isolation, no-send refusal and shared examples.

Linux Python behavior/safety/conformance, installed-wheel and workflow/package/process checks passed. One whole-change High review found a Linux procfs thread-count test enabled on Windows; it now skips where procfs is absent, while portable cancellation tests remain. The affected worker-drop and no-send tests passed after the fix. No other blocking source findings were reported. Full tests and lint passed on landing commit c2840ffe3.

Real Windows ABI, memory boundaries, import, MSVC consumer, privacy, replacement and interruption still require the authorized combined run after core completion. Source review and Linux execution do not satisfy those criteria. C DLL-only staging is within the approved scope; static localization remains deferred. No publication was performed.

The first native Windows dispatch failed before preparation completed: pandas 3 required an unpinned tzdata dependency. The development requirements now use the existing pandas 2 tzdata version and hashes. A fresh High review accepted the fix; hash-checked Windows wheel downloads passed for both dependency locks. Native execution must be rerun before claiming readiness.

The next native run built and installed the wheel and passed shared replay cases and examples, then stopped at Windows-specific Arrow lint errors. The correction separates platform methods without changing pointer arithmetic, bounds, refusals or VirtualQuery, and scopes Linux-only imports to their consumers. One fresh High review accepted the complete change. Policy, both Clippy profiles, 19 Rust Arrow tests and four Python Arrow safety tests passed; source grew by one line. Full landing tests and lint run before push. Native Python memory boundaries remain for the combined Windows run.
