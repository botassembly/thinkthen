"""Validate bounded candidates, replay choose, and print the proposed span."""

import json
import os
from pathlib import Path
import subprocess

fixture = Path(__file__).resolve().parent
original = (fixture / "text.txt").read_text(encoding="utf-8")
question = fixture / "flagged-span.json"
candidates = json.loads(question.read_text(encoding="utf-8"))["options"]
if not 2 <= len(candidates) <= 8 or "keep" not in candidates:
    raise ValueError("Supply keep and a bounded list of candidate spans")
for candidate in candidates.values():
    start, end = candidate["start"], candidate["end"]
    if type(start) is not int or type(end) is not int:
        raise ValueError("Offsets must be integers")
    if not 0 <= start < end <= len(original):
        raise ValueError("Span lies outside the original text")
    if original[start:end] != candidate["text"]:
        raise ValueError("Span text does not match its original coordinates")
environment = dict(os.environ)
environment.pop("THINKTHEN_API_KEY", None)
environment.pop("THINKTHEN_BASE_URL", None)
result = subprocess.run(
    ["thinkthen", "choose", f"@{question}",
     "--url", (fixture / "url.txt").read_text().strip(),
     "--model", "jev-1.13.0", "--replay", str(fixture / "recording"),
     "--no-cache"],
    input=original, text=True, capture_output=True, env=environment,
)
if result.returncode not in (0, 3):
    raise RuntimeError(result.stderr.strip() or f"choose exited {result.returncode}")
pick = json.loads(result.stdout)
if pick is not None and pick not in candidates:
    raise ValueError("Unknown candidate label")
proposed = candidates["keep"] if pick is None else candidates[pick]
print(json.dumps({"pick": pick, "proposed": proposed}, ensure_ascii=False,
                 separators=(",", ":")))
