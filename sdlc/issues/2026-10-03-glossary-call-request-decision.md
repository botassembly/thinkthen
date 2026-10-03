# A glossary for call, request and decision

Status: open. Filed 2026-10-03 from the docs message "Proxy, terms and function drafts decided" (ask 2), on Ian's product decision of 2026-10-02. Owner: the queue owner.
Milestone: 0.2

Ian decided three terms that count the work:

- A **call** is one use of a function on any surface.
- A **request** is one send to a model.
- A **decision** is one question answered about one piece of evidence.

The repository has no glossary. `CONTRIBUTING.md` defines terms, so the three go there, and the specification links them.

`meta.requests` lists question keys, not requests, so its name clashes with the new term. `meta.requests_sent` counts requests and fits. Renaming `meta.requests` changes the result schema on every surface and every saved recording that `audit` and `diff` read. The ticket weighs a rename with a compatibility read of the old name against keeping the name and defining it in the glossary. The proposed default keeps the name in 0.2 and defines it, because a schema rename touches every binding. Ian can overturn that default.
