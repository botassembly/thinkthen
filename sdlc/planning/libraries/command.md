# The command surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to the command.

## What really good looks like

A shell user expects one file to install and a `--help` that fits a screen. The answer is a bare JSON value on standard output, the diagnostics are sentences on standard error, and the exit code carries the outcome class. A program in any language runs the binary as a child process and reads one line per record in input order. One thing earns the praise: one process stays alive, holds one secure connection, and answers each record as it arrives.

```sh
coproc TT { thinkthen decide 'The command only reads files.' --lines; }
while read -r cmd; do
  printf '%s\n' "$cmd" >&"${TT[1]}"
  read -r answer <&"${TT[0]}"
  [ "$answer" = true ] && run_it || refuse
done
```

## Goals

- The Linux build is static. `ldd` names no shared library.
- The hot path reads the arguments, the two variables, and standard input. Nothing else.
- A record-mode process prints record N before record N+1 arrives, flushes each line, and pays one handshake for the run.
- A child-process caller reads standard output alone: one line per record, input order, the exit codes of `channels.md`.
- A closed downstream pipe ends the run quietly at the code it earned.
- Completions for bash, zsh, and fish and a manual page come from the parser.

## Anti-goals

- No configuration file and no dotfile. Each costs a read at start and lets one command answer twice on two machines.
- No daemon and no `serve`. `specification/roadmap.md` declined both, and record mode serves the loop.
- No wrapper script as the install path. It adds a process start and swallows the exit code.
- No rule in the parser or the printer. A rule with a second home drifts from the libraries.
- No caller parses standard error. Every failure class earns an exit code.
- No system TLS library and no update check at start. Either one breaks the single-file install.

## Where this language wastes time

- **The process start.** A call inside a loop pays a fork, an exec, the loader, and a new handshake. Record mode over one process is the fix.
- **The new secure connection.** A handshake costs several round trips and much of a short judgment. One process holds one pool.
- **The dynamic loader.** A system TLS library is loaded at every start. `rustls` and a `musl` target remove it.
- **A buffered standard output.** Held lines make a `coproc` caller wait forever. One flush per record.
- **Reading ahead.** Reading all the input first delays the first answer. The reader stays lazy.

## How little code

No foreign-function barrier exists here. The command links the engine as an ordinary Rust dependency. The parser is `clap` with derive, and `clap_complete` and `clap_mangen` generate the completions and the manual page at release time.

The shim holds the parser, the two variables, the mapping to one engine request, the printer for the bare value and `--details`, the exit-code function, and the sentence for standard error. It never holds threshold math, request JSON, a retry, a recording name, or the question-file grammar.

`cargo-dist` builds an archive per platform and writes the installer script and the Homebrew formula, so no user needs a Rust toolchain. Prebuilt: static `musl` Linux on x86-64 and aarch64, macOS on both, Windows on x86-64.

## Tests only this surface needs

- A cold start reaches the loopback stub inside the shared budget.
- A `coproc` reads record 1's answer before record 2 is written.
- The committed completions and manual page match the parser.
- A planted dotfile in the working directory and in `HOME` is never read.

## Open questions for the ADR

1. Settled by Ian on 2026-09-20: one crate named `thinkthen`, and no `thinkthen-cli`. The install path is a `curl` installer that downloads a release from GitHub. `cargo install thinkthen` should also give the command if one crate can carry both, and the installer is the path that must work.
2. Does the long-lived record-mode process get a written protocol for callers in other languages, or do they stay on one process per batch?
