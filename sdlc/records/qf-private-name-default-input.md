# Quick Fix: discover the configured private-name list

Status: fresh independent Medium review accepted corrected candidate `c11bee17ed1b71397f772b01096e3100e196b04c`; the coordinator integrated the unchanged lint behavior. The coordinator claimed this lint-only correction on main `e6b39f4e`. The prior recurrence cleanup passed independent review and the current tracked tree is clean.

The external 30-entry list already existed at `~/.config/thinkthen/private-names.txt`, but an unset `THINKTHEN_PRIVATE_NAMES` made lint skip it. The lint rung now uses that existing file when the explicit override is empty or absent. A nonempty explicit override still wins. If neither is configured, the existing skip remains. The real-path check, external-only requirement, fixed case-insensitive path/content checks, refusal behavior and safe file/line reporting are unchanged. No private value enters the repository.

The exact standalone guard passed with the override absent and checked all 30 configured entries. An explicit alternate list containing a synthetic match failed with the expected content refusal; output was withheld. An explicit one-entry list with no match passed, proving that it still overrides the configured default. The full shell file passes `sh -n`, and the diff whitespace check passes. No full lint, runtime build, provider, or new permanent test ran for this input-discovery change.

## What the build taught us

A missing environment export was mistaken for a missing prerequisite. Discovering the already configured local input prevents that shell difference from silently disabling the privacy check. Explicit configuration and external storage still govern the guard; the checker does not acquire or embed its own name list.

Fresh review found an unset-HOME edge: the first fallback expanded HOME under `set -u`. The corrected condition checks that HOME is nonempty before composing the path. The standalone guard now preserves the existing skip when both HOME and the override are absent. The configured 30-entry default and explicit override cases still pass.

The reviewer passed seven standalone boundary cases: configured default, missing default, unset HOME, empty HOME, explicit clean override, synthetic path refusal and in-repository list refusal. Shell syntax and diff checks passed. The original explicit-list contract remains intact.
