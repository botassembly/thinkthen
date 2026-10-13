

PM amendment, 2026-10-13. Remaining work. Close 0560 now; its tooling and baseline are done and reviewed. 0543 runs `sdlc/scripts/coverage --release` once on the candidate SHA and stores only its summary. The deleted-line allowance equals 59,962 minus the candidate's mapped line count. Each collection passes when its reached count is at least its lower baseline count minus that allowance. The candidate record shows this in one table, and no new script is written for it. Delete the stale branch `ticket/0560-measure-coverage`.
