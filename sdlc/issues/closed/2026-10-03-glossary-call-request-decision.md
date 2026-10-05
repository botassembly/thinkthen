# A glossary for call, request and decision

Status: closed.
Resolution: 0402
Milestone: 0.2

Ian decided three terms that count the work:

- A **call** is one use of a function on any surface.
- A **request** is one send to a model.
- A **decision** is one question answered about one piece of evidence.

The repository has no glossary. `CONTRIBUTING.md` defines terms, so the three go there, and the specification links them.

`meta.requests` lists question keys, not requests, so its name clashes with the new term. `meta.requests_sent` counts requests and fits. Renaming `meta.requests` changes the result schema on every surface and every saved recording that `audit` and `diff` read. The ticket weighs a rename with a compatibility read of the old name against keeping the name and defining it in the glossary. The proposed default keeps the name in 0.2 and defines it, because a schema rename touches every binding. Ian can overturn that default.

Slice C landed the glossary and retained the compatibility metadata name. The short [0402 note](../../records/0402-documentation.md) records the behavior and remaining recipe limits; Git retains the original verification history.
