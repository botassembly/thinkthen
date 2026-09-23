#!/usr/bin/env python3
"""Review 4, item 9: details on DECIDE always reads NULL probability
and sends, and choose and tag send '' where the README promises NULL.

The regression: lib.rs set every row's null bit and never cleared it,
so the struct children read NULL no matter what the engine answered
(round three sent 0.97|1). Choose and tag rows carry model '' instead
of NULL.

PASS on the fix: a decide's probability and sends are present; a choose
or tag row's model reads NULL. The three verbs are probed with their
recorded questions.
"""

from __future__ import annotations

import json
import sys

from review4_lib import CASES, Harness, finish, verdict


def questions() -> dict[str, str]:
    """One recorded question per kind, spelled as the surface takes them."""
    cases = json.loads(CASES.read_text())["cases"]
    wanted = {}
    for case in cases:
        verb = case.get("verb")
        question = case.get("question", {})
        if verb in {"decide", "choose", "tag"} and verb not in wanted:
            wanted[verb] = json.dumps(question, separators=(",", ":"))
    return wanted


def main() -> int:
    harness = Harness()
    d = harness.open()
    c = harness.connect(d)
    harness.load(c)
    ask = questions()

    results = []

    # DECIDE: probability and sends must be present.
    ok, error, rows = harness.run(
        c,
        f"SELECT audit.probability, audit.sends FROM "
        f"(SELECT thinkthen_details('{ask['decide']}', 'please refund order 4471') "
        f"AS audit)",
    )
    results.append(
        verdict(
            "details on a decide question carries probability and sends",
            ok and rows and rows[0][0] is not None and rows[0][1] is not None,
            f"probability={rows[0][0] if rows else None} "
            f"sends={rows[0][1] if rows else None}"
            + ("" if ok else f" — {error}"),
        )
    )

    # CHOOSE and TAG: model reads NULL, never ''.
    for kind in ("choose", "tag"):
        ok, error, rows = harness.run(
            c,
            f"SELECT audit.model FROM (SELECT thinkthen_details('{ask[kind]}', "
            f"'please refund order 4471') AS audit)",
        )
        model = rows[0][0] if rows else "<no row>"
        results.append(
            verdict(
                f"details on a {kind} question reads model NULL, never ''",
                ok and model is None,
                f"model={model!r}" + ("" if ok else f" — {error}"),
            )
        )

    return 0 if all(r == 0 for r in results) else 1


if __name__ == "__main__":
    finish(main())
