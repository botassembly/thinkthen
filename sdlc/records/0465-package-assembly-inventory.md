# 0465: Ship the complete npm module inventory

Status: READY FOR LANDING. The reviewed branch is pushed; the coordinator owns landing.

The assembled npm archive now includes `complete.js`, `_complete.js` and both declaration files. A fresh offline install of the actual assembled archive loaded the Linux native addon, imported the public complete API, and compiled and ran the existing CommonJS and ESM public type fixture. Removing installed `complete.js` made import fail as expected. The other three platform entries were assembly fixtures; this check makes no runtime claim for those hosts.

Optional Linux source-package smoke now requires every public family without requiring a Flutter archive that `release-container` does not pack. A separately supplied private Flutter archive still takes the existing Dart/C pairing and installed app route. The smoke preflight fixture reached pair validation without Flutter and refused a missing Dart archive before backend startup.

The first full test run exposed a local ThinkThen configuration entry in the triage demo. A fresh `XDG_CONFIG_HOME` made that demo pass. The next full run reached every binding smoke and found that Dart's selected compiler was not exported to its Python ABI check. Exporting `TT_DART` in the Dart smoke branch made its focused smoke pass. On final source `baf613301`, full tests passed: 1,757 workspace tests, 337 library-only tests, 23 external consumer tests, doctests, supporting checks and all nineteen binding smokes. Full lint and policy passed under 10 GB memory and 1 GB swap limits. Fresh read-only review accepted `93a81e959` and accepted the one-line Dart gate correction at `baf613301` with no findings.

No registry change, hosted run, publication, release candidate or public Flutter archive is claimed. The ticket remains open for landing.

## What the build taught us

Copying the package manifest is insufficient when the release assembler has its own file list. A fresh installed archive catches missing runtime modules and declarations. A smoke script that selects a tool in a shell variable must export it before a child checker reads that selection.
