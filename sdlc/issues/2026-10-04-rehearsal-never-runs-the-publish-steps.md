# The rehearsal never runs the publish steps

Status: open for account/trusted-publisher prerequisites and hosted qualification. Rehearsal now checks supported publish inputs without publication.
Milestone: 0.2

The first real release, run 37035814818 for `v0.1.0`, was the first run of every publish step. Five of eight publish jobs failed. Maven Central, pub.dev and the tap published 0.1.0, and the rest could not take it, so the next release had to be 0.1.1. The failures:

1. crates.io and RubyGems had no trusted publisher.
2. PyPI's action was pinned to a tag object, not a commit.
3. `npm publish` read a bare relative path as a GitHub repository.
4. NuGet answered 400 because the push sent no protocol header.

The 0.1.1 run then failed once more: npm refused with "OIDC permission denied", because its trusted publisher allowed only staged publishing. Ticket 0393 owns staged publishing.

Ticket 0391 fixed items 2 to 4 and added `workflows --remote-pins`. Nothing yet checks a publish step before a release.

To evaluate:

1. Which registries take a dry run that a rehearsal can make: `cargo publish --dry-run`, `npm publish --dry-run`, `dart pub publish --dry-run`, a Maven Central upload that is validated and then dropped, TestPyPI and a test NuGet feed.
2. Whether a rehearsal can prove each trusted publisher exists without publishing, for example by asking for the OIDC token exchange alone.
3. The cost: rehearse mode must still publish nothing and need no approval.

## Reconciliation, 2026-10-08

Commit `065a4e5dd` adds rehearsal publish-input checks; 0398 adds Cargo publication dry-run, wheel validation and checked Homebrew rendering. [0398 record](../records/0398-release-safety.md) retains their evidence and failed historical candidate runs. Registry account setup, npm staged approval under 0393, hosted rehearsal and final release QA remain obligations under 0425. No offline check establishes an active external account or permission, and no release workflow is authorized by this record.
