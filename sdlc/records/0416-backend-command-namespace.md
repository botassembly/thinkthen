# 0416: Put backend checks under backends

The public command is now `thinkthen backends check`. The hidden `thinkthen check` alias uses the same arguments and handler. Questions, items, answers, checks, datasets, runs, setups, findings, people and search are reserved and refuse before requests.

Compiled-command checks retain success, refusal, counted no-send, budgets, retries, secrecy, timeout and cancellation behavior. Affected documentation examples and settings inventory passed. One fresh whole-change review found a stale saved help output and a test configuration race caused by duplicate modules. The saved output is corrected and canonical/alias cases now share one sequential behavior test. No production behavior changed in those corrections.

Landing gates: full tests and lint. No proxy commands or new backend behavior were added. Unrelated recipe milestone prose checks are corrected in 0419.
