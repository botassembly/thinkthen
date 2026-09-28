# 0229 unused-entry report code review

Status: **ACCEPT** at `07f277a4dbab52bebfda9c74d21c99d51cf33e29` from a fresh read-only Sol Medium reviewer. No actionable defect remained.

The reviewer traced complete manifest validation before opening the folder, the existing shared directory gate, validation of every entry before output, lexical reporting and the absence of create or remove calls. Two compiled outside-in cases passed. They prove a real keyless replay with an independently pinned digest, exact unused output, duplicate, extra, empty and invalid manifest behavior, bad-entry refusal and unchanged names, bytes, types, modes and modification times.

An independent probe against the retained binary, SHA-256 `c14b04f78289ad2f94f911a3220eaeae8d943b6a39315a2400d9b22797d6bb96`, also passed ignored names and stable locks, invalid manifest before an absent directory, symlink refusal and unchanged snapshots. Offline policy and diff checks passed. The reviewer checked the 313-line growth, reuse of the scanner and gate, and the demo's explicit one-message limit.

Register 107's reporting criterion is met by the command and executable complete-run example. Arbitrary suite-manifest production, automatic deletion and external bench maintenance remain optional integrations, not claims made by this command. Partial saved output never proves suite-wide completeness. The integrated root total is 95,161 after retaining the separately reviewed Rust descriptions. The product source merges unchanged; focused existing evidence is retained and integration checks the counters, policy and ticket records.
