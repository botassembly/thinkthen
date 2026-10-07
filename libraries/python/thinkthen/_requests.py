"""Private named request builders; native admission and execution remain native."""
from __future__ import annotations
from dataclasses import dataclass
from ._complete import (
    ABSENT, Absent, Carrier, CandidateInput, ChooseSpec, Controls, DecideSpec,
    Files, FindSpec, ImageInput, QuestionFile, QuestionSet, RankSpec,
    RecognitionSpec, RecordInput, RelationSpec, ScoreSpec, Selection, TagSpec,
    TextInput, decode, to_json,
)

@dataclass(frozen=True, repr=False)
class Request(Carrier):
    function: str
    question: Carrier
    input: Selection
    controls: Controls

    def question_json(self):
        """The native grammar; a QuestionFile must be loaded by the native edge."""
        if isinstance(self.question, QuestionFile):
            raise ValueError("question files require the native loader")
        return to_json(self.question)


def _snapshot(value):
    return decode(type(value).__name__, to_json(value))


def _build(verb, question, source, controls):
    expected = {
        "decide": (DecideSpec, QuestionFile), "choose": (ChooseSpec, QuestionFile),
        "tag": (TagSpec, QuestionFile), "score": (ScoreSpec, QuestionFile),
        "filter": (DecideSpec, QuestionFile),
        "rank": (DecideSpec, ScoreSpec, QuestionSet, QuestionFile),
        "find": (FindSpec, QuestionFile), "annotate": (QuestionSet, QuestionFile),
        "recognize": (RecognitionSpec, QuestionFile), "relate": (RelationSpec, QuestionFile),
    }
    if not isinstance(question, expected[verb]): raise ValueError("wrong question kind")
    if not isinstance(source, (TextInput, RecordInput, CandidateInput, ImageInput, Files)): raise ValueError("explicit input required")
    question, source = _snapshot(question), _snapshot(source)
    controls = decode("Controls", {}) if controls is None else _snapshot(controls)
    if not isinstance(controls, Controls): raise ValueError("typed controls required")
    images = isinstance(source, ImageInput) or (isinstance(source, Files) and source.media == "image")
    if images and verb not in ("decide", "choose", "score"): raise ValueError("this function is text-only")
    if isinstance(source, CandidateInput) and verb != "find": raise ValueError("candidates require find")
    if isinstance(source, TextInput) and verb in ("filter", "rank", "find", "annotate", "relate"): raise ValueError("this function requires a complete record set")
    if isinstance(source, Files):
        if source.unit == "window":
            if source.window is ABSENT: raise ValueError("window requires a size")
        elif source.window is not ABSENT: raise ValueError("window size requires window units")
        if source.media == "image" and source.unit != "file": raise ValueError("image sources require file units")
    if isinstance(source, ImageInput) and not source.images: raise ValueError("images require attachments")
    if verb not in ("rank",) and controls.top is not ABSENT: raise ValueError("top requires rank")
    if verb != "find" and controls.none is not ABSENT: raise ValueError("none requires find")
    return Request(verb, question, source, controls)


def decide(question: DecideSpec | QuestionFile, source: Selection, controls: Controls | None = None) -> Request:
    return _build("decide", question, source, controls)

def choose(question: ChooseSpec | QuestionFile, source: Selection, controls: Controls | None = None) -> Request:
    return _build("choose", question, source, controls)

def tag(question: TagSpec | QuestionFile, source: Selection, controls: Controls | None = None) -> Request:
    return _build("tag", question, source, controls)

def score(question: ScoreSpec | QuestionFile, source: Selection, controls: Controls | None = None) -> Request:
    return _build("score", question, source, controls)

def filter(question: DecideSpec | QuestionFile, source: Selection, controls: Controls | None = None) -> Request:
    return _build("filter", question, source, controls)

def rank(question: RankSpec, source: Selection, controls: Controls | None = None) -> Request:
    return _build("rank", question, source, controls)

def find(question: FindSpec | QuestionFile, source: Selection, controls: Controls | None = None) -> Request:
    return _build("find", question, source, controls)

def annotate(question: QuestionSet | QuestionFile, source: Selection, controls: Controls | None = None) -> Request:
    return _build("annotate", question, source, controls)

def recognize(question: RecognitionSpec | QuestionFile, source: Selection, controls: Controls | None = None) -> Request:
    return _build("recognize", question, source, controls)

def relate(question: RelationSpec | QuestionFile, source: Selection, controls: Controls | None = None) -> Request:
    return _build("relate", question, source, controls)
