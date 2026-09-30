# Quick Fix: pack a remapped C library under release-pack --reuse

Found in the first checkpoint sweep on main `985f05f2d`. Every surface check passed. The release smoke then failed the C archive: `lib/libthinkthen.a` and `lib/libthinkthen.so` held the builder's home. The release smoke had not run in a sweep since Dart's check began reporting not run, so this stayed hidden.

## Cause

The C check builds its library with path remaps. Later checks, such as COBOL's, Ada's and Objective-C's, rebuild the same library into `libraries/c/target` without remaps. `release-pack --reuse` packed that last copy. The lane's library held 51 home-path strings, mostly from `ring`'s C and assembly debug lines.

## Change

Under `--reuse`, `part_c` rebuilds the C library's debug build with the three remaps `release-pack` already exports. The `release-pack` header, the scripts README, the `publish-builds` manifest line and the C check's comment say so.

## Retained behavior

`release-pack` without `--reuse` is unchanged. Every other part still packs the files its check built. The release smoke still scans each unpacked library for the builder's home and tests the C archive through the C check.

## Proof

After a plain unremapped build left 51 home-path strings in the lane's library, `release-pack --reuse x86_64-unknown-linux-gnu OUT c` took 39 seconds. Its unpacked archive held no file with the home path. The checkpoint sweep after landing runs the release smoke on the rebuilt copy.

## Deferred gap

The packed C library is a rebuild of the tested one, differing only in path remaps. Each sweep rebuilds it once at pack time, and the next C check rebuilds it again. Giving every check that builds `libraries/c` the same remaps, or its own build folder, would let `--reuse` pack the tested file.

## What the build taught us

A gate step that is skipped whenever any surface is skipped can hide a regression for a long time. The first attempt added a fourth remap and claimed to compile nothing; review showed the differing flags recompile anyway.
