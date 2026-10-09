# AGENTS.md tells outside agents how to report

Status: closed.
Resolution: 0467
Milestone: 0.2

Ian wants `AGENTS.md` to tell an outside agent how to report a problem: open a GitHub issue with a label, use the discussion board for questions, and link a fork in the issue in place of opening a pull request.

On main `81062bd31`, `AGENTS.md` does not say this. It addresses the agents that build this repository. `CONTRIBUTING.md` says to use the bug template and gives a "Before you open a pull request" section, and `SECURITY.md` covers private reports.

The change adds a short section for outside agents to `AGENTS.md`, under the workspace's 5,000-character cap that `sdlc/scripts/lint` checks on `CLAUDE.md`. It names the labels the repository uses and links the discussion board. `CONTRIBUTING.md` changes its pull request section to match. Ian decides whether outside pull requests are refused or only discouraged.
