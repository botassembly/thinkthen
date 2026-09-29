"""Installed native STRUCT preview and active send-total boundary."""

from __future__ import annotations

import sys

from harness import Backend, case, expect, main, rows, run, said

BODY = '{"state":"Refund me please.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}'
P1 = "thinkthen_plan('asks for a refund', '{\"7\":\"Refund me please.\"}', '{}')"


@case
def p1_native_struct_is_keyless_and_sends_nothing():
    expect(len(BODY.encode()), 120, "independent P1 byte count")
    with Backend() as backend:
        got = run([f"SELECT {P1}", f"SELECT typeof({P1})"], backend.base(), keyless=True)
        expect(rows(got[0]), [[{
            "records": 1, "requests": 1, "estimated_bytes": 120,
            "estimated_input_tokens": {"lower": 61, "upper": 109},
            "upper_bound": False, "first_body": BODY,
        }]], "P1 native value and independent request literal")
        expect("STRUCT" in rows(got[1])[0][0], True, "native SQL type")
        expect(backend.count(), 0, "keyless plan accepts no request")


@case
def plan_refusals_and_named_binding_never_send():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_plan(settings := '{}', keyed_json := '{\"7\":\"Refund me please.\"}', question := 'asks for a refund')",
            "SELECT thinkthen_plan('asks for a refund', '{\"7\":\"Refund me please.\"}', 4)",
            "SELECT thinkthen_plan('asks for a refund', '[\"not keyed\"]')",
            "SELECT thinkthen_plan('asks for a refund', '{\"7\":\"one\",\"7\":\"two\"}')",
            "SELECT thinkthen_plan('{\"decide\":\"asks for a refund\",\"threshold\":0.7}', '{\"7\":\"Refund me please.\"}', '{\"threshold\":0.8}')",
            "SELECT thinkthen_plan('{\"decide\":\"Refund?\",\"batch\":1}', '{\"7\":\"Refund me\"}', '{\"batch\":2}')",
            "SELECT thinkthen_plan('asks for a refund', '{\"7\":\"Refund me please.\"}', '{\"max_estimated_input_tokens_total\":100}')",
            "SELECT thinkthen_plan(stare := 'x', keyed_json := '{}', question := 'asks for a refund')",
        ], backend.base(), keyless=True)
        expect(rows(got[0])[0][0]["first_body"], BODY, "out-of-order named preview")
        expect(said(got[5]), "thinkthen usage: settings repeats `batch` from the question or named arguments (retryable: no)",
               "question batch and settings batch conflict before planning")
        for result in got[1:7]:
            expect(said(result).startswith("thinkthen usage:"), True, "typed/settings/key refusal")
        expect("does not support the supplied arguments" in got[7]["error"], True, "unknown name")
        expect(backend.count(), 0, "all previews and refusals send nothing")


@case
def positive_process_total_denies_the_next_actual_send():
    with Backend() as backend:
        got = run([
            "SET thinkthen_cache = 'off'",
            "SET thinkthen_max_retries = 0",
            "SET thinkthen_max_requests_total = 1",
            f"SELECT {P1}",
            "SELECT thinkthen_decide('asks for a refund', 'Refund me please.')",
            "SELECT thinkthen_decide('asks for a refund', 'Different text.')",
            f"SELECT {P1}",
        ], backend.base())
        expect(rows(got[3])[0][0]["requests"], 1, "plan does not spend total")
        expect(rows(got[4]), [[True]], "one admitted send")
        expect(said(got[5]), "thinkthen usage: this process has spent its request total of 1; raise SET thinkthen_max_requests_total or RESET it (retryable: no)", "typed spent-total refusal")
        expect(rows(got[6])[0][0]["first_body"], BODY, "plan after the total is spent")
        expect(backend.count(), 1, "only the admitted call reaches loopback")


if __name__ == "__main__":
    sys.exit(main())
