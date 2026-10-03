# The `ruby` platform gem on RubyGems is still the 0.0.1 placeholder

Status: open. Filed 2026-10-03 from the 0.1.1 release (record `sdlc/records/0128-release-0-1.md`). Owner: the queue owner.
Milestone: 0.2

RubyGems holds `thinkthen` 0.1.1 as four platform gems: `x86_64-linux`, `aarch64-linux`, `x86_64-darwin-24` and `arm64-darwin-24`. The plain `ruby` platform gem is still 0.0.1, the placeholder that reserved the name before the release. The release workflow publishes no `ruby` platform gem.

On a platform with no matching gem, such as Windows, `gem install thinkthen` falls back to the newest `ruby` platform gem and installs the 0.0.1 placeholder. A user then gets a gem that does nothing, with no message saying why.

Choices:

1. Yank the 0.0.1 placeholder. `gem install` on another platform then fails with RubyGems' own "could not find a valid gem" message. Ian holds the RubyGems login, so the yank is his step. A yank cannot be undone by republishing the same version.
2. Publish a `ruby` platform gem with each release that fails at install with one sentence naming the four supported platforms.
3. Publish a `ruby` platform gem that builds the native extension from source with Rust. That costs a Rust toolchain on the user's machine, as decision 7 of ticket 0128 rejected for PyPI.

The proposed default is choice 1, the smallest step, followed by choice 2 if users report confusion. Windows gems arrive with the Windows stage 1 tickets, which shrinks the fallback case.
