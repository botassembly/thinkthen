# Launch gaps found in marketing prep

Status: Open. Four gaps for the builders, and one standing authorization on record.

The agent that holds the marketing job ran six read-only surveys on 2026-09-20. The marketing record is `products/thinkthen/objections.md` in the marketing repository. Four findings belong to this repository. None changes the order of ticket 0015.

## 1. The second backend has no owner

Ian's plan promotes "works with another server" in the week after launch, and the docs site shows it from day one. ADR 0010 holds the adapter on the roadmap. It calls the small server that puts a local model behind the System One shape "its own project". `specification/roadmap.md` holds it with no owner. How-to 18 waits for slice 13, the last one. No ticket, issue, or experiment covers the server.

The cheapest honest proof is small. A stub or a tiny server on the loopback address answers in the wire shape, and how-to 18 points `--url` at it and compares two deciders. That proves "one address picks the backend". It does not prove "runs on a local model". A real local model behind the shape needs token probabilities from a local runtime, and nobody has tried it. I recommend a numbered experiment for that before anyone promises it. The launch copy holds the word "local" until then.

## 2. Release binaries and the installer have no ticket

Ian ruled on 2026-09-20 that the command installs through a `curl` installer that downloads a GitHub release. `sdlc/planning/libraries/command.md` names `cargo-dist` and the targets. `.github/workflows/` holds `gate.yml` alone. No workflow, script, or ticket builds a release. The install line is the third line of the home page, so this blocks the launch as surely as a missing command. The repository stays at `botassembly/thinkthen` by Ian's ruling of the same day. The install script can be served from `thinkthen.dev/install.sh`, which keeps the line short.

## 3. A first run with no key

The vendor's quickstart says a key comes from its dashboard. Ian confirmed on 2026-09-20 that there is a waitlist and that his key took about a day. He called it no big deal. A new user still waits a day, so the first thing they run has to work with no key, and the install page should tell them to ask for the key first. The pieces exist: `--replay` opens no connection and reads no key. The gap is packaging. A release archive, or the README's first example, would need to carry one small recording folder and the input it answers. This is a stumble-register item, and it is cheap once the installer exists.

## 4. One page on exit code 1 under `set -e`

A public thread will raise it. The demos already run under `set -euo pipefail`, and `if thinkthen decide ...; then` is safe. No how-to says so in one place. One short page closes it: the `if` form, the `||` form, what `--raw` prints, and the host that treats exit 2 as a block.

## A standing authorization for paid marketing measurements

Ian typed on 2026-09-20: "you can use a dollar to do whatever you need to do. Don't have to ask again." It covers paid measurements by the marketing agent up to one US dollar in all, through `sdlc/scripts/live`, under the credential rules in `CLAUDE.md`. It does not cover the builders' own probe, which asks for its own go-ahead. The first use is a round on public labeled sets under experiment 206, held under 25 US cents. The spend is reported in the findings issue for each round.

## What Ian can overturn

All of it. Gaps 1 and 2 need an owner more than a ruling.
