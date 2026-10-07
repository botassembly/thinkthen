"""The shared portable corpus as one request (ADR 0111, ticket 0304).

The content cut is gone, so the five portable texts ride one request. That
request keeps the fixture's state and model and carries the three fixture
bodies' questions in first-seen order. Each surface's portable check reads
the bodies its backend captured through `one_portable_request`, and a row's
keys through `question_keys`.
"""

import hashlib
import json
import re
from pathlib import Path

FIXTURES = Path(__file__).resolve().parents[2] / "specification/fixtures/batching"


def fixture_questions():
    """The fixture's state, model and questions in order, across its three bodies."""
    bodies = [json.loads((FIXTURES / f"portable-{at}.request.json").read_text()) for at in (1, 2, 3)]
    questions = [question for body in bodies
                 for _, question in sorted(body["questions"].items(), key=lambda item: int(item[0][1:]))]
    return bodies[0]["state"], bodies[0]["model"], questions


def one_portable_request(captured):
    """Assert the captured bodies (text or bytes) are one request holding the fixture's questions."""
    assert len(captured) == 1, captured
    sent = json.loads(captured[0])
    state, model, questions = fixture_questions()
    assert (sent["state"], sent["model"]) == (state, model), sent
    assert [sent["questions"][f"q{at}"] for at in range(1, len(sent["questions"]) + 1)] == questions, sent


def question_keys(url, body):
    """The authored portable questions' framed cache/2 keys in wire order.

    The controlled full/capture backend reports the requested fixture model.
    Keep these expectations independent of returned metadata and captured JSON.
    Historical cache/1 fixture validation retains its separate legacy oracle.
    """
    one_portable_request([body])
    state, model, questions = fixture_questions()

    # Match the endpoint resolver: lowercase scheme and literal host letters,
    # preserve host escapes and bracketed addresses, and drop trailing slashes.
    scheme, rest = url.strip().rstrip("/").split("://", 1)
    authority, separator, path = rest.partition("/")
    if not authority.startswith("["):
        host, colon, port = authority.partition(":")
        host = re.sub(r"%[0-9A-Fa-f]{2}|[A-Z]", lambda match:
                      match[0] if match[0].startswith("%") else match[0].lower(), host)
        authority = host + colon + port
    url = scheme.lower() + "://" + authority + separator + path

    def compact(value):
        return json.dumps(value, separators=(",", ":"), ensure_ascii=False)
    def key(question):
        parts = ["systemone", url, compact(model), compact(model),
                 compact(state), compact(question)]
        framed = b"thinkthen.question-key/2\0"
        for part in parts:
            encoded = part.encode("utf-8")
            framed += len(encoded).to_bytes(8, "big") + encoded
        return hashlib.sha256(framed).hexdigest()

    return [key(question) for question in questions]
