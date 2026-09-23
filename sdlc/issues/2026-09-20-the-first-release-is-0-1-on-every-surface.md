# The first release is 0.1 on every surface

Status: Closed on 2026-09-22. The ruling is recorded.

Ian ruled on 2026-09-20: "first version release will be 0.1 across all libs/exts. until then number 0.0.1 thru 0.0.9999 as necessary."

## What the ruling fixes

- The first public release of the command, of every library, and of every database extension is 0.1.0.
- Every build before that is 0.0.N, counting up from 0.0.1. Both crates carry 0.0.1 today.
- Nothing is numbered 1.0 until Ian says so. The record's phrase "version one" means the first release, and that release is 0.1.0.

## What follows for the builder

- **One number for all surfaces.** Every surface embeds the same engine, and the number names the engine. The version-sync check from `2026-09-20-lessons-from-biomcp-for-release-install-ci-and-docs.md` holds it. The crate, the Python, npm, gem, and R package files, the C header, and the three extensions' version fields all read the same string. The release job fails on a mismatch.
- **A registry never gives a number back.** crates.io, PyPI, npm, and RubyGems all refuse a second upload of a version, even after a yank. A build that only needs testing installs from a local file, the way the experiments installed their wheel, tarball, gem, and R binary. A 0.0.N goes to a registry only when someone outside needs it.
- **Cargo treats each 0.0.N as a break from the last.** `^0.0.3` matches 0.0.3 alone. That suits builds before a release. From 0.1.0 on, 0.1.N is a compatible fix and 0.2.0 may break callers.
- **PostgreSQL owes no upgrade script between 0.0.N builds.** `ALTER EXTENSION UPDATE` needs a script from each released version to the next. The first one is owed at the release after 0.1.0.
- **The placeholder that claims a registry name stays 0.0.0.** It sits below the range and spends no real number. Ian's todo list has the steps.

## One open point, decided by default

A library or an extension may ship after the command has moved past 0.1.0. One number for all surfaces means that surface joins at the current number, and its own first release is then not 0.1.0. The other reading gives each surface its own 0.1.0 and lets the numbers drift apart. That ends the version-sync check, and a user can no longer tell from the number which engine a package holds. **The default is one number.** Ian can overturn it.
