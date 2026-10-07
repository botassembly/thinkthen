# 0426: Complete typed C calls and owned results

Status: complete. Full integration tests, lint and executable documentation passed before the main push. Final platform qualification remains part of 0425.

The C library exposes all ten named functions through typed question, record, source, image and result handles. Consumers inspect probabilities, locations, authored question metadata, answer and call identities, provenance, usage, costs, failures and detailed observations without decoding a whole result. Existing symbols and layouts remain compatible.

Native file batches own their question, records and backing storage. The engine must remain live until the batch is freed; batch access stays on its creating thread. Independently owned result rows may outlive the batch. Native processing retains cancellation and joins its workers before releasing their backing storage.

## Checks and review

The public C consumer executed all 247 applicable shared cases and all 24 image cases with no skips. Thirteen installed sanitizer consumers, nineteen unit/layout cases, dynamic and static export checks, canonical C/Rust/CLI consumers and focused Clippy/policy checks passed. A fresh High review accepted committed source 6d8a367db with no blocking product, ownership, secrecy or ABI findings. Full tests, lint, specification and affected C checks run on the landing commit before pushing main.

A follow-up real consumer caught the find answer discriminator disagreeing with the public header. The getter now returns the declared value of five; question and result discriminators retain seven. The prior failing sanitizer consumer now checks exact probabilities, selected records, file locations and the none variant. A fresh High review accepted the correction. The shared image comparison retains exact request contents while allowing concurrent arrival order; its distinct refusal and ordering regressions pass after a fresh review.

## What the build taught us

File readers must preserve lazy admission rather than invent caller-defined batch boundaries. A native owned batch retained completed rows while refusing a later invalid item without sending that stage. Explicit JSONL must use the shared parser; ordinary text remains literal. Image admission needs the actual original bytes, order and media, and request counts establish zero-send refusals.

An earlier named-provider test used its external default address for six fake-key requests. No paid work resulted. All named-provider fixtures now use an explicit owned loopback address, checked before engine creation. Larger image capture is bounded to three bodies and leaves the ordinary capture limit unchanged.

The landing run also exposed a private replay parser rejecting the public-runtime fixture field. The private runner now admits that separate array without interpreting its grammar; its original typed replay exchanges and unknown-field refusal remain. All seven existing private conformance tests pass, and a fresh review accepted the correction.

## Remaining work

The host families adopt these typed carriers. The global parity table, installed packages and final Windows/macOS qualification remain required before 0.2 is complete.
