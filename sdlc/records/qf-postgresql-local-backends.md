# PostgreSQL checks all named backends

Quick Fix `qf-postgresql-local-backends`, 2026-10-05. Status: source reviewed and PostgreSQL check passed; integrated landing gates belong to the coordinator.

The final installed PostgreSQL check on `9c522d072` passed 93 steps and failed `named_backend_settings`. The shared backend table contained seven rows after llama.cpp and MLX landed, while this fixture supplied five fake keys and markers and assumed Perplexity's earlier position and five total sends.

The fixture now supplies fake llama.cpp and MLX keys and markers. Path counts accumulate each shared row's declared path. Replay and later send totals use the table's row count. Exact permission, secrecy, bearer, replay and send assertions remain. Production behavior and dependencies stay unchanged.

Fresh independent Sol Medium source review returned ACCEPT. Bash syntax and Python compilation passed. The existing full PostgreSQL check ran through the lane 2 clean-environment launcher with the existing scratch, usage guard, loopback backend, memory limits and shared cache locks. It exited 0 with 94 passed and 0 failed, including `named_backend_settings`; conformance reported 30 passed, 0 failed and 2 not run. The log is `/tmp/thinkthen-qf-postgresql-local-backends/postgresql.log`. No paid calls ran. Broader release qualification remains with the coordinator.
