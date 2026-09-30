"""The shared portable corpus as one request (ADR 0111, ticket 0304).

The content cut is gone, so the five portable texts ride one request. That
request keeps the fixture's state and model and carries the three fixture
bodies' questions in first-seen order. Each surface's portable check reads
the bodies its backend captured through `one_portable_request`, and a row's
keys through `question_keys`.
"""

import hashlib
import json
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
    """Each question's key in one request body, in wire order (ADR 0111
    section 2): the SHA-256 of the adapter, the URL, the model, the state and
    one question as the body carries them, joined by line feeds."""
    request = json.loads(body)

    def compact(value):
        return json.dumps(value, separators=(",", ":"), ensure_ascii=False)
    head = "\n".join(["systemone", url, compact(request["model"]), compact(request["state"])])
    names = sorted(request["questions"], key=lambda name: int(name[1:]))
    return [hashlib.sha256(f"{head}\n{compact(request['questions'][name])}".encode()).hexdigest() for name in names]
