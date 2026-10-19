# 0656: Replace the Go fixture consumers and tool shims in its gate

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0657, 0658, 0688

## Outcome

- Changes: `libraries/go/check.sh`, `libraries/go/fixtures/owned_consumer.go`, `libraries/go/fixtures/full_consumer.go`, `libraries/go/fixtures/shared_consumer.go`, `libraries/go/fixtures/consumer_bridge.go`, `libraries/go/fixtures/owned_installed.py`, `libraries/go/fixtures/shared_installed.py`, `libraries/go/fixtures/dump.go`, `sdlc/scripts/children`, `sdlc/scripts/coverage_installed_c_hosts.py`.
- The gate relies on `go.mod`'s go line for the minimum version instead of parsing `go version` with sed, and runs on every target in Go's `check_platforms`.
- One Go packet dumper over the public API, following the 0642 contract, replaces the four consumer files and their two Python drivers, which are deleted. `sdlc/scripts/children` and the Go consumer names in `sdlc/scripts/coverage_installed_c_hosts.py` follow the deletion.
- `fixtures/retention_test.go` stays as a Go ownership test, and `fixtures/package_owned.py` stays as the packaging helper.

### Proof

- On the M5 through `pm job run --machine m5 0656 SHA --memory 8 -- sh libraries/go/check.sh`, the gate installs the module, builds the dumper outside the repository, and matches the 0642 oracle for every shared case. Before the change the gate runs seven `owned_installed.py` modes and `shared_installed.py` instead.
- Done once by hand and not committed: changing one expected value in the oracle fails the Go comparison with a diff.
- linux-arm64 runs at the 0619 container checkpoint on the M5, and linux-x86_64 in the 0619 x86_64 container on Yellow.

## Evidence

- `libraries/go/check.sh` reads the minimum from `go.mod`, then parses `go version` with sed. It runs `fixtures/owned_installed.py` in six modes (encoding, plan, default, feed, usage, example) and then `fixtures/shared_installed.py`.
- The Go fixtures total more than 1,300 lines: `owned_consumer.go` 678, `full_consumer.go` 278, `consumer_bridge.go` 154 and `shared_consumer.go` 100, plus the Python drivers. Their churn is visible in 279a2a317, 270af10c2, b92676dd1, ebba1063f and e4da8ade7, all on 2026-10-13 to 2026-10-15.
- `sdlc/scripts/children` lists `libraries/go/fixtures/owned_installed.py` and a stale `libraries/go/fixtures/portable_batch.py`. `sdlc/scripts/coverage_installed_c_hosts.py` names the Go consumer binaries (`plan-consumer`, `owned-consumer`, `feed-consumer`, `usage-consumer`, `shared-consumer`).
- `package_owned.py` is also called by `sdlc/scripts/release-pack`, `.github/workflows/gate.yml`, `sdlc/scripts/coverage_installed_c_hosts.py` and `site/scripts/smoke-bindings.mjs`, so it stays.
- The dumper is written once against the typed calls from 0658. 0657 widens Go's `check_platforms` to darwin-arm64 and linux-arm64.
