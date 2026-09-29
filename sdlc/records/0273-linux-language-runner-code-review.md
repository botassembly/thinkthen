# 0273 Linux runner tool code review

Fresh High review accepted corrected candidate `9c26a6896e26687d6a809bce180d3e3779e6d346`. The coordinator merged that exact helper, fixture and workflow code. The code-path diff from the reviewed candidate is empty. This completes the static implementation; it does not qualify an actual runner or close the language distribution issues.

The first review rejected two provenance gaps. An apt simulation containing any `Inst` line could proceed without validating its dependency selection, and a resolved `cc` path merely containing `gcc-13` could pass. The corrected acquisition path validates each selected version against the isolated signed snapshot metadata, refuses foreign labels, removals and changed root pins, and records the selection before installation. The `cc` selector resolves to the selected GCC binary and its exact package owner and version.

Focused fixtures cover valid acquisition, foreign or absent metadata, a changed dependency version, an unowned compiler path and a wrong compiler owner. The reviewer ran the language fixture, workflow checks, pages, tickets, Python syntax and diff checks. The earlier 43 workflow self-tests remain valid because the corrective commit did not change workflow ordering. The coordinator ran merged pages, tickets and diff checks without repeating the full suite.

The existing 0272 source, C and managed receipts remain intact. No SDK or package body was downloaded, no apt installation or compiler/consumer/container/Actions run occurred, and no SQL or DataFrame validation ran. Actual runner qualification remains in the existing release work.

Preparation lesson: a positive installed-tool fixture does not cover the acquisition branch. Include the selected package plan and literal downstream executable in the smallest useful proof before declaring tool provenance enforced.
