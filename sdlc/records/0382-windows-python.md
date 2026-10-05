# 0382: Windows Python wheel

Adds the win_amd64 abi3 .pyd wheel, its installed-package gate and five-wheel release inventories. Existing Unix API/wheels remain covered. A narrow Windows readable-memory adapter uses the existing pinned windows-sys dependency; production-wheel smoke checks package imports, panic-hook isolation, no-send refusal and shared examples.

Linux Python behavior/safety/conformance, installed-wheel and workflow/package/process checks passed. One whole-change High review found a Linux procfs thread-count test enabled on Windows; it now skips where procfs is absent, while portable cancellation tests remain. The affected worker-drop and no-send tests passed after the fix. No other blocking source findings were reported. Full tests and lint passed on landing commit c2840ffe3.

Real Windows ABI, memory boundaries, import, MSVC consumer, privacy, replacement and interruption still require the authorized combined run after core completion. Source review and Linux execution do not satisfy those criteria. C DLL-only staging is within the approved scope; static localization remains deferred. No publication was performed.
