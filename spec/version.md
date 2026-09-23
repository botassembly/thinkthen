# Identity

`thinkthen` answers for itself before it answers anything else. The version it reports is the version its package declares, so a build in hand can always be matched to the source that produced it.

```bash
package_version="$(sed -n 's/^version = "\([^"]*\)"$/\1/p' ../crates/thinkthen/Cargo.toml | head -1)"
test -n "$package_version"
thinkthen --version | mustmatch like "thinkthen $package_version"
thinkthen -V | mustmatch like "thinkthen $package_version"
```

Both spellings print one line to standard output and exit zero. Nothing reaches standard error.

```bash
thinkthen --version 2>/dev/null | wc -l | mustmatch like "1"
thinkthen --version 2>&1 >/dev/null | mustmatch like ""
```

The help names the tool and the two flags it carries today.

```bash
thinkthen --help | head -1 | mustmatch like "Semantic commands for the shell: if, grep, and sort that understand meaning"
thinkthen --help | grep -c -- '--version' | mustmatch like "1"
thinkthen --help | grep -c -- '--help' | mustmatch like "1"
```

With no arguments the tool asks for one. It prints its help on standard error and exits two, so a script that forgets an argument stops instead of reading an empty answer.

```bash
thinkthen >/dev/null 2>&1 && exit 1
thinkthen 2>/dev/null | mustmatch like ""
thinkthen 2>&1 >/dev/null | head -1 | mustmatch like "Semantic commands for the shell: if, grep, and sort that understand meaning"
```
