# Contributing

Thank you for helping. Read `README.md`, then `specification/README.md`. The specification is the contract, and code follows it.

## Build the command from a checkout

Build the command and put it on your `PATH`:

```sh
cargo build --locked --release -p thinkthen --bin thinkthen
mkdir -p "$HOME/.local/bin"
install -m 755 target/release/thinkthen "$HOME/.local/bin/thinkthen"
export PATH="$HOME/.local/bin:$PATH"
```

## External contributions

Open a [GitHub issue](https://github.com/botassembly/thinkthen/issues/new/choose) for a bug, documentation correction or proposed change. Choose an existing [repository label](https://github.com/botassembly/thinkthen/labels), such as `bug`, `documentation` or `enhancement`. The bug template supplies the `bug` label and asks for the command, output and version. If GitHub does not let you apply a label, name the label in the issue so a maintainer can apply it.

Ask questions on the [discussion board](https://github.com/botassembly/thinkthen/discussions) when it is enabled. Discussions is currently disabled; until it opens, use an issue and name the `question` label.

Link your fork and the relevant branch or commit in the issue instead of opening a pull request. Include the checks you ran and their results. Keep keys, private data and security reports out of public issues; report security problems as [SECURITY.md](SECURITY.md) directs.

Before you report a proposed code change, run the gate ladder, cheapest rung first:

```sh
sdlc/scripts/install
sdlc/scripts/lint
sdlc/scripts/test
sdlc/scripts/spec
sdlc/scripts/surfaces
```

No rung touches the network. Tests replay recorded responses, so you need no key. Maintainers can run the first four rungs in GitHub Actions by starting `.github/workflows/gate.yml` by hand.

## The four names

ADR 0015 fixes four names, and other pages link this table.

| Name | What it is | What runs it |
| --- | --- | --- |
| question file | JSON that holds what to ask, with its options, levels, and cuts | `thinkthen` |
| transform | one `.jq` file that reads saved rows | `jq` |
| how-to | a folder under `demos/` that holds a worked example you can run again | the `spec` gate |
| pipeline | a Bash script that runs your own job over your own input | Bash |

A transform is a metric or a policy. A metric reads a whole run and prints numbers. A policy reads one row and names an action. A question file holds one question. A question set holds several named questions, and `annotate` reads one.

## Calls, requests and decisions

| Term | Definition |
| --- | --- |
| <a id="call"></a> **call** | One use of a ThinkThen function on any surface. |
| <a id="request"></a> **request** | One send to a model. |
| <a id="decision"></a> **decision** | One question answered about one piece of evidence. |

One call can answer several decisions. A request can carry several questions. A retry sends another request. A cache hit or replay can answer a decision without sending a request.

One `choose` call over two records can answer two decisions in one packed request. Replaying the saved answers sends zero requests. If the original request is retried once before succeeding, two requests were sent.

Tag expansion, annotations, recognition stages, relation questions, packing, partial replies, and failures have their own accounting; one record does not always mean one backend question.

`meta.requests` keeps its existing name for compatibility. It lists the stored-answer question keys behind a result. It does not count sends. `meta.requests_sent` counts the result's attributed transport attempts, including retries. Run or call facts give the corresponding total.

## How work is recorded

`sdlc/` is the record. A problem goes in `sdlc/issues/`. Authorized work is a ticket in `sdlc/tickets/`. An architecture decision is an ADR in `sdlc/planning/`. A change that alters behavior updates the specification and its pages in the same commit.

- `sdlc/planning/design-study.md` says what the tool is, what version one holds, and how it fits with botassembly.
- `sdlc/planning/rust-standards.md` says how the code is judged. Every rule names the tool that enforces it.
- `sdlc/planning/plan.md` holds the build order and its state.
- `sdlc/planning/adr/` holds the decisions.

## Reporting a bug

Use the bug template. Give the command, the output, and `thinkthen --version`. Report a security problem privately, as `SECURITY.md` says.
