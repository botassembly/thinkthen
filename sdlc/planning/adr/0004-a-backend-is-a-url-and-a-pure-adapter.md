# ADR 0004: A backend is a URL and a pure adapter

- Status: Accepted. ADR 0007 renames `--backend` to `--profile` and drops the five backend environment variables. ADR 0010 removes the four-value profile and replaces the key rule. The key no longer drops when the address changes. It goes only to the address the user named, or to the default base when the user names none, as the amendment below states. The adapter rules stand.
- Date: 2026-09-18

An agent decision that carries out Ian's ruling in ADR 0003. Ian can overturn it cheaply while the code is small.

## Context

Ian ruled that a backend should be a URL and a simple adapter, easy to configure, with nothing hard-coded to one vendor. He expects competing decider models, hosted and local. The first vendor's wire format is one JSON request and one JSON response. Most local model servers speak a different format: chat completions that can return token probabilities.

## Decision

**A backend profile is four values:** a URL, an adapter name, a model name, and the name of the environment variable that holds the key. Flags and environment variables can set each one. A configuration file arrives in a later ticket.

**An adapter is a pure translation.** It turns thinkthen's own plan into request bytes, and it turns response bytes into thinkthen's own answers. It lives in `thinkthen-core`, touches no network, and is tested with fixture files alone. The binary sends the bytes. Every adapter in version one posts JSON with a bearer key.

**The set of adapters is an enum compiled into the binary.** The first is `systemone`, the first vendor's format. The tool ships one built-in profile named `jev` that uses it. That profile is a row of data, and any user can define another the same way.

**The second adapter is planned as `chat-logprobs`.** It asks any server that speaks the common chat-completions format for one constrained token and reads the token probabilities. Such servers include local ones. It proves the tool is free of any one vendor, and it lets a user run with no hosted service at all.

**A subprocess adapter is the later escape hatch.** It would exchange JSON with a user's program over standard input and output. Dynamic plugin libraries are refused.

**A key never crosses hosts.** A profile's key goes only to that profile's URL. Overriding the URL on the command line drops the key unless the user also names a key variable.

**`specification/backends.md` is the published contract,** with fixture requests and responses that another implementer can test against.

## Consequences

A vendor with an unusual shape waits for the subprocess adapter or serves one of the two formats. Internal types stay neutral, so the first vendor's field names never leak into results.

## Amendment, 2026-09-26: the key goes only to the address the user named

ADR 0010 replaced the rule "A key never crosses hosts." The key comes from `THINKTHEN_API_KEY` alone, and no option names another variable. It goes only to the address the user named with `--url`, `THINKTHEN_BASE_URL`, or the configuration file's `url`. When the user names none, it goes to the default base in `specification/backends.md`. Naming the address is the user's own act, so a changed address keeps the key. `specification/backends.md` holds the rule. Its loopback limit on plain `http://` keeps the key off the network in clear text. The profile paragraph above is history too. A profile under ADR 0032 holds a name and local limits, and never an address, an adapter, or a key. Local experiment 273, report 12, found the stale status line.
