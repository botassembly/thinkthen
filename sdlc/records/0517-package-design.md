# 0517: Native package design evidence

The lasting choices live once in [the native package design](../decisions/2026-10-09-native-package-design.md). [The binding author guide](../../libraries/BINDING-AUTHOR.md) references it and assigns assembly to 0501. No shipping manifest, loader, ticket, build tool or release workflow changes in this design slice.

The design reads the package-only outcome and Windows Node/C#/JVM outcomes from the PM ticket rewrite at `9c582fe8d`, plus the existing release matrix, package scripts, actual language manifests and current loader constraints. It distinguishes intended distribution targets from existing installed proof. Official Microsoft, Oracle, Apple, Dart, Flutter and GnuCOBOL sources support the package conventions and released API choices; the decision links those sources beside their claims.

The pub exception needs a PM ruling before the Dart/Flutter implementation. The actual COBOL development-compiler requirement remains visible until its constrained migration proves released-compiler compatibility or receives an explicit ruling. Mac dependency closure and Apple integration, Windows dependencies and host execution, and final platform qualifications belong to the owning migrations and authorized candidate. This slice runs no product build and supplies no platform qualification evidence.

Focused checks passed: `git diff --check`; existence checks for every relative link in the changed package guide and design; repository `policy.py` with Cargo offline. Policy emitted existing source-size warnings; this documentation change modifies none of those source files. Fresh review and landing belong to the coordinator.
