# Ticket 0015 live job

Completed on 2026-09-20 after the local implementation and review passed and Ian authorized the paid work.

```sh
sdlc/scripts/live --max-tokens 100000 probes/annotate-0015/run
```

The job pins the reviewed hosted base `https://api.typesafe.ai/v1` and filled resumable caches for how-tos 39 and 14. It also compared one mixed `decide`/`choose`/`score` group with the same questions alone and compared six labeled near-cut decisions alone and beside neighboring questions. It wrote recordings and result rows only under the two how-to folders and this probe folder. The reservation covered 29 requests.

All 29 recordings passed the schema, endpoint, request-response key, model, usage, digest-link, and credential-marker checks. They report 10,104 input tokens and 1,533 output tokens in total.

`mixed-summary.json` reports equal packed and separate values. Packing used 371 input tokens; the three separate requests used 915. `borderline-summary.json` reports six labeled cases, one answer flip, and a largest probability shift of 0.04. The packed form resolved three cases and got all three right. The separate form resolved four and got all four right. The earlier 1, 5, 10, 20, and 40 packing probe, tagging probe, model probe, and status check were not repeated.
