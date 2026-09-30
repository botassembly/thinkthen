"""The question key of ADR 0111 section 2, as the tests and the shared-case
runner recompute it from a request body."""

import hashlib
import json


def question_keys(url, body):
    """Each question's key in one request body, in wire order: the SHA-256 of
    the adapter, the URL, the model, the state and one question as the body
    carries them, joined by line feeds."""
    request = json.loads(body)

    def compact(value):
        return json.dumps(value, separators=(",", ":"), ensure_ascii=False)
    head = "\n".join(["systemone", url, compact(request["model"]), compact(request["state"])])
    names = sorted(request["questions"], key=lambda name: int(name[1:]))
    return [hashlib.sha256(f"{head}\n{compact(request['questions'][name])}".encode()).hexdigest()
            for name in names]
