# 0735: Classify installed check drivers as test code in settings scans

Status: OPEN.

Milestone: 0.2.1

Phase: 2

Depends on: 0689

Reviews: revision 45f4ac6, accept

## Outcome

- Changes: `sdlc/scripts/settings`.
- The settings scanner recognizes the installed-library check driver as test support and does not demand a public settings row for its conformance selector.
- Fix the classification class using the existing scanner and source layout. Keep checking real product environment reads, including a variable with the same spelling when read by product code.
- Extend the existing built-in self-tests to distinguish those cases. Preserve every existing missing-row, duplicate, unknown-variable and product-code assertion.
- Do not document a test control as a product setting, ignore the variable globally, add a checker/helper, or change runtime behavior.

### Proof

- On main d81fb759ad230f1070bf5fe56af6110666827e60, the existing settings self-test reports the check driver's selector as undocumented product code; retain that before-repair receipt.
- Through an admitted M5 job, the existing `python3 sdlc/scripts/settings --self-test` passes after the repair, including a real product-read negative case, and the normal settings scan succeeds. Use the normal job and declared tool resolver.
- The configured closing gate passes before landing. Run specification replay with the result-tag lane once this scanner no longer blocks it.

## Evidence

The 0707 targeted job 1792498904879-39616 reached `sdlc/scripts/spec` and failed in settings --self-test: ten failures reported that THINKTHEN_CONFORMANCE_IDS is read by product code with no settings row. The read is libraries/rust/check.py:35, unchanged from published main d81fb759a. The scanner's existing exclusions at settings:119-137 recognize test folders and test-prefixed files but misclassify this installed check driver. The selector read came from the landed value-tree test driver; it is not a new product setting or a result-tag change. The repair belongs to scanner classification and preserves its product boundary.

## Progress

- 2026-10-20 started
- 2026-10-20 next: Fresh review accepted 45f4ac618 after exact-head M5 job 1792499956418-98486 passed all25 settings self-tests normal scan and policy, retaining before-repair selector failures from0691 job1792498503545-29335 and0707 job1792498904879-39616; narrow library-root check-driver classification preserves same-name genuine product refusals, and integrated closure plus aggregate specification replay remain.
