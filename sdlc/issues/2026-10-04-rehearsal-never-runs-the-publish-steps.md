# The rehearsal never runs the publish steps

Status: open. Filed 2026-10-04 by the queue owner from the 0.1 releases. Owner: the queue owner.
Milestone: 0.2

The first real release, run 37035814818 for `v0.1.0`, was the first run of every publish step. Five of eight failed, so 0.1.0 never shipped and the next release had to be 0.1.1:

1. crates.io and RubyGems had no trusted publisher.
2. PyPI's action was pinned to a tag object, not a commit.
3. `npm publish` read a bare relative path as a GitHub repository.
4. NuGet answered 400 because the push sent no protocol header.
5. On the 0.1.1 run, npm refused with "OIDC permission denied" because its trusted publisher allowed only staged publishing.

Ticket 0391 fixed items 2 to 4 and added `workflows --remote-pins`. Nothing yet checks a publish step before a release.

To evaluate:

1. Which registries take a dry run that a rehearsal can make: `cargo publish --dry-run`, `npm publish --dry-run`, `dart pub publish --dry-run`, a Maven Central upload that is validated and then dropped, TestPyPI and a test NuGet feed.
2. Whether a rehearsal can prove each trusted publisher exists without publishing, for example by asking for the OIDC token exchange alone.
3. The cost: rehearse mode must still publish nothing and need no approval.
