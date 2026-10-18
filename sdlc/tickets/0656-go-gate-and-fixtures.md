# 0656: Replace the Go fixture consumers and tool shims in its gate

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0657, 0688

## Outcome

- The gate takes go from the 0615 manifest, relies on `go.mod`'s go line for the version, and runs on every supported target, darwin-arm64 and linux-arm64 included.
- One Go packet dumper over the public API, following the 0642 contract, replaces the four consumer files and their Python drivers, which are deleted.
- retention_test.go stays as a Go ownership test.

### Proof

Through pm job: natively on the M5 (darwin-arm64), in the pinned arm64 container on the M5 (linux-arm64), and in the pinned x86_64 container on Yellow (linux-x86_64). On each, the gate installs the module, builds the dumper outside the repository, and matches the 0642 oracle for every shared case.

## Evidence

- libraries/go/check.sh exits 77 off Linux x86-64 (:12-14) and requires absolute THINKTHEN_GO_BIN and THINKTHEN_PYTHON_BIN paths (:15-20).
- It parses the Go version with sed (:24-29).
- It runs seven separate fixture modes through owned_installed.py (:71-77).
- The Go fixtures total more than 1,300 lines: owned_consumer.go 678, full_consumer.go 278, consumer_bridge.go 154, shared_consumer.go 100, plus Python drivers. Their churn is visible in 279a2a317, 270af10c2, b92676dd1, ebba1063f and e4da8ade7, all on 2026-10-13 to 2026-10-15.
