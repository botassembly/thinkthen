# 0517: Native package design evidence

The lasting choices live once in [the native package design](../decisions/2026-10-09-native-package-design.md). [The binding author guide](../../libraries/BINDING-AUTHOR.md) references it and assigns the shared inventory to 0501 and final distribution assembly to 0530. No shipping manifest, loader, ticket, build tool or release workflow changes in this design slice.

The design reads the package-only outcome and Windows Node/C#/JVM outcomes from the PM ticket rewrite at `9c582fe8d`, plus the existing release matrix, package scripts, actual language manifests and current loader constraints. It distinguishes intended distribution targets from existing installed proof. Official Microsoft, Oracle, Apple, Dart, Flutter and GnuCOBOL sources support the package conventions and released API choices; the decision links those sources beside their claims.

The PM approved the pub exception in [the build-time asset ruling](../decisions/2026-10-09-pub-build-time-native-asset.md). The actual COBOL development-compiler requirement remains visible until its constrained migration proves released-compiler compatibility or receives an explicit ruling. Mac dependency closure and Apple integration, Windows dependencies and host execution, and final platform qualifications belong to the owning migrations and authorized candidate. This design runs no product build and supplies no platform qualification evidence. The reviewed design and approved ruling complete 0517's design outcome; package implementations belong to the named migration tickets.

Focused checks passed: `git diff --check`; existence checks for every relative link in the changed package guide and design; repository `policy.py` with Cargo offline. Policy emitted existing source-size warnings; this documentation change modifies none of those source files. A fresh read-only reviewer accepted corrected design f129cc5c. Integration retains main's rewritten migration ownership paragraphs and adds only the design reference to the guide.

## What the build taught us

Package-manager rules can require a build-time asset route rather than embedding binaries in the published archive. Record that exception before implementation and keep intended platform coverage separate from completed native checks. Preserve current ownership when a reviewed branch predates a ticket rewrite.

Fresh review found stale ownership references after the PM ticket rewrite at `aa23c7922`. The guide and design now separate 0501 inventory from 0530 assembly, assign the COBOL compiler compatibility work to 0529 with 0505 complete C views as its dependency, and assign Rust Polars to 0527. The design names the separate host migrations in 0522–0529. The approved package architecture and release hold remain unchanged. Focused whitespace and local relative-link checks passed.
