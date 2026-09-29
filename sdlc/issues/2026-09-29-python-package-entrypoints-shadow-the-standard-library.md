# Python package entrypoints shadow the standard library

Status: Open for fresh code review and integration. Five source package gates run a local `types.py` file directly: C# `tests`, JVM `tests`, Dart `checks`, Ada `checks`, and Swift `Tests/fixtures`. A clean Python interpreter puts that directory ahead of the standard library, so `importlib.util` reaches the local file while stdlib `functools` imports `types`. The recursive import fails before a J1 assertion or backend call. Ambient site startup or `PYTHONWARNINGS` can preload the stdlib module and hide the defect; a warm gate pass does not prove clean startup.

At main `68de4aa36`, direct `python3 -S` entries with minimal environment and no preimports showed the local-file recursion in all five. The first five-output probe piped to `head`, so its pipeline statuses are not script exit receipts; a separate complete C# invocation exited 1 with `ImportError: cannot import name 'MappingProxyType' from partially initialized module 'types'`. The [preparation](../records/2026-09-29-clean-python-package-entry-preparation.md) maps each gate, parent interpreter, sibling dependency, ratchet and member list. The Objective-C and Zig instances had already received the same rename method in the five-package batch, which later landed at `e90de34a7`.

Completion criteria for these five normal source entries:

1. Rename only the five local J1 runners to `public_types.py` and update each real `check.sh` invocation. Preserve runner bodies, J1 assertions, sibling imports and product archive inventories.
2. Demonstrate direct script startup with the script directory on `sys.path` under a clean interpreter, without `runpy`, stdlib preimports, `PYTHONWARNINGS` or `PYTHONSAFEPATH`. Distinguish the earlier `-S` red diagnostic from ordinary gate invocation.
3. Run each selected J1 runner directly with its actual matching host/native artifacts and saved-response loopback. Retain exact schema/runtime counts per executed host, with C#/JVM/Java/Kotlin/Scala, Dart, Ada and Swift named separately. Record missing build prerequisites or unrelated failures honestly; a startup proof alone does not qualify a package gate.
4. Measure all affected configured Python ratchets, run focused shell/syntax/policy/pages/tickets/diff checks, and obtain fresh independent code review. Do not claim full package, release runner or publication qualification from these focused results.

Only the normal source branches invoke these J1 files; the installed-release branches use other checks. The broader nine-package issue and its table counts remain under coordinator ownership. SQL, DataFrame and Python frame work remain held.
