# check prints the model it sent, not the model that answered

Status: Open

Found 2026-09-25 while drawing the talk's "Bring your own backend" slide. Ian asked for this change.

## What happens

`thinkthen check --url BASE` prints a `model NAME` line. NAME is the model the check put in its requests: `--model`, then the configuration file's `model`, then `jev-latest` ([check.md](../../specification/check.md), "Command line"). A server that offers one model never needs a model name. Yet with no `--model`, the check prints `model jev-latest` against any server, and that name has nothing to do with the server.

Every reply already names the model that answered. The decoder refuses a reply with no `model` ([backends.md](../../specification/backends.md)). The check reads that name and then drops it. The spec says the check does not compare the two names, because an alias such as `jev-latest` normally answers as a version.

## Why it matters

A person who brings their own backend wants to see which model answered. Today they see the default alias, or the name they typed. A second question follows: why must they name a model at all when the server knows its own?

## Options

1. Print the model each reply names, beside or in place of the model sent. For example, `model sent jev-latest, answered your-model`.
2. Also make the model name optional in the request body for any backend, so a single-model server needs no `--model`. This changes the wire rule and needs an ADR.
3. Keep today's line and document it in the check's help as "the model this check asked for".

Ian's stated preference is that the model name comes from the server.
