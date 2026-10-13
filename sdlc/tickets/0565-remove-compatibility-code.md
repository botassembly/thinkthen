

PM amendment, 2026-10-13. Remaining work. Close 0565 now. The 0576 Rust lane removes the `compatibility: DecisionResult` field in `core/result/complete/atomic.rs` and `tests/backend/native_results/compatibility.rs` while it merges the result families. 0578 owns the 0.1.2 README text in the R and MCP READMEs. The 0564 lane adds the outside-in test that refuses an old configuration file. At the candidate, `git grep -n -w -i -E 'legacy|compat|compatibility|frozen' crates libraries databases` finds only identifiers that the 0543 record justifies in one line each; English prose is exempt.
