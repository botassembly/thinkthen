# A backend compliance check

Status: Closed on 2026-09-25 after a check against main. Ticket 0121 thinkthen check landed at cad9237a (sdlc/records/0121-build-backend-check.md). Earlier status: Ticketed as 0121 (`thinkthen check`), design accepted 2026-09-24 on `ticket/0121-backend-check`. It builds after 0086 lands.

Filed by the marketing session on 2026-09-24, from Ian's note "ThinkThen API checker".

## Ask

A user gives ThinkThen an address and a key and asks one question: does this backend work with ThinkThen? Today `thinkthen status` reports local configuration, cache, and counts. It sends nothing to the backend.

Ian wants the check inside the ThinkThen CLI. It should confirm four things:

1. **Connection.** The address answers.
2. **Key.** The backend accepts the key.
3. **Compliance by endpoint.** Every endpoint ThinkThen uses exists: binary, multi-class, and multi-label.
4. **Compliance by field.** Every request field ThinkThen sends is accepted, and every reply field it reads comes back. That covers context, options, descriptions, thresholds, and probabilities.

Ideally the check sends one complete request per endpoint, using every field and option ThinkThen can send. It then reports each finding as critical or warning. A critical finding means a ThinkThen function will fail. A warning means an optional field is missing or ignored. It exits 0 only when nothing is critical, so a script or a list can trust it.

## Why

- The ideal state says a local or open model is reached by a small server presenting the same shape.
- Several open servers now claim that shape: Kev, SemIf, openjev-sglang, and system-one. Nobody can yet say which ones work with ThinkThen.
- The marketing session keeps a private awesome list of ThinkThen backends. A backend joins it only after this check passes.

The check spends a few small requests per run. Ian can overturn the command name and the critical and warning split.
