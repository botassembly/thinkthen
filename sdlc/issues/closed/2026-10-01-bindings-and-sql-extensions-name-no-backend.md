# The bindings and SQL extensions cannot name a backend

Status: closed.
Resolution: 0f263872a
Kind: idea
When: 0.2 work starts on main
Milestone: 0.2

A program or SQL session could name a backend in its own settings, as the command does with `--backend`.

ADR 0114's build order step 2 gives Python, TypeScript, Ruby, R, C and the three SQL extensions a `backend` engine or session setting over the Rust builder. Ticket 0334 built step 1, the command, the configuration file and the Rust builder, and deferred step 2. Until step 2 lands, these surfaces have no `backend` setting of their own. The Ollama debt issue notes that they reach ADR 0115's `ollama` description form only through this step.
