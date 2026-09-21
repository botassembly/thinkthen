# System One probability-total capture

The authorized command was:

```sh
sdlc/scripts/live --max-tokens 84000 probes/probability-total-0038/run
```

It made 50 independent requests with retries and caching disabled. Forty answered and ten were refused. Every refusal reported total `0.9900000000000001`, 17 members, and the then-active tolerance `3.774758283725532e-15`. The live ledger moved from 18,440,118 to 18,524,118 charged tokens, the exact 84,000-token reservation. [result.json](result.json) holds these safe derived values. No response body, member, label, evidence, or credential was retained, and no second run occurred.
