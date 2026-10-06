# 0437: Honor Cargo’s native build folder

Native installation asks Cargo metadata for the effective target folder from the same calling directory as the build. It copies the resulting library instead of assuming libraries/c/target, and refuses a missing artifact before creating the installation directory. Existing header, soname and pkg-config layout remain. The linked install issue is closed.

Twenty table cases cover default, absolute, relative, spaced and caller-configured paths under sh and bash, with copied bytes and a stale default artifact that cannot mask a miss. Existing checksum/archive/link checks, shell syntax and whitespace passed. One fresh read-only review accepted. Full tests and lint run on the landing commit before push. jq was already required by the gates; fixtures stub compilation while real offline Cargo metadata resolves paths. No paid call or extra proof tool was used.
