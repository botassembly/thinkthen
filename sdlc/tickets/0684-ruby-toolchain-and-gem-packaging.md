# 0684: Simplify the Ruby toolchain, gem layout and checks

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0615, 0618, 0619, 0683, 0688

## Outcome

- Changes: `libraries/ruby/setup-ruby.sh`, `libraries/ruby/toolchain.env`, `libraries/ruby/build.sh`, `libraries/ruby/check.sh`, `libraries/ruby/thinkthen.gemspec`, `libraries/ruby/fallback/`, `libraries/ruby/tests/test_package.rb`, `libraries/ruby/README-DEVELOPMENT.md`, `sdlc/planning/libraries/ruby.md`, `sdlc/scripts/release-container`, `sdlc/scripts/release-workflow`, `sdlc/scripts/release-pack`, `.github/workflows/gate.yml`, `site/README.md`, `site/scripts/smoke-bindings.mjs`.
- On Linux, the 0619 pinned image supplies both pinned Rubies. On macOS, `sdlc/scripts/tools.py` supplies them through the `ruby` and `ruby34` groups. `setup-ruby.sh` is deleted, and every caller resolves Ruby through the manifest.
- Precompiled platform gems keep one extension per supported Ruby minor, built by `build.sh` for each minor in `RUBY_MINORS`.
- One diagnostic gem for the generic `ruby` platform stays. The packaging step builds it from its own spec under `libraries/ruby/fallback/`, and the `THINKTHEN_RUBY_FALLBACK` switch in `thinkthen.gemspec` is deleted.
- The Ruby packet dumper is 0712.

### Proof

- On the M5: `pm job run --machine m5 0684 SHA --memory 8 -- sh -c 'cargo build --locked --offline -p conformance-backend && THINKTHEN_TEST_PROFILE=full sh libraries/ruby/check.sh'`. Both Ruby minors build, install the platform gem with no library path and run their installed checks. Before the change, `git grep setup-ruby` finds callers in the release scripts, `gate.yml` and the site runner.
- `ruby tests/test_package.rb` in that run proves that installing the diagnostic gem on an unsupported Ruby fails with a message naming Ruby 3.4 and 4.0.
- The same full check in the pinned arm64 container runs at the 0619 container checkpoint, and the x86_64 container on Yellow at the 0691 checkpoint after this lands.

## Evidence

- **Wrapper over the manifest.** `libraries/ruby/setup-ruby.sh` now only calls `sdlc/scripts/tools.py` for the `ruby34` and `ruby` groups (0615). On macOS, `tools.py` installs each Ruby through its provider and `ruby_source` for libyaml. `toolchain.env` reads `sdlc/tools.json` for the versions (Ruby 3.4.11 and 4.0.7, OpenSSL 3.0.22, rbs) and fixes the macOS 15.0 deployment target.
- **Callers of the wrapper.** `sdlc/scripts/release-container`, `sdlc/scripts/release-workflow`, `.github/workflows/gate.yml`, `site/scripts/smoke-bindings.mjs`, `site/README.md`, `libraries/ruby/README-DEVELOPMENT.md` and `sdlc/planning/libraries/ruby.md` still name `setup-ruby.sh`. 0620 rewrites `.github/workflows/release.yml`, so this ticket leaves that file to it.
- **Gem layout.** 0592's one extension per Ruby minor is on main: `build.sh` builds `lib/thinkthen/3.4/` and `lib/thinkthen/4.0/`, and `lib/thinkthen.rb` loads by `RUBY_VERSION`. `build.sh` builds the diagnostic gem by running `gem build` again with `THINKTHEN_RUBY_FALLBACK=1`, and `thinkthen.gemspec` switches on it. `part_ruby` in `sdlc/scripts/release-pack` keeps the generic gem on the x86_64 Linux target. 0425 plans to publish that diagnostic gem in place of the RubyGems placeholder.
- **Failures, not skips.** 0618 landed; `check.sh` fails through `not_run` with exit 1, so no exit-77 path remains to remove.
- Package release numbers, tool pins and the Ruby 3.4 and 4.0 minimums stay under the no-version-numbers ruling.
