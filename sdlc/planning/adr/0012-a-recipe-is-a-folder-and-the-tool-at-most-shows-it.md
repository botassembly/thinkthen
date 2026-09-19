# ADR 0012: A recipe is a folder, and the tool at most shows it

- Status: Accepted as amended by Ian on 2026-09-19. ADR 0015 item 2 records the amendment: the thing is called a transform, the later commands are `transform list` and `transform show`, and the tool never starts `jq`. The text below keeps the word "recipe" as it was written
- Date: 2026-09-19

An agent proposal in answer to two questions from Ian. Can a recipe mix configuration (the questions, the options, the thresholds) with `jq` steps? Is a command with subcommands for managing and viewing recipes a good idea? It becomes Accepted on his word, after the recipes have been tried as files, as ADR 0010 requires.

## Context

Three families of recipe are now in sight: metrics over saved rows, a policy that turns several answers into one action with a reason, and monitors over a running pipeline. Ian's test in ADR 0010 for carrying recipes inside the tool is that several exist. The tool has two rules that bear on the answer. It never runs a command, and a dependency needs a second review.

## Options

- **A. Files in the repository.** `recipes/` holds them, and people copy what they need. No surface at all.
- **B. The tool lists and shows its built-in recipes.** `thinkthen recipe list` and `thinkthen recipe show NAME` print text. The user runs it: `jq -f <(thinkthen recipe show metrics)`. The recipes ship inside the binary, so nobody copies and pastes. The tool runs nothing and gains no dependency.
- **C. The tool runs recipes.** `thinkthen jq NAME` embeds a `jq` engine. One tool does everything. It adds a large dependency whose behaviour differs from `jq` in small ways, it puts a second language inside a small tool, and it is the `report` command again under another name.
- **D. A recipe file that the tool executes as a pipeline:** shaping steps, the questions, and the steps after. This is the small language that ADR 0007 cut from the saved-question file. The shell is already the pipeline language.
- **E. Commands that manage recipes:** add, remove, edit, install. This is a package manager. `ls`, `cp`, and git already manage files.

## Decision

1. **A recipe is a folder.** It holds the question file, the `.jq` files, one short script with the pipeline line, and a page. The mix Ian asked for exists as files side by side. The question file is the configuration: questions, options, thresholds, and `on`. The `.jq` files are the steps before and after. Each piece is something the tool or `jq` already reads, so no new format is invented.
2. **Parameters use what exists.** A `.jq` file takes its cut through `--argjson`. A threshold inside a question file is changed with one `jq` line before the run.
3. **Every recipe is a how-to.** Its page is a demo under ADR 0011, so the gate runs it.
4. **Start with A.** Move to B when the three families exist as files and people do copy them. B is two subcommands and no more.
5. **C, D, and E are declined.** C returns only if needing `jq` on the machine proves a real barrier for users.

## Consequences

The recipes slice builds folders under `recipes/`. The roadmap entry for `report` points here for its fifth outcome. Nothing is added to the command surface now.
