# ADR 0004: A backend is a URL and a pure adapter

- Status: Accepted. ADR 0007 renames `--backend` to `--profile` and drops the five backend environment variables. The rest stands.
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
