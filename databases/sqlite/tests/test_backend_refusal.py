#!/usr/bin/env python3
"""The shared backend member refuses atomically until SQLite gains selection."""
import json
from helper import Backend, Child, environment, expect, main


def test_backend_refusal_preserves_configuration_without_sends():
    backend = Backend()
    process = Child('''
db = connect()
valid = run(db, "SELECT thinkthen_configure(?)", ('{"model":"retained","cache":false}',))
refused = [run(db, "SELECT thinkthen_configure(?)", (source,)) for source in (
    '{"backend":"local","model":"replaced"}', '{"backend":1}',
    '{"backend":"local","backend":"other"}')]
say(valid=valid, refused=refused)
input()
answer = run(db, "SELECT thinkthen_details('attention?', 'refund')")
say(answer=answer)
''', environment(backend))
    got = process.read()
    expect(got["valid"], [['{"model":"retained","cache":false}']], "prior configure")
    expect(got["refused"][0], "thinkthen usage: settings JSON has unknown key backend (retryable: no)", "backend refusal")
    expect(all(message.startswith("thinkthen usage:") for message in got["refused"]), True, "type and duplicate remain usage")
    expect(backend.count(), 0, "refused replacement sends nothing")
    expect("sk-sqlite-loopback" in json.dumps(got), False, "refusal keeps key secret")
    process.send()
    result = process.result()
    details = json.loads(result["answer"][0][0])
    expect(details["meta"]["model"], "retained", "refusal leaves prior configuration")
    expect(backend.count(), 1, "retained configuration sends once")


if __name__ == '__main__':
    raise SystemExit(main(globals()))
