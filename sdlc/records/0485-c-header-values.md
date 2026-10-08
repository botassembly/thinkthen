# 0485: Generate C values from Rust definitions

0492 absorbed this change and landed at 2502db27f. The C header now derives its values from production Rust declarations. The existing export check compares the generated header and rejects drift in either source or header, alongside its retained layout and prototype checks.

Independent review accepted implementation 443b593f47216b3e88351beb7f5135b4245212aa. The integrated ABI check passed, including planted value and declaration changes. Full evidence and the corrected findings live in [the 0492 record](0492-generated-c-header.md).

## What the build taught us

Generating the values removes the second hand-maintained truth. This ticket needs no separate comparison table or verification framework.
