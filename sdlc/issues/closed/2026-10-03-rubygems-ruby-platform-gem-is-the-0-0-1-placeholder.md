# The `ruby` platform gem on RubyGems is still the 0.0.1 placeholder

Status: closed.
Resolution: 0425
Milestone: 0.2

RubyGems holds `thinkthen` 0.1.1 as four platform gems: `x86_64-linux`, `aarch64-linux`, `x86_64-darwin-24` and `arm64-darwin-24`. The plain `ruby` platform gem is still 0.0.1, the placeholder that reserved the name before the release. The release workflow publishes no `ruby` platform gem.

On a platform with no matching gem, such as Windows, `gem install thinkthen` falls back to the newest `ruby` platform gem and installs the 0.0.1 placeholder. A user then gets a gem that does nothing, with no message saying why.

The 0.1.1 public install checks reached the placeholder two more ways. Ruby 3.3 on Linux installed it, because the platform gems need Ruby 3.4. Ruby 3.4.6 on macOS 26 installed it too, because the 0.1.1 macOS gems matched only darwin 24. Ticket 0394 fixes the macOS case. The Ruby floor stays, and the install page now names Ruby 3.4.

Choices:

1. Yank the 0.0.1 placeholder. `gem install` on another platform then fails with RubyGems' own "could not find a valid gem" message. Ian holds the RubyGems login, so the yank is his step. A yank cannot be undone by republishing the same version.
2. Publish a `ruby` platform gem with each release that fails at install with one sentence naming the four supported platforms.
3. Publish a `ruby` platform gem that builds the native extension from source with Rust. That costs a Rust toolchain on the user's machine, as decision 7 of ticket 0128 rejected for PyPI.

The proposed default is choice 1, the smallest step, followed by choice 2 if users report confusion. Windows gems arrive with the Windows stage 1 tickets, which shrinks the fallback case.

## Reconciliation, 2026-10-08

Commit `e2ec4deb6` builds the matching-version unsupported-platform diagnostic gem, and the `60f0dcb9a` installed campaign qualifies candidate Ruby behavior. The plain fallback no longer silently installs an inert candidate. Old public 0.0.1 placeholders remain an external registry obligation until Ian authorizes remediation/publication; no yank or public release ran. [0432 qualification](../../records/0432-shared-parity-cases.md#final-installed-qualification-2026-10-08) does not claim current registry remediation. The old proposed default and four-platform list below are historical alternatives, superseded for candidate assembly by the diagnostic gem.
