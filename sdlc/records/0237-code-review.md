# 0237 R code review

Status: **ACCEPT** at `96749c136b2975203ed4019cff540c35c83e2744` after a fresh read-only Sol High review and the same reviewer's correction check. High effort covered native ownership, R interruption, completion lifetime and observation provenance.

The first review found that success-only host remapping left compacted indexes in errors and completion receipts. The correction sends original live positions through the generated wrapper and FFI before worker spawn, stores them in Account and maps observations before publishing immutable snapshots. Success, accounted error and completion reads now share the same provenance. Recognition retains its existing original-position path.

The reviewer independently checked installed extension SHA-256 `e8bc241dfeaa323cf520d900afa79fc07f2a350ddfcab2d2a9f5c4148a27cffd` and reran facts.R: 16 checks and 8 sends, including both formerly failing cases and the dynamic-label receipt. The remaining focused source-installed results, strict checks and measured growth are in the build record. The worktree was clean and no new finding remained. Full tarball and platform proof remain outside this acceptance.
