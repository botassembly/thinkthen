# ADR 0001: Start private as thinkn in the botassembly organization

- Status: Accepted
- Date: 2026-09-18

Ian can overturn this cheaply. He asked for a repository in the botassembly organization and named the tool in dictation. The spelling and the visibility are an agent's reading.

## Context

Ian dictated the name as "think", said it reads as "think and decide" or "think and resolve", and filed his captures in a folder named `thinkn`. The crate name `think` is taken on crates.io. `thinkn` is free on crates.io and in the botassembly organization. The design captures used `decide` and then `sem`. Botassembly has an open proposal for a lowercase program role named `decide`, and a tool with the same name would confuse the two.

Nothing is built. The design has ten questions open for Ian.

## Decision

**The repository, the binary, and the published crate are named `thinkn`.** Command families sit under the name, so a command line reads `thinkn decide if`.

**The repository starts private.** It goes public when Ian rules on question 1 of the design study. A rename or a visibility change on GitHub is one command.

**The repository is written as if public from the first commit.** It names no private project and no customer.

## Consequences

A later rename touches the crate names, the binary name, and the documents. The cost stays small while the code is small.
