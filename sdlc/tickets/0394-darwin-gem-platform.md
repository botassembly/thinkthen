# 0394: macOS gems install on every macOS version

Status: in progress. Branch `ticket/0394-darwin-gem-platform`. Reserved 2026-10-03 from the 0.1.1 public install checks. A fresh ticket review returned four findings, all answered, then ACCEPT. Needs a 0.1.2 release to reach users.

Milestone: 0.1

## Outcome

1. The two macOS gems name no macOS version. A release ships `thinkthen-VERSION-arm64-darwin.gem` and `thinkthen-VERSION-x86_64-darwin.gem`. The 0.1.1 gems were `arm64-darwin-24` and `x86_64-darwin-24`.
2. `gem install thinkthen` on Ruby 3.4 picks the matching macOS gem on macOS 15 and on macOS 26. A Ruby on macOS 26 reports darwin 25. RubyGems offers the gem on every macOS version. The extension still needs macOS 15.0 to load.
3. The Linux gems keep their names: `x86_64-linux` and `aarch64-linux`.
4. The Ruby gem check fails when a macOS gem names a macOS version. The installed-file mode runs the same check on the release's own gem, so every rehearsal proves the files that ship.

## Evidence

- Starts from: the 0.1.1 public install checks, measured on 2026-10-03 on the M5 and in Ruby 3.4 containers. Quick Fix `qf-public-install-checks` writes their results into `sdlc/records/0128-release-0-1.md`, "Public install checks".
  - RubyGems holds `thinkthen` 0.1.1 as `x86_64-linux`, `aarch64-linux`, `x86_64-darwin-24` and `arm64-darwin-24`, each requiring Ruby `>= 3.4, < 4`. The plain `ruby` gem is the 0.0.1 placeholder.
  - On the M5, macOS 26.4 on Apple Silicon, with Ruby 3.4.6 and RubyGems 3.6.9, `gem install thinkthen` installed 0.0.1 `ruby`. `require "thinkthen"` then left `ThinkThen::Engine` undefined. `gem install thinkthen --platform arm64-darwin-24` installed 0.1.1, and the first-run sample replayed `true`. The darwin 24 build loads and runs on macOS 26.
  - `--platform arm64-darwin-25`, the platform a Ruby built on macOS 26 reports, also fell back to 0.0.1.
  - RubyGems 3.6.9 matches a macOS version exactly. `Gem::Platform.new("arm64-darwin-24") === Gem::Platform.new("arm64-darwin-25")` is false. A platform with no version matches every version: `Gem::Platform.new(["arm64", "darwin"])` prints `arm64-darwin`, and it matches `arm64-darwin-23`, `-24` and `-25` in both directions. It does not match `x86_64-darwin-25`.
  - `libraries/ruby/thinkthen.gemspec` sets `spec.platform = Gem::Platform::CURRENT`. The release builds the macOS gems on the `macos-15` and `macos-15-intel` runners. Their Ruby reports darwin 24.
  - The macOS floor is 15.0. `libraries/ruby/toolchain.env` sets `RUBY_MACOS_DEPLOYMENT_TARGET=15.0`, `release.yml` sets `MACOSX_DEPLOYMENT_TARGET: '15.0'`, and `check.sh` refuses any other target. Record `0128-phase-3a-build.md` measured a Mach-O minimum of 15.0.
- Keeps: the gem's files, its Ruby floor `>= 3.4, < 4`, its macOS floor 15.0, the Linux platform names, the four-gem count in `release.yml` and `release-smoke`, and the `rubygems` publish job. The build still runs on the same runners.
- Changes: the gemspec, the gem check, the changelogs, the placeholder issue and the milestones.
  - `libraries/ruby/thinkthen.gemspec`: on macOS, the platform keeps the CPU and the OS and drops the version. Elsewhere it stays `Gem::Platform::CURRENT`.
  - `libraries/ruby/check.sh`: the gem check and the installed-file mode both fail when a macOS gem's platform has a version. Each also checks that the gem's platform matches darwin 23, 24 and 25 for its CPU.
  - `CHANGELOG.md` gains a 0.1.2 heading with one line for this fix. `libraries/dart/CHANGELOG.md` gains a 0.1.2 heading that says the Dart package is unchanged.
  - `sdlc/issues/2026-10-03-rubygems-ruby-platform-gem-is-the-0-0-1-placeholder.md` gains the two other ways the install checks reached the placeholder: Ruby 3.3 on Linux, and macOS that is not darwin 24.
  - `sdlc/planning/milestones.md` lists 0394 as an open 0.1 item for 0.1.2. It marks the public install checks done.
- Proof: offline checks on Linux and a planted platform, then the 0.1.2 rehearsal.
  - On the Beelink, `sh libraries/ruby/check.sh` passes with the pinned Ruby 3.4.11. The Linux gem still reads `x86_64-linux`.
  - The gemspec, loaded by Ruby 3.4 with `Gem::Platform.local` stubbed to `arm64-darwin-24` and to `x86_64-darwin-24`, sets `arm64-darwin` and `x86_64-darwin`. Stubbed to `x86_64-linux`, it sets `x86_64-linux`.
  - The new check's Ruby lines, given a planted spec whose platform is `arm64-darwin-24`, fail with their message. Given `arm64-darwin`, they pass.
  - `sdlc/scripts/tickets`, `sdlc/scripts/lint` with the private-names list, and `git diff --check`.
  - The 0.1.2 rehearsal from `release/0.1`: both macOS smoke jobs print `check ruby: pass, installed`, and the draft lists `thinkthen-0.1.2-arm64-darwin.gem` and `thinkthen-0.1.2-x86_64-darwin.gem`. The coordinator records the run.
  - After 0.1.2 publishes, `gem install thinkthen` on the M5 installs 0.1.2 `arm64-darwin` and replays the first-run sample.
- Defers: older macOS, the placeholder, and the release itself.
  - macOS 14 and older. Today they install the 0.0.1 placeholder, which does nothing. After this ticket they install the real gem, and its extension fails to load with the loader's message. A load-time sentence that names macOS 15 is separate work. This ticket proves macOS 15 on the runners and macOS 26 on the M5.
  - The 0.0.1 `ruby` placeholder. The open issue holds the choices. Yanking it is Ian's step on RubyGems.
  - The 0.1.2 version bump, the rehearsal, the tag and the dispatch belong to the coordinator and Ian under `release-process.md` section 5.

## What Ian can overturn

- One macOS gem per CPU for every macOS version. The other choice is one gem per macOS version.
- The loader's own message on macOS 14 and older.

## What the build taught us

- `Gem::Platform::CURRENT` is the string `"current"`, not a platform. The first draft called `.os` on it, which would have broken every gem build. The gemspec now reads `Gem::Platform.local`. The stubbed-host proof caught it: stubbing the constant made `Specification#platform=` resolve the host again, so the proof stubs `Gem::Platform.local` instead.
- Proof as run on the Beelink: `sh libraries/ruby/check.sh` passed with the pinned Ruby 3.4.11 and built `thinkthen-0.1.0-x86_64-linux.gem`. The gemspec with `Gem::Platform.local` stubbed gave `arm64-darwin-24` to `arm64-darwin`, `x86_64-darwin-24` to `x86_64-darwin`, `arm64-darwin-25` to `arm64-darwin`, and kept `x86_64-linux` and `aarch64-linux`. The new `gem_platform` check failed on the published `thinkthen-0.1.1-arm64-darwin-24.gem` and `thinkthen-0.1.1-x86_64-darwin-24.gem` with `the macOS gem arm64-darwin-24 names macOS version 24`. It passed on `thinkthen-0.1.1-x86_64-linux.gem` and on a copy of the arm64 gem re-platformed to `arm64-darwin`.
