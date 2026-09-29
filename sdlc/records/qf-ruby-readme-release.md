# Ruby README release-channel Quick Fix

Date: 2026-09-29. Fresh Medium review accepted `51388c4c2`; the coordinator integrated the correction and closed the issue.

`libraries/ruby/README.md` ended its source-check paragraph with a permanent claim that the gem is never published. The current `libraries/ruby/thinkthen.gemspec` names the gem `thinkthen`, requires Ruby 3.4 through 3.x, and packages a host platform extension. The guarded `rubygems` job in `.github/workflows/release.yml` selects release mode, checks `RELEASE_ARMED`, expects four platform gems and pushes each to RubyGems. The site catalog records `gem install thinkthen`. These are source and release-path facts; they do not show that any version is already available in the registry.

The README now names RubyGems platform gems as the release channel and gives the exact catalog command conditionally for a compatible release. Its existing source checkout setup, offline build and `check.sh` guidance remains. No gemspec, workflow, registry, site or package artifact changed. The gemspec's comment about requiring Ian's word for publication describes an approval boundary, not a permanent ban on a release.

The focused check compared the README's gem name and command with the gemspec and catalog, and the release-channel description with the guarded job. `sdlc/scripts/pages`, `sdlc/scripts/tickets`, link and diff checks passed. No build, test, release operation or registry query ran. The correction showed that a source-only build paragraph can carry a stale permanent publication claim even after the release path lands; release availability still needs its separate release evidence.
