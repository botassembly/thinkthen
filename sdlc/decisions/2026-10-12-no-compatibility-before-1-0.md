# No backward compatibility before 1.0

Date: 2026-10-12. Ian can overturn this ruling.

Ian said: "There are no users, so don't worry about anyone using any of the old code. Rip out old code. I don't want to manage or maintain backwards compatibility at all until we hit 1.0."

## Ruling

- Before 1.0, no release keeps an old name, format, method, export, file layout or behavior for compatibility. Old code is deleted when its replacement lands.
- This covers what shipped in 0.1.2 too: the Rust wrapper methods, the frozen 0.1 C exports, the raw JSON and complete-result compatibility readers in each language, old cache and store formats with their migrations, and old configuration upgrades.
- When old input meets new code, a clear refusal is enough. No migration, alias, shim or deprecation period is needed.
- Write no upgrade guides or upgrade notes before 1.0. A release note may say what changed in one line.
- 0565 removes the compatibility code that remains in 0.2. 0561 to 0564 follow this ruling.
