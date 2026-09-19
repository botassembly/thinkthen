# The `decide` family

Status: **Settled** for `if`. The other verbs are being drafted from Ian's captures and land here before their tickets.

`decide` judges meaning. Evidence arrives on standard input. The question arrives as arguments. Every verb obeys [channels.md](channels.md), prints the result in [result.md](result.md), and reaches a backend as [backends.md](backends.md) describes.

## `thinkthen decide if CONDITION`

Asks whether a condition holds for the evidence.

```sh
thinkthen decide if 'the customer explicitly requests a refund' < message.txt
thinkthen decide if 'the customer explicitly requests a refund' --min-prob 0.9 --status < message.txt
thinkthen decide if 'the customer explicitly requests a refund' --plan < message.txt
```

- `CONDITION` is one argument. It states a fact that is true or false of the evidence. The decider model reads it as the question.
- Standard input is read to its end as UTF-8 text and becomes the evidence. Empty input is a usage error, because a judgment about nothing is a mistake in the pipeline.
- `--min-prob P` sets the pass mark. Without it the result is `unassessed`.
- `--status` sets the exit code from the assessment and needs `--min-prob`.
- `--plan` prints the request and stops.
- The backend options are `--backend`, `--url`, `--adapter`, `--model`, `--key-env`, `--timeout`, and `--max-retries`.

Writing a good condition matters more than any option. A condition works when it names one fact that is visible in the evidence. "Mentions a delivery date" works. "Is a good reply" does not.
