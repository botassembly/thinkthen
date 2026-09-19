# ADR 0001: Start private as thinkthen in the botassembly organization

- Status: Accepted
- Date: 2026-09-18

Ian ruled the name and the organization. The visibility is an agent's decision, and Ian can overturn it cheaply.

## Context

Ian asked for a repository in the botassembly organization. He first dictated the name as "think", read as "think and decide" or "think and resolve". The crate name `think` is taken on crates.io. An agent read his folder name and started the repository as `thinkn`. Ian then ruled the name `thinkthen` the same day. `thinkthen` is free on crates.io and in the organization.

The design captures used `decide` and then `sem`. Botassembly has an open proposal for a lowercase program role named `decide`, and a tool with the same name would confuse the two.

Nothing is built. The design has ten questions open for Ian.

## Decision

**The repository, the binary, and the published crate are named `thinkthen`.** Command families sit under the name, and a command line reads as a sentence: `thinkthen decide if`.

**The repository starts private.** It goes public when Ian rules on question 1 of the design study. A visibility change on GitHub is one command.

**The repository is written as if public from the first commit.** It names no private project and no customer.

## Consequences

The name is long to type. A user who minds can alias it. The first commits and the first GitHub address carry the earlier spelling `thinkn`, and GitHub redirects the old address.
