# 0249 Dart and Flutter independent review

Status: awaiting fresh read-only reviewer. This file records the candidate boundary; the coordinator fills the verdict and findings after independent review. It is not an approval.

Candidate: `ticket/0249-dart-integration` source in `libraries/dart/`, the root README row, the Dart consumer issue checkpoint, and `sdlc/records/0249-dart-{preflight,build}.md`. Review the public value/facts/error contract, FFI ownership and invalid-input cleanup, complete request-body fixtures and prior-receipt provenance, all 29 executable shared J1 cases through an installed consumer, isolate cancellation join order, Flutter 17-arrival host and one-arrival Linux app, privacy guard, executable discovery, source ratchet, and honest platform/release limits. The shared `surfaces.txt` and `policy.py` registration is assigned to the coordinator's registry lane.

First fresh review found one shared harness dependency: public-binding tests could start a preexisting `conformance-backend` because only the C-only `build()` path built it. The correction moves the backend build into `start_backend()` and fixes its target directory to the path subsequently executed. A focused absent-executable Dart 29-case proof passed with the prior artifact restored afterward. The source branch then merged PHP's landed non-Cargo pattern from main and added one Dart registry entry, a named pubspec, a host source ratchet, and private nested Flutter manifest checks. Policy and the surface registry pass with Dart-specific planted failures. The same reviewer will recheck the helper and registration together.

Verdict: correction pending independent recheck.
