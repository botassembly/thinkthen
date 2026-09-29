Status: closed. Fresh Medium review accepted Quick Fix `51388c4c2`; the README now describes conditional RubyGems installation and preserves source instructions.

# The Ruby README says the gem is never published

`libraries/ruby/README.md:47` ends with "The gem is never published."

`.github/workflows/release.yml` lines 312 to 329 run a `rubygems` job that pushes four platform gems, and ticket 0128 phase 3a landed that job at `3b540d19`. Ticket 0128 updated the gemspec and removed "Not published." from TypeScript's `package.json`. No item names the Ruby README line.

A reader of the README concludes the gem cannot be installed from RubyGems. After 0.1 that is false.

## Done when

The Ruby README states the RubyGems channel the release job publishes to, with the install command from the site catalog.

## Accepted correction

The [Quick Fix build record](../../records/qf-ruby-readme-release.md) traces the current gemspec, guarded release job, catalog command and README wording. The accepted correction names the channel and conditional install command while retaining the source checkout instructions. It does not claim a version is already published. Fresh Medium review accepted the correction; the coordinator closed this issue and its original Pages row together.
