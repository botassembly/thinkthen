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

Slice C adds the glossary and retains the compatibility metadata name. Independent High source ACCEPT and the named full test/specification/actual 350 replay/strict zero-stale/final site checkpoint are retained in `sdlc/records/0402-c-glossary-build.md` and its final manifest. This closure reaches main with the reviewed C slice landing.
