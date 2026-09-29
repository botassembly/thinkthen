#!/usr/bin/env python3
"""Installed PostgreSQL find edges through the existing counted loopback proxy."""

import json
import pathlib
import subprocess
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[2] / "sqlite" / "tests"))
from conditional_backend import ConditionalBackend  # noqa: E402

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance" / "children"))
from children import child_env  # noqa: E402


CASES = {
    "duplicate": (["same", "same", "other"], False, [0.1, 0.8, 0.1], 1),
    "real_tie": (["first", "second"], False, [0.5, 0.5], 0),
    "none_tie": (["first", "second"], True, [0.4, 0.2, 0.4], None),
    "max_units": ([f"unit {index}" for index in range(255)], False, [1.0] + [0.0] * 254, 0),
    "max_none": ([f"unit {index}" for index in range(254)], True, [1.0] + [0.0] * 254, 0),
}


def sql(socket, *statements):
    command = ["psql", "-X", "-q", "-At", "-v", "VERBOSITY=verbose", "-h", socket,
               "-U", "postgres", "-d", "postgres"]
    for statement in ["SET statement_timeout='30s'", *statements]:
        command += ["-c", statement]
    done = subprocess.run(command, capture_output=True, text=True, timeout=30,
                          check=False, env=child_env())
    return done.stdout.strip(), done.stderr.strip()


def quoted(text):
    return "'" + text.replace("'", "''") + "'"


def array(values):
    return "ARRAY[" + ",".join(quoted(value) for value in values) + "]::text[]"


def proxy(base, mode):
    units, none, probabilities, _ = CASES[mode]
    names = {f"u{index:03}": value for index, value in enumerate(probabilities[:len(units)], 1)}
    if none:
        names["none"] = probabilities[-1]
    reply = json.dumps({"model": "jev-latest", "answers": {"q1": {
        "type": "choice", "probabilities": names}}}).encode()
    with ConditionalBackend(base, reply=reply) as backend:
        print(backend.server.server_port, flush=True)
        sys.stdin.readline()
        print(backend.count(), flush=True)


def verify(socket, mode):
    units, none, probabilities, selected = CASES[mode]
    setting = "'{\"none\":true}'::json" if none else "NULL::json"
    statement = f"SELECT thinkthen_find('Which unit?', {array(units)}, {setting})"
    out, error = sql(socket, statement)
    if error:
        raise AssertionError(error)
    expected = {
        "index": selected,
        "value": units[selected] if selected is not None else None,
        "probability": probabilities[selected] if selected is not None else probabilities[-1],
        "candidates": [{"index": index if index < len(units) else None,
                        "probability": probability}
                       for index, probability in enumerate(probabilities)],
    }
    got = json.loads(out)
    if got != expected:
        raise AssertionError(f"{mode}: got {got!r}, expected {expected!r}")


def invalid(socket):
    nulls = [
        "SELECT thinkthen_find(NULL, ARRAY['a','b']) IS NULL",
        "SELECT thinkthen_find('Which?', NULL::text[]) IS NULL",
        "SELECT thinkthen_find('Which?', NULL::text[], NULL::json) IS NULL",
        "SELECT thinkthen_find('Which?', ARRAY[]::text[]) IS NULL",
    ]
    for statement in nulls:
        out, error = sql(socket, statement)
        if (out, error) != ("t", ""):
            raise AssertionError(f"null/empty: {statement}: {(out, error)!r}")
    refused = [
        "SELECT thinkthen_find('Which?', ARRAY['one'])",
        "SELECT thinkthen_find('Which?', ARRAY['one',NULL])",
        "SELECT thinkthen_find('Which?', ARRAY['one','   '])",
        "SELECT thinkthen_find('   ', ARRAY['one','two'])",
        "SELECT thinkthen_find('Which?', array_fill('x'::text, ARRAY[256]))",
        "SELECT thinkthen_find('Which?', array_fill('x'::text, ARRAY[255]), '{\"none\":true}'::json)",
        "SELECT thinkthen_find('Which?', ARRAY[repeat('x',16777216),'y'])",
    ]
    for statement in refused:
        _, error = sql(socket, statement)
        if "ERROR:  22023: thinkthen usage:" not in error:
            raise AssertionError(f"expected pre-send Usage: {statement}: {error}")
    _, error = sql(socket, "SELECT thinkthen_find('Which?', ARRAY['one','two'], true)")
    if "find's none and deadline moved into the settings object" not in error:
        raise AssertionError(f"old positional none must refuse: {error}")
    _, error = sql(socket, "SELECT thinkthen_find('Which?', ARRAY[1,2])")
    if "ERROR:  42883:" not in error:
        raise AssertionError(f"wrong SQL type must fail at binding: {error}")
    _, error = sql(socket, "SET thinkthen.deadline_ms = 0",
                   "SELECT thinkthen_find('Which?', ARRAY['one','two'])")
    if "ERROR:  57014: thinkthen deadline:" not in error:
        raise AssertionError(f"expired GUC deadline: {error}")
    _, error = sql(socket, "SET thinkthen.max_requests_total = 0",
                   "SELECT thinkthen_find('Which?', ARRAY['one','two'])")
    if "ERROR:  22023: thinkthen usage: thinkthen.max_requests_total" not in error:
        raise AssertionError(f"spent request total: {error}")


if __name__ == "__main__":
    command, *args = sys.argv[1:]
    {"proxy": proxy, "verify": verify, "invalid": invalid}[command](*args)
