# 0684: Simplify the Ruby toolchain, gem layout and checks

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0683, 0688

## Outcome

- On Linux, the 0619 pinned image supplies both pinned Rubies. On macOS, the 0615 resolver supplies them. `setup-ruby.sh` and the source build of Ruby and OpenSSL are deleted.
- Precompiled platform gems keep one extension per supported Ruby minor, built by the shared packaging step.
- One diagnostic gem for the generic `ruby` platform stays. The packaging step builds it, and the `THINKTHEN_RUBY_FALLBACK` switch in the gemspec is deleted.
- The Ruby conformance runner becomes a packet dumper following the 0642 contract, and `tests/test_owned_session.rb` is deleted.

### Proof

- On the M5 through pm job, natively and in the pinned arm64 container, a clean Ruby runs `gem install` on the candidate platform gem and executes a named call on each supported Ruby minor, with no library path. Its dump matches the 0642 oracle.
- Installing the diagnostic gem on an unsupported Ruby fails with its message naming the supported versions.

## Evidence

- **Self-built Ruby.**
  - `libraries/ruby/toolchain.env` pins Ruby 3.4.11 and 4.0.7 by checksum, plus libyaml, the OpenSSL 3.6 series and a macOS 15 floor.
  - `setup-ruby.sh` builds Ruby from source.
  - About 47 commits since 2026-09-15 touch the toolchain, install prefixes or macOS (2d58a6aed, 4162db0b0, 36e8427fd, 68646ab8e, e2ec4deb6).
- **Gem layout.** 0592 settled one extension per Ruby minor version in each platform gem, and that is on main. The diagnostic gem for the generic `ruby` platform is switched by `THINKTHEN_RUBY_FALLBACK` inside the gemspec (`thinkthen.gemspec:20-28`). 0425 plans to publish that diagnostic gem in place of the RubyGems placeholder.
- **Silent skips.** `check.sh:22` exits 77, and so do `build.sh` and `setup-ruby.sh`; 0618 makes these failures.
- **Fixture consumers.** `tests/test_owned_session.rb` drives conformance.
