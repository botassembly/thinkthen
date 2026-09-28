# 0230 code and API review

Fresh independent Sol High review examined source `360f4e7a`, integrated candidate `6ff22a8e`, then accepted corrected source `ed566d69`, integrated candidate `b5d61215`. High review was selected for the wider public Rust API, shared batch metadata and C ownership contract.

The first review found lost parent closure metadata after 413 splitting, an empty call object bypassing route eligibility, a schema/runtime disagreement on annotation context, and seven site C consumers still reading bare replies. The correction carries the original closure in a private receipt, extends the existing split assertion, validates route eligibility independently of option keys, and aligns the schema and corpus. The same reviewer accepted those changes and the recorded focused proof without repeating unrelated tests.

The review covered whole structured input under Evidence plus Serialize, dynamic labels, ordered stopping and final joined facts, split request lists and usage shares, borrowed per-engine/thread failure facts, typed C ABI and direct process usage preservation, consumer/schema changes, and the measured root/C growth with shared planner/serializer reuse. No further concrete product code or API defect remained.

The site migration is still required before public 0.1 under [the marketing-owned issue](../issues/closed/2026-09-28-site-c-examples-need-json-value-wrapper.md). Acceptance of this implementation does not close that dependency. Root integration changes records and lane/status entries only; product source remains the reviewed candidate.
