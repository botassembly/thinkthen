# 0424: Replay staged DuckDB answers under spent caps

Recognize, relations and relate now reach native replay/cache lookup before actual-send quota admission. Both nested ABIs retain the same native atomic request cap; live requests and retries still reserve against the shared process count. The unused string quota wrapper was removed.

One fresh High source review accepted the change. DuckDB 1.5.4 and 1.5.5 Python hosts and stock CLIs returned stored answers without keys at zero and already-spent positive caps, with zero new sends or tokens. Strict misses, live-zero refusal and one-send positive limits passed, as did core reservation, retry, cancellation and fork cases. Policy, Clippy and ratchets passed. The retained grouped ABI compiles and shares the tested runner; stock hosts exercise the portable ABI. Full tests and lint run on the landing commit. An unrelated NULL-settings verifier also fails on the pre-fix artifact.
