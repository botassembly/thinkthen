# Include the file reader in Ruby public names

The installed surface sweep found an obsolete expected inventory: approved 0420 adds `ThinkThen.files` and `Engine#files`, but the public-name test omitted them. Add `files` to its shared expected list. The assertion still requires exact inventories and excludes private helpers. Product code and other expected names are unchanged.

Fresh review accepted the one-line correction. The actual native-library public-name test passes: one case, two assertions, no failures or skips. Ruby syntax and diff checks pass. The remaining Ruby surface suite runs with the final integrated package checks.

## What the build taught us

Keep an approved public addition in the existing exact-name contract. Preserve its private-name exclusion instead of weakening the assertion.
