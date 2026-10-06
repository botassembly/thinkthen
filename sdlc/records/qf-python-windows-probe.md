# Quick Fix: keep the Python memory probe on the stable ABI

Native Windows run [37386706627](https://github.com/botassembly/thinkthen/actions/runs/37386706627) built the production wheel but failed to link the installed memory probe. Its hand-declared `PyGILState_Check` is absent from the stable ABI import library.

The probe now uses the stable `PyGILState_Ensure` return value to check the caller's original lock state. An originally detached caller aborts before any reference decrement. An attached caller decrements the reference and matches its acquisition with `PyGILState_Release`. The existing exit gate protects worker releases during finalization. [CPython documents these state values and stable functions](https://docs.python.org/3.10/c-api/init.html#c.PyGILState_Ensure). Production keeps `abi3-py310` and excludes every probe hook.

On Linux CPython 3.13.5, a fresh installed probe wheel passed six routine Arrow and release tests. Four stress cases were deselected. A real detached native stream release aborted with `SIGABRT`. The extension imports Ensure and Release and no Check symbol. The production release wheel passed its contents check. Policy, format, probe Clippy and root/Python ratchets passed. The Python Rust counter grows one line to 7,980. No dependency lock changed. Builds used a lane lock, shared cache locks, two Cargo jobs, a 6 GiB memory limit and a 1 GiB swap limit.

Native Windows link and installed memory qualification remain pending. The hosted workflow also writes unignored `native-platform` artifacts inside its checkout before demanding a clean tree for the crate. Those generated files independently explain that final refusal. This fix leaves workflow routing to the coordinator.
