# 0455: Local MCP calls over the native engine

The local `thinkthen mcp` server exposes the ten named functions over stdio. One native engine owns parsing, question/file loading, image admission, cache, record/replay and complete results. EOF, cancellation and broken output stop and join owned work; credentials remain startup settings.

A fresh High review found four concrete admission defects. The fixes retain loaded question metadata while explicit fields override its pointer, validate finite arrays before sending, enforce the whole-set size limit before composition, and install the existing file-size signal handler at startup. A fresh narrow High review accepted those corrections. Focused evidence covers 34 Rust tests, 16 installed groups, nine client checks and 25 public cases. Full integration tests, lint and executable documentation passed before the push.

## What the build taught us

The transport must reuse native admission, including explicit-field precedence and whole-set byte limits. A local server still needs the command’s file-size and cancellation behavior; successful small calls do not establish either.

## Remaining

The complete shared parity row and bounded timing campaign remain required. Final Windows behavior is measured on the completed 0.2 build. HTTP serving, proxy policy and unmeasured speed claims remain out.
