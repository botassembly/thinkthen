# 0394: macOS gems install on every macOS version

Status: ready. Reserved 2026-10-03 from the 0.1.1 public install checks. Needs a 0.1.2 release to reach users.

Milestone: 0.1

## Outcome

1. The two macOS gems name no macOS version. A release ships `thinkthen-VERSION-arm64-darwin.gem` and `thinkthen-VERSION-x86_64-darwin.gem`, in place of `arm64-darwin-24` and `x86_64-darwin-24`.
2. `gem install thinkthen` on Ruby 3.4 picks the matching macOS gem on macOS 26, whose Ruby reports darwin 25. It also picks it on any other macOS version.
3. The Linux gems keep their names: `x86_64-linux` and `aarch64-linux`.
4. The Ruby gem check fails when a macOS gem names a macOS version. The installed-file mode runs the same check on the release's own gem, so every rehearsal proves the files that ship.

## Evidence

- Starts from: the 0.1.1 public install checks, 2026-10-03 (record `sdlc/records/0128-release-0-1.md`, "Public install checks").
  - RubyGems holds `thinkthen` 0.1.1 as `x86_64-linux`, `aarch64-linux`, `x86_64-darwin-24` and `arm64-darwin-24`, each requiring Ruby `>= 3.4, < 4`. The plain `ruby` gem is the 0.0.1 placeholder.
  - On the M5, macOS 26.4 on Apple Silicon, with Ruby 3.4.6 and RubyGems 3.6.9, `gem install thinkthen` installed 0.0.1 `ruby`. `require "thinkthen"` then left `ThinkThen::Engine` undefined. `gem install thinkthen --platform arm64-darwin-24` installed 0.1.1, and the first-run sample replayed `true`. The darwin 24 build loads and runs on macOS 26.
  - `--platform arm64-darwin-25`, the platform a Ruby built on macOS 26 reports, also fell back to 0.0.1.
  - RubyGems 3.6.9 matches a macOS version exactly. `Gem::Platform.new("arm64-darwin-24") === Gem::Platform.new("arm64-darwin-25")` is false. A platform with no version matches every version: `Gem::Platform.new(["arm64", "darwin"])` prints `arm64-darwin`, and it matches `arm64-darwin-23`, `-24` and `-25` in both directions. It does not match `x86_64-darwin-25`.
  - `libraries/ruby/thinkthen.gemspec` sets `spec.platform = Gem::Platform::CURRENT`. The release builds the macOS gems on the `macos-15` and `macos-15-intel` runners, whose Ruby reports darwin 24.
- Keeps: the gem's files, its Ruby floor `>= 3.4, < 4`, the Linux platform names, the four-gem count in `release.yml` and `release-smoke`, and the `rubygems` publish job. The build still runs on the same runners.
- Changes: the gemspec, the gem check, the changelogs and the placeholder issue.
  - `libraries/ruby/thinkthen.gemspec`: on macOS, the platform keeps the CPU and the OS and drops the version. Elsewhere it stays `Gem::Platform::CURRENT`.
  - `libraries/ruby/check.sh`: the gem check and the installed-file mode both fail when a macOS gem's platform has a version. Each also checks that the gem's platform matches darwin 23, 24 and 25 for its CPU.
  - `CHANGELOG.md` gains a 0.1.2 heading with one line for this fix. `libraries/dart/CHANGELOG.md` gains a 0.1.2 heading that says the Dart package is unchanged.
  - `sdlc/issues/2026-10-03-rubygems-ruby-platform-gem-is-the-0-0-1-placeholder.md` gains the two other ways the install checks reached the placeholder: Ruby 3.3 on Linux, and macOS that is not darwin 24.
- Proof: offline checks on Linux and a planted platform, then the 0.1.2 rehearsal.
  - On the Beelink, `sh libraries/ruby/check.sh` passes with the pinned Ruby 3.4.11. The Linux gem still reads `x86_64-linux`.
  - The gemspec, loaded by Ruby 3.4 with `Gem::Platform.local` stubbed to `arm64-darwin-24` and to `x86_64-darwin-24`, sets `arm64-darwin` and `x86_64-darwin`. Stubbed to `x86_64-linux`, it sets `x86_64-linux`.
  - The new check's Ruby lines, given a planted spec whose platform is `arm64-darwin-24`, fail with their message. Given `arm64-darwin`, they pass.
  - `sdlc/scripts/tickets`, `sdlc/scripts/lint` with the private-names list, and `git diff --check`.
  - The 0.1.2 rehearsal from `release/0.1`: both macOS smoke jobs print `check ruby: pass, installed`, and the draft lists `thinkthen-0.1.2-arm64-darwin.gem` and `thinkthen-0.1.2-x86_64-darwin.gem`. The coordinator records the run.
  - After 0.1.2 publishes, `gem install thinkthen` on the M5 installs 0.1.2 `arm64-darwin` and replays the first-run sample.
- Defers: older macOS, the placeholder, and the release itself.
  - The macOS floor. An unversioned gem is offered on macOS 14 and older too. This ticket proves macOS 15 on the runners and macOS 26 on the M5. If the extension needs a newer macOS than the user has, it fails to load with the loader's message instead of installing the placeholder. Measuring and pinning the floor is separate work.
  - The 0.0.1 `ruby` placeholder. The open issue holds the choices. Yanking it is Ian's step on RubyGems.
  - The 0.1.2 version bump, the rehearsal, the tag and the dispatch belong to the coordinator and Ian under `release-process.md` section 5.

## What Ian can overturn

- One macOS gem per CPU for every macOS version, in place of one gem per macOS version.

## What the build taught us
