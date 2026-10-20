---
date: 2026-10-11
status: design input for 0.3
source: Ian's dictation on 2026-10-11; 0.2's [`specification/mcp.md`](../../../specification/mcp.md) and ticket 0455
---

# ThinkThen MCP in 0.3

## Ian's rulings, 2026-10-11

- **One tool, not eleven.** The MCP server exposes one tool that takes a command, as if the agent were running the CLI. The tool delegates to the same grammar and handles it as the CLI does. BioMCP did this: it went from 35 tools to one, which saves context, and the agent can ask the tool for help.
- **What it invokes.** The ten functions plus `run` for the toolbox, together with help: list the functions, list the toolbox, and describe any function or tool.
- **Do not market MCP yet.** Market it once 0.3 ships.

## What 0.2 ships

0.2's `thinkthen mcp` is a local stdio server with ten tools, one per function, in a fixed order. It refuses resources, prompts, administration, runs, audit and transforms. This follows Ian's ruling of 2026-10-06 and ticket 0455, recorded in `specification/mcp.md`. 0.2 ships as built. 0.3 replaces it.

## The 0.3 shape

- **One tool, `thinkthen`,** with one string or argument-list input holding a command line, such as `decide "Is this a refund request?" --text "..."` or `run support/triage --file diff=changes.diff`.
- **Help through the tool.** `help`, `help decide`, `toolbox list` and `toolbox describe TOOL` return the same text the CLI prints. The tool description stays short and tells the agent to ask for help.
- **Same engine and same results.** The tool parses the command with the CLI's parser and returns the CLI's JSON result. No second argument schema to maintain.
- **Local or proxy.** The MCP server runs functions locally, or sends them to a ThinkThen proxy when one is configured, like any other caller.
- **Files.** A local stdio server reads the caller's paths, as the CLI does. Through a remote proxy, paths are read on the client side first, so a server never reads the agent's disk.

## To decide

1. Whether the tool takes one string or an argument list. An argument list avoids shell quoting. BioMCP's choice is the model to copy.
2. Which administrative commands, such as cache status, the tool allows.
