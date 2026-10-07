"""Independent question-key/2 framing over validated compact wire bytes."""

import hashlib
import json


def question_keys(url, body, reported_model):
    """Use the authored response model and original state/question spans."""
    if isinstance(body, bytes):
        body = body.decode("utf-8")
    decoder = json.JSONDecoder()

    def members(text):
        parsed, end = decoder.raw_decode(text)
        if not isinstance(parsed, dict) or end != len(text) or text[:1] != "{":
            raise ValueError("question keys require a compact JSON object")
        found, at = {}, 1
        while text[at] != "}":
            name, close = decoder.raw_decode(text, at)
            if text[close] != ":":
                raise ValueError("question keys require compact member separators")
            start = close + 1
            _, close = decoder.raw_decode(text, start)
            found[name] = text[start:close]
            if text[close] not in ",}":
                raise ValueError("question keys require compact member separators")
            at = close + (text[close] == ",")
        return found

    request = members(body)
    questions = members(request["questions"])
    answered = json.dumps(reported_model, separators=(",", ":"), ensure_ascii=False)
    head = ["systemone", url, request["model"], answered, request["state"]]
    keys = []
    for name in sorted(questions, key=lambda name: int(name[1:])):
        digest = hashlib.sha256(b"thinkthen.question-key/2\0")
        for part in [*head, questions[name]]:
            raw = part.encode("utf-8")
            digest.update(len(raw).to_bytes(8, "big"))
            digest.update(raw)
        keys.append(digest.hexdigest())
    return keys
