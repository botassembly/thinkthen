# Two release secrets remain outside trusted publishing

Status: closed.
Resolution: 0425
Milestone: 0.2

crates.io, PyPI, npm, RubyGems, NuGet and pub.dev publish through trusted publishing and keep no secret. Two jobs still read secrets from the `release` environment:

1. `maven` reads a Maven Central username, password and GPG signing key.
2. `tap` reads `TAP_DEPLOY_KEY`, a deploy key with write access to `botassembly/homebrew-thinkthen`.

Nothing records when either secret expires or was last rotated. An expired token fails only during a release.

To evaluate:

1. Whether Maven Central offers trusted publishing from GitHub Actions now.
2. Whether the tap can be updated by a GitHub App token or the release job's own token instead of a deploy key.
3. A rehearsal check that each remaining secret is set and still accepted, without publishing.
