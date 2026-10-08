from __future__ import annotations
from dataclasses import dataclass, fields
from typing import Literal, Mapping, Union
from types import MappingProxyType
import math
import re

JsonValue = Union[None, bool, int, float, str, tuple["JsonValue", ...], Mapping[str, "JsonValue"]]

QuestionText = str | tuple[JsonValue, ...] | Mapping[str, JsonValue]
Description = QuestionText | None

class Absent:
    def __repr__(self): return "ABSENT"
ABSENT = Absent()

class Carrier:
    def __repr__(self): return f"<{type(self).__name__}: content withheld>"

class Identity(str):
    def __new__(cls, value):
        if not isinstance(value, str) or re.fullmatch(r"[0-9a-f]{64}", value) is None:
            raise ValueError("invalid identity")
        return super().__new__(cls, value)
    def __repr__(self): return f"<{type(self).__name__}>"

class CallId(Identity): pass

class SdkRequestId(Identity): pass

class ObservationId(Identity): pass

class FailureId(Identity): pass

class AnswerId(Identity): pass

class Digest(Identity): pass

@dataclass(frozen=True, repr=False, kw_only=True)
class Position(Carrier):
    file: str | Absent = ABSENT
    first: int | Absent = ABSENT
    last: int | Absent = ABSENT
    images: tuple[str, ...] | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class Usage(Carrier):
    input_tokens: int | Absent = ABSENT
    output_tokens: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ProfileWarning(Carrier):
    tuned_for: str
    running: str

@dataclass(frozen=True, repr=False, kw_only=True)
class BatchWarning(Carrier):
    tuned_for: int | Literal['max']
    running: int | Literal['max']

@dataclass(frozen=True, repr=False, kw_only=True)
class Attempt(Carrier):
    ordinal: int
    request_sha256: Digest
    wall_ms: int
    outcome: Literal['ok', 'status', 'transport']
    sdk_request_id: SdkRequestId
    status: int | Absent = ABSENT
    server_ms: int | Absent = ABSENT
    request_id: str | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class Facts(Carrier):
    call_id: CallId
    records: int
    requests_sent: int
    cache_answers: int
    seconds: float
    input_tokens: int | Absent = ABSENT
    output_tokens: int | Absent = ABSENT
    model: str | Absent = ABSENT
    estimated_cost_usd: str | Absent = ABSENT
    command_ms: int | Absent = ABSENT
    attempts: tuple[Attempt, ...] | Absent = ABSENT
    held_model_mismatch: bool | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class QuestionSource(Carrier):
    origin: Literal['live', 'cache', 'replay', 'proxy', 'memory']
    answered_by: str
    batch_size: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class Observed(Carrier):
    observation_id: ObservationId

@dataclass(frozen=True, repr=False, kw_only=True)
class FailedObservation(Carrier):
    failure_id: FailureId

@dataclass(frozen=True, repr=False, kw_only=True)
class Meta(Carrier):
    tool: str
    url: str
    model: str
    requests_sent: int
    cached: bool
    requests: tuple[Digest, ...]
    failed_questions: int
    origin: Literal['live', 'cache', 'replay', 'proxy', 'memory'] | None
    question_sources: tuple[QuestionSource, ...]
    observations: tuple[Observation, ...]
    question_sha256: Digest | Absent = ABSENT
    questions_sha256: Digest | Absent = ABSENT
    answered_by: str | Absent = ABSENT
    usage: Usage | Absent = ABSENT
    profile_warning: ProfileWarning | Absent = ABSENT
    batch_setting: int | Literal['max'] | Absent = ABSENT
    batch_warning: BatchWarning | Absent = ABSENT
    context_sha256: Digest | Absent = ABSENT
    attempts: tuple[Attempt, ...] | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class YesNo(Carrier):
    kind: Literal['yes_no']
    probability: float

@dataclass(frozen=True, repr=False, kw_only=True)
class Choice(Carrier):
    kind: Literal['choice']
    pick: str
    probabilities: Mapping[str, float]
    confidence: float | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class Tags(Carrier):
    kind: Literal['tag']
    probabilities: Mapping[str, float]

@dataclass(frozen=True, repr=False, kw_only=True)
class Score(Carrier):
    kind: Literal['score']
    level: str
    probabilities: Mapping[str, float]
    confidence: float | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class FindAnswer(Carrier):
    kind: Literal['find']
    pick: str
    probabilities: Mapping[str, float]
    confidence: float | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class DecideQuestion(Carrier):
    verb: Literal['decide']
    text: QuestionText
    true_: Description | Absent = ABSENT
    false_: Description | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ChooseQuestion(Carrier):
    verb: Literal['choose']
    text: QuestionText
    options: tuple[str, ...]
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class TagQuestion(Carrier):
    verb: Literal['tag']
    text: QuestionText
    labels: tuple[str, ...]
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ScoreQuestion(Carrier):
    verb: Literal['score']
    text: QuestionText
    levels: tuple[str, ...]
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class FindQuestion(Carrier):
    verb: Literal['find']
    text: QuestionText
    none: bool
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RelationRule(Carrier):
    name: str
    source: str
    target: str
    reads: str
    either: bool
    single: bool | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RelateFields(Carrier):
    name: str
    kind: str

@dataclass(frozen=True, repr=False, kw_only=True)
class RelateQuestion(Carrier):
    verb: Literal['relate']
    fields: RelateFields | None
    relations: tuple[RelationRule, ...]
    threshold: float | str | None
    profile: str | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RecognizeQuestion(Carrier):
    instructions: str | Absent = ABSENT
    entity_definition: str | Absent = ABSENT
    verb: Literal['recognize']
    kinds: Mapping[str, Description]
    relations: tuple[RelationRule, ...] | Absent = ABSENT
    threshold: float | str | None
    relation_threshold: float | str | None
    on: str | tuple[str, ...] | Absent = ABSENT
    profile: str | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class Failure(Carrier):
    kind: Literal['backend']
    cause: Literal['missing_answer', 'wrong_kind', 'missing_probability', 'invalid_probability', 'invalid_distribution', 'unexpected_probability']

@dataclass(frozen=True, repr=False, kw_only=True)
class FailedField(Carrier):
    failed: Failure

@dataclass(frozen=True, repr=False, kw_only=True)
class Entity(Carrier):
    text: str
    start: int
    end: int
    length: int
    kind: str
    strength: float
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class Endpoint(Carrier):
    name: str
    kind: str
    record: JsonValue | Absent = ABSENT
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class Edge(Carrier):
    relation: str
    source: Endpoint
    target: Endpoint
    probability: float
    either: Literal[True] | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class EntityEdge(Carrier):
    relation: str
    source: Entity
    target: Entity
    probability: float
    either: Literal[True] | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class Recognition(Carrier):
    entities: tuple[Entity, ...]
    relations: tuple[EntityEdge, ...] | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class PieceOdds(Carrier):
    start: int
    end: int
    tags: Mapping[str, float]

@dataclass(frozen=True, repr=False, kw_only=True)
class NameOdds(Carrier):
    start: int
    end: int
    kinds: Mapping[str, float] | None
    edges: Mapping[str, float] | None

@dataclass(frozen=True, repr=False, kw_only=True)
class Span(Carrier):
    start: int
    end: int

@dataclass(frozen=True, repr=False, kw_only=True)
class PairOdds(Carrier):
    relation: str
    source: Span
    target: Span
    probability: float

@dataclass(frozen=True, repr=False, kw_only=True)
class RecognitionAnswer(Carrier):
    pieces: tuple[PieceOdds, ...]
    names: tuple[NameOdds, ...]
    pairs: tuple[PairOdds, ...]

@dataclass(frozen=True, repr=False, kw_only=True)
class AnnotationSuccess(Carrier):
    answer_id: AnswerId
    value: SuccessValue
    question: AtomicQuestion
    answer: AtomicAnswer
    threshold: float | str | None
    request: Digest

@dataclass(frozen=True, repr=False, kw_only=True)
class AnnotationFailure(Carrier):
    failure_id: FailureId
    question: AtomicQuestion
    failure: Failure
    request: Digest

@dataclass(frozen=True, repr=False, kw_only=True)
class RelationSuccess(Carrier):
    relation: str
    reads: str
    method: str
    direction: str
    source: Endpoint
    target: Endpoint | None
    request: Digest
    answer_id: AnswerId
    probability: float
    accepted: bool
    answer: AtomicAnswer | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RelationFailure(Carrier):
    relation: str
    reads: str
    method: str
    direction: str
    source: Endpoint
    target: Endpoint | None
    request: Digest
    failure_id: FailureId
    failure: Failure

@dataclass(frozen=True, repr=False, kw_only=True)
class RelationAnswer(Carrier):
    questions: tuple[RelationEntry, ...]

@dataclass(frozen=True, repr=False, kw_only=True)
class DecideResult(Carrier):
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    meta: Meta
    value: bool | None
    question: DecideQuestion
    answer: YesNo
    threshold: float | str | None
    input: JsonValue | Absent = ABSENT
    position: Position | Absent = ABSENT
    input_file: str | Absent = ABSENT
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT
    source: PhysicalSource | Absent = ABSENT
    images: tuple[NativeImage, ...] | Absent = ABSENT

    index: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ChooseResult(Carrier):
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    meta: Meta
    value: str | None
    question: ChooseQuestion
    answer: Choice
    threshold: float | str | None
    input: JsonValue | Absent = ABSENT
    position: Position | Absent = ABSENT
    input_file: str | Absent = ABSENT
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT
    source: PhysicalSource | Absent = ABSENT
    images: tuple[NativeImage, ...] | Absent = ABSENT

    index: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class TagResult(Carrier):
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    meta: Meta
    value: tuple[str, ...]
    question: TagQuestion
    answer: Tags
    threshold: float | str | None
    input: JsonValue | Absent = ABSENT
    position: Position | Absent = ABSENT
    input_file: str | Absent = ABSENT
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT
    source: PhysicalSource | Absent = ABSENT

    index: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ScoreResult(Carrier):
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    meta: Meta
    value: float
    question: ScoreQuestion
    answer: Score
    threshold: None
    input: JsonValue | Absent = ABSENT
    position: Position | Absent = ABSENT
    input_file: str | Absent = ABSENT
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT
    source: PhysicalSource | Absent = ABSENT
    images: tuple[NativeImage, ...] | Absent = ABSENT

    index: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class FilterResult(Carrier):
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    meta: Meta
    value: bool
    input: JsonValue
    question: DecideQuestion
    answer: YesNo
    threshold: float | str | None
    position: Position | Absent = ABSENT
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT
    source: PhysicalSource | Absent = ABSENT

    index: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RankMemberResult(Carrier):
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    value: int
    question: DecideQuestion
    answer: YesNo
    threshold: None
    meta: Meta
    source: PhysicalSource | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RankMember(Carrier):
    name: str
    result: RankMemberResult

@dataclass(frozen=True, repr=False, kw_only=True)
class RankResult(Carrier):
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    meta: Meta
    value: int
    input: JsonValue
    question: AtomicQuestion
    answer: AtomicAnswer
    threshold: None
    question_name: str | Absent = ABSENT
    position: Position | Absent = ABSENT
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT
    source: PhysicalSource | Absent = ABSENT

    index: int | Absent = ABSENT
    members: tuple[RankMember, ...] | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class FindResult(Carrier):
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    meta: Meta
    value: JsonValue
    question: FindQuestion
    answer: FindAnswer
    threshold: None
    position: Position | Absent = ABSENT
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT

    index: int | None | Absent = ABSENT
    candidates: tuple[FindCandidate, ...] | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class AnnotateResult(Carrier):
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    meta: Meta
    input: JsonValue
    value: Mapping[str, AnnotatedValue]
    answers: Mapping[str, AnnotationEntry]
    position: Position | Absent = ABSENT
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT

    index: int | Absent = ABSENT
    source: PhysicalSource | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RecognizeResult(Carrier):
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    meta: Meta
    value: Recognition
    question: RecognizeQuestion
    answer: RecognitionAnswer
    input: JsonValue | Absent = ABSENT
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT

    index: int | Absent = ABSENT
    source: PhysicalSource | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RelateResult(Carrier):
    input: JsonValue | Absent = ABSENT
    schema: Literal['thinkthen.result/2']
    answer_id: AnswerId
    meta: Meta
    value: tuple[Edge, ...]
    question: RelateQuestion
    answer: RelationAnswer
    file: str | Absent = ABSENT
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT

    index: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class CallError(Carrier):
    kind: Literal['usage', 'backend', 'local', 'cancelled', 'deadline', 'defect']
    message: str
    retryable: bool
    facts: Facts | Absent = ABSENT
    attempts: tuple[Attempt, ...] | Absent = ABSENT
    stopped: Stopped | Absent = ABSENT

Observation = Observed | FailedObservation

AtomicAnswer = YesNo | Choice | Tags | Score | FindAnswer

AtomicQuestion = DecideQuestion | ChooseQuestion | TagQuestion | ScoreQuestion | FindQuestion

AnnotationEntry = AnnotationSuccess | AnnotationFailure

RelationEntry = RelationSuccess | RelationFailure

Result = DecideResult | ChooseResult | TagResult | ScoreResult | FilterResult | RankResult | FindResult | AnnotateResult | RecognizeResult | RelateResult

SuccessValue = bool | None | str | float | tuple[str, ...]

AnnotatedValue = bool | None | str | float | tuple[str, ...] | FailedField


_ALIASES = {
    'SuccessValue': ['bool', 'null', 'str', 'number', '[str]'],
    'Observation': ['Observed', 'FailedObservation'],
    'AtomicAnswer': ['YesNo', 'Choice', 'Tags', 'Score', 'FindAnswer'],
    'AtomicQuestion': ['DecideQuestion', 'ChooseQuestion', 'TagQuestion', 'ScoreQuestion', 'FindQuestion'],
    'AnnotationEntry': ['AnnotationSuccess', 'AnnotationFailure'],
    'RelationEntry': ['RelationSuccess', 'RelationFailure'],
    'Result': ['DecideResult', 'ChooseResult', 'TagResult', 'ScoreResult', 'FilterResult', 'RankResult', 'FindResult', 'AnnotateResult', 'RecognizeResult', 'RelateResult'],
    'AnnotatedValue': ['bool', 'null', 'str', 'number', '[str]', 'FailedField'],
}

_ENUMS = {
    'origin': ['live', 'cache', 'replay', 'proxy', 'memory'],
    'outcome': ['ok', 'status', 'transport'],
    'cause': ['missing_answer', 'wrong_kind', 'missing_probability', 'invalid_probability', 'invalid_distribution', 'unexpected_probability'],
    'error_kind': ['usage', 'backend', 'local', 'cancelled', 'deadline', 'defect'],
}



def _invalid():
    # Never quote the offending payload in a diagnostic.
    raise ValueError("invalid complete result")


def _json(value):
    if value is None or type(value) in (str, bool, int):
        return value
    if type(value) is float and math.isfinite(value):
        return value
    if isinstance(value, (list, tuple)):
        return tuple(_json(x) for x in value)
    if isinstance(value, Mapping) and all(type(k) is str for k in value):
        return MappingProxyType({k: _json(v) for k, v in value.items()})
    return _invalid()


def decode(kind, value):
    """Validate a result/2 carrier, preserving absence, null and array order."""
    if kind in _ALIASES or "|" in kind:
        for variant in _ALIASES.get(kind, kind.split("|")):
            try:
                return decode(variant, value)
            except ValueError:
                pass
        return _invalid()
    if kind.startswith("["):
        if not isinstance(value, (list, tuple)): return _invalid()
        return tuple(decode(kind[1:-1], x) for x in value)
    if kind.startswith("{"):
        if not isinstance(value, Mapping) or not all(type(k) is str for k in value): return _invalid()
        return MappingProxyType({k: decode(kind[1:-1], v) for k, v in value.items()})
    if kind in _MODELS:
        shape = _MODELS[kind]
        if not isinstance(value, Mapping) or set(value) - {k.rstrip("?") for k in shape}: return _invalid()
        held = {}
        for key, typed in shape.items():
            name = key.rstrip("?")
            if name not in value:
                if not key.endswith("?"): return _invalid()
                continue
            held[name + "_" if name in ("true", "false") else name] = decode(typed, value[name])
        _check(kind, value)
        return globals()[kind](**held)
    if kind == "one":
        if type(value) is int and value == 1: return value
        return _invalid()
    if kind == "bytes":
        if isinstance(value, (bytes, bytearray)): return bytes(value)
        return _invalid()
    if kind in _ENUMS:
        if type(value) is str and value in _ENUMS[kind]: return value
        return _invalid()
    if kind in ("CallId", "SdkRequestId", "ObservationId", "FailureId", "AnswerId", "Digest"):
        return globals()[kind](value)
    if kind.startswith("="):
        literal = True if kind == "=true" else kind[1:]
        if type(value) is type(literal) and value == literal: return value
    elif kind in ("json", "description", "text"):
        if kind in ("description", "text") and value is not None and not isinstance(value, (str, tuple, list, Mapping)): return _invalid()
        if kind == "text" and value is None: return _invalid()
        return _json(value)
    elif kind == "null":
        if value is None: return None
    elif kind == "bool":
        if type(value) is bool: return value
    elif kind == "str":
        if type(value) is str: return value
    elif kind in ("uint", "positive", "batch"):
        if kind == "batch" and value == "max": return value
        if type(value) is int and value >= (0 if kind == "uint" else 1): return value
    elif kind in ("number", "probability", "threshold"):
        if type(value) in (int, float) and math.isfinite(value) and (kind == "number" or (0 <= value <= 1 if kind == "probability" else 0 < value <= 1)): return value
        if kind == "threshold":
            if value is None: return None
            if type(value) is str and re.fullmatch(r"(?:0(?:\.[0-9]+)?|1(?:\.0+)?):(?:0(?:\.[0-9]+)?|1(?:\.0+)?)", value):
                low, high = map(float, value.split(":"))
                if low < high: return value
    elif kind == "cost":
        if type(value) is str and re.fullmatch(r"[0-9]+\.[0-9]{6}", value): return value
    return _invalid()


def _check(kind, value):
    if kind == "Usage" and not value: _invalid()
    if kind == "RankResult" and "members" in value:
        if not value["members"] or "question_name" not in value or value["question"]["verb"] != "decide" or value["answer"]["kind"] != "yes_no": _invalid()
    cut_only = ("ChooseSpec", "TagSpec", "ChooseMember", "TagMember", "RecognitionSpec", "RelationSpec", "TagResult", "FilterResult", "RecognizeQuestion", "RelateQuestion")
    if kind in cut_only:
        for key in ("threshold", "relation_threshold"):
            if key in value and (type(value[key]) not in (int, float) or not 0 < value[key] <= 1): _invalid()
    if kind == "ChooseResult" and value["threshold"] is not None and type(value["threshold"]) not in (int, float): _invalid()
    if kind == "Meta":
        if len(value["requests"]) != len(value["question_sources"]) or len(value["requests"]) != len(value["observations"]): _invalid()
        if ("question_sha256" in value) == ("questions_sha256" in value): _invalid()
        sources = value["question_sources"]
        if any(s["origin"] not in ("live", "cache", "replay") for s in sources): _invalid()
        if not sources:
            if value["origin"] is not None or value["cached"] or value["requests_sent"] or "answered_by" in value: _invalid()
        else:
            origins = [s["origin"] for s in sources]
            origin = "live" if "live" in origins else "replay" if "replay" in origins else "cache"
            if value["origin"] != origin or value["cached"] != (origin != "live"): _invalid()
            names = {s["answered_by"] for s in sources}
            if len(names) == 1:
                if value.get("answered_by") != next(iter(names)): _invalid()
            elif "answered_by" in value: _invalid()
        if value["failed_questions"] != sum("failure_id" in x for x in value["observations"]): _invalid()
    if kind in ("Span", "NameOdds", "PieceOdds", "Entity"):
        if value["end"] < value["start"]: _invalid()
        if kind == "Entity" and value["length"] != value["end"] - value["start"]: _invalid()
    if kind == "Position":
        if ("first" in value) != ("last" in value) or ("first" in value and ("file" not in value or value["last"] < value["first"])): _invalid()
    if kind in ("Entity", "Endpoint"):
        if ("first_line" in value) != ("last_line" in value): _invalid()
        if "first_line" in value and ("file" not in value or value["last_line"] < value["first_line"]): _invalid()
    if kind == "RankResult" and value["answer"]["kind"] not in ("yes_no", "score"): _invalid()
    if kind in ("DecideResult", "FilterResult") and value["threshold"] is None: _invalid()
    if kind.endswith("Result"):
        if (kind == "AnnotateResult") != ("questions_sha256" in value["meta"]): _invalid()


def to_json(value):
    """Project immutable carriers without creating IDs or default metadata."""
    if isinstance(value, Carrier):
        return {field.name[:-1] if field.name in ("true_", "false_") else field.name: to_json(getattr(value, field.name))
                for field in fields(value) if getattr(value, field.name) is not ABSENT}
    if isinstance(value, Mapping): return {k: to_json(v) for k, v in value.items()}
    if isinstance(value, tuple): return [to_json(x) for x in value]
    if isinstance(value, Identity): return str(value)
    return value


@dataclass(frozen=True, repr=False, kw_only=True)
class DecideSpec(Carrier):
    decide: QuestionText
    true_: Description | Absent = ABSENT
    false_: Description | Absent = ABSENT
    threshold: float | str | None | Absent = ABSENT
    model: str | Absent = ABSENT
    profile: str | Absent = ABSENT
    batch: int | Literal['max'] | Absent = ABSENT
    on: str | tuple[str, ...] | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ChooseSpec(Carrier):
    choose: QuestionText
    options: Labels
    threshold: float | Absent = ABSENT
    model: str | Absent = ABSENT
    profile: str | Absent = ABSENT
    batch: int | Literal['max'] | Absent = ABSENT
    on: str | tuple[str, ...] | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class TagSpec(Carrier):
    tag: QuestionText
    labels: Labels
    threshold: float | Absent = ABSENT
    model: str | Absent = ABSENT
    profile: str | Absent = ABSENT
    batch: int | Literal['max'] | Absent = ABSENT
    on: str | tuple[str, ...] | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ScoreSpec(Carrier):
    score: QuestionText
    levels: Labels
    model: str | Absent = ABSENT
    profile: str | Absent = ABSENT
    batch: int | Literal['max'] | Absent = ABSENT
    on: str | tuple[str, ...] | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class FindSpec(Carrier):
    find: QuestionText
    none: bool | Absent = ABSENT
    model: str | Absent = ABSENT
    profile: str | Absent = ABSENT
    on: str | tuple[str, ...] | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class QuestionFile(Carrier):
    path: str

@dataclass(frozen=True, repr=False, kw_only=True)
class QuestionSet(Carrier):
    version: Literal[1]
    questions: Mapping[str, AnnotationSpec]
    batch: int | Literal['max'] | Absent = ABSENT
    threshold: float | str | None | Absent = ABSENT
    profile: str | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RecognitionPlan(Carrier):
    instructions: str | Absent = ABSENT
    entity_definition: str | Absent = ABSENT
    kinds: Labels | Absent = ABSENT
    relations: tuple[PlanRule, ...] | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class PlanRule(Carrier):
    name: str
    source: str
    target: str
    reads: str | Absent = ABSENT
    either: bool | Absent = ABSENT
    single: bool | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RecognitionSpec(Carrier):
    version: Literal[1]
    recognize: RecognitionPlan
    threshold: float | Absent = ABSENT
    relation_threshold: float | Absent = ABSENT
    model: str | Absent = ABSENT
    profile: str | Absent = ABSENT
    on: str | tuple[str, ...] | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RelationPlan(Carrier):
    relations: tuple[PlanRule, ...]
    fields: RelateFields | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class RelationSpec(Carrier):
    version: Literal[1]
    relate: RelationPlan
    threshold: float | Absent = ABSENT
    model: str | Absent = ABSENT
    profile: str | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class Files(Carrier):
    paths: tuple[str, ...]
    unit: Literal['line', 'window', 'file']
    window: int | Absent = ABSENT
    media: Literal['text', 'image'] | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class TextInput(Carrier):
    text: JsonValue

@dataclass(frozen=True, repr=False, kw_only=True)
class RecordInput(Carrier):
    records: tuple[JsonValue, ...]
    context: JsonValue | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class CandidateInput(Carrier):
    units: tuple[JsonValue, ...]
    context: JsonValue | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ImageBytes(Carrier):
    data: bytes
    name: str | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ImageInput(Carrier):
    images: tuple[ImageBytes, ...]
    text: JsonValue | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class Controls(Carrier):
    batch: int | Literal['max'] | Absent = ABSENT
    context: JsonValue | Absent = ABSENT
    on: str | tuple[str, ...] | Absent = ABSENT
    threshold: float | str | None | Absent = ABSENT
    top: int | Absent = ABSENT
    none: bool | Absent = ABSENT
    model: str | Absent = ABSENT
    attempts: bool | Absent = ABSENT
    deadline_ms: int | Absent = ABSENT
Labels = tuple[str, ...] | Mapping[str, Description]
QuestionSpec = DecideSpec | ChooseSpec | TagSpec | ScoreSpec
RankSpec = DecideSpec | ScoreSpec | QuestionSet | QuestionFile
Selection = TextInput | RecordInput | CandidateInput | ImageInput | Files

_ALIASES.update({
    'Labels': ['[str]', '{description}'],
    'QuestionSpec': ['DecideSpec', 'ChooseSpec', 'TagSpec', 'ScoreSpec'],
    'RankSpec': ['DecideSpec', 'ScoreSpec', 'QuestionSet', 'QuestionFile'],
    'Selection': ['TextInput', 'RecordInput', 'CandidateInput', 'ImageInput', 'Files'],
})
_ENUMS.update({
    'unit': ['line', 'window', 'file'],
    'media': ['text', 'image'],
})

@dataclass(frozen=True, repr=False, kw_only=True)
class DecideMember(Carrier):
    decide: QuestionText
    true_: Description | Absent = ABSENT
    false_: Description | Absent = ABSENT
    threshold: float | str | None | Absent = ABSENT
    on: str | tuple[str, ...] | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ChooseMember(Carrier):
    choose: QuestionText
    options: Labels
    threshold: float | Absent = ABSENT
    on: str | tuple[str, ...] | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class TagMember(Carrier):
    tag: QuestionText
    labels: Labels
    threshold: float | Absent = ABSENT
    on: str | tuple[str, ...] | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class ScoreMember(Carrier):
    score: QuestionText
    levels: Labels
    on: str | tuple[str, ...] | Absent = ABSENT
    name: str | Absent = ABSENT
    wording_version: int | Absent = ABSENT
    item_schema: InputDeclaration | Absent = ABSENT
    context_schema: InputDeclaration | Absent = ABSENT

AnnotationSpec = DecideMember | ChooseMember | TagMember | ScoreMember

@dataclass(frozen=True, repr=False, kw_only=True)
class StringDeclaration(Carrier):
    type: Literal['string']

@dataclass(frozen=True, repr=False, kw_only=True)
class NumberDeclaration(Carrier):
    type: Literal['number']

@dataclass(frozen=True, repr=False, kw_only=True)
class BooleanDeclaration(Carrier):
    type: Literal['boolean']

@dataclass(frozen=True, repr=False, kw_only=True)
class ArrayDeclaration(Carrier):
    type: Literal['array']
    items: StringDeclaration

@dataclass(frozen=True, repr=False, kw_only=True)
class ObjectDeclaration(Carrier):
    type: Literal['object']
    properties: Mapping[str, PropertyDeclaration]
    required: tuple[str, ...] | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class PhysicalSource(Carrier):
    file: str
    first_line: int | Absent = ABSENT
    last_line: int | Absent = ABSENT

@dataclass(frozen=True, repr=False, kw_only=True)
class NativeImage(Carrier):
    media: Literal['image/png', 'image/jpeg']
    base64: str
    width: int
    height: int

@dataclass(frozen=True, repr=False, kw_only=True)
class Stopped(Carrier):
    cause: str
    retryable: bool
    status: int | Absent = ABSENT
    at: int | Absent = ABSENT

InputDeclaration = StringDeclaration | ObjectDeclaration
PropertyDeclaration = StringDeclaration | NumberDeclaration | BooleanDeclaration | ArrayDeclaration

_MODELS = {
    'Position': {'file?': 'str', 'first?': 'positive', 'last?': 'positive', 'images?': '[str]'},
    'Usage': {'input_tokens?': 'uint', 'output_tokens?': 'uint'},
    'ProfileWarning': {'tuned_for': 'str', 'running': 'str'},
    'BatchWarning': {'tuned_for': 'batch', 'running': 'batch'},
    'Attempt': {'ordinal': 'positive', 'request_sha256': 'Digest', 'wall_ms': 'uint', 'outcome': 'outcome', 'sdk_request_id': 'SdkRequestId', 'status?': 'uint', 'server_ms?': 'uint', 'request_id?': 'str'},
    'Facts': {'call_id': 'CallId', 'records': 'uint', 'requests_sent': 'uint', 'cache_answers': 'uint', 'seconds': 'number', 'input_tokens?': 'uint', 'output_tokens?': 'uint', 'model?': 'str', 'estimated_cost_usd?': 'cost', 'command_ms?': 'uint', 'attempts?': '[Attempt]', 'held_model_mismatch?': 'bool'},
    'QuestionSource': {'origin': 'origin', 'answered_by': 'str', 'batch_size?': 'positive'},
    'Observed': {'observation_id': 'ObservationId'},
    'FailedObservation': {'failure_id': 'FailureId'},
    'Meta': {'tool': 'str', 'url': 'str', 'model': 'str', 'requests_sent': 'uint', 'cached': 'bool', 'requests': '[Digest]', 'failed_questions': 'uint', 'origin': 'origin|null', 'question_sources': '[QuestionSource]', 'observations': '[Observation]', 'question_sha256?': 'Digest', 'questions_sha256?': 'Digest', 'answered_by?': 'str', 'usage?': 'Usage', 'profile_warning?': 'ProfileWarning', 'batch_setting?': 'batch', 'batch_warning?': 'BatchWarning', 'context_sha256?': 'Digest', 'attempts?': '[Attempt]'},
    'YesNo': {'kind': '=yes_no', 'probability': 'probability'},
    'Choice': {'kind': '=choice', 'pick': 'str', 'probabilities': '{probability}', 'confidence?': 'probability'},
    'Tags': {'kind': '=tag', 'probabilities': '{probability}'},
    'Score': {'kind': '=score', 'level': 'str', 'probabilities': '{probability}', 'confidence?': 'probability'},
    'FindAnswer': {'kind': '=find', 'pick': 'str', 'probabilities': '{probability}', 'confidence?': 'probability'},
    'DecideQuestion': {'verb': '=decide', 'text': 'text', 'true?': 'description', 'false?': 'description', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'ChooseQuestion': {'verb': '=choose', 'text': 'text', 'options': '[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'TagQuestion': {'verb': '=tag', 'text': 'text', 'labels': '[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'ScoreQuestion': {'verb': '=score', 'text': 'text', 'levels': '[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'FindQuestion': {'verb': '=find', 'text': 'text', 'none': 'bool', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'RelationRule': {'name': 'str', 'source': 'str', 'target': 'str', 'reads': 'str', 'either': 'bool', 'single?': 'bool'},
    'RelateFields': {'name': 'str', 'kind': 'str'},
    'RelateQuestion': {'verb': '=relate', 'fields': 'RelateFields|null', 'relations': '[RelationRule]', 'threshold': 'threshold', 'profile?': 'str', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'RecognizeQuestion': {'verb': '=recognize', 'kinds': '{description}', 'instructions?': 'str', 'entity_definition?': 'str', 'relations?': '[RelationRule]', 'threshold': 'threshold', 'relation_threshold': 'threshold', 'on?': 'str|[str]', 'profile?': 'str', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'Failure': {'kind': '=backend', 'cause': 'cause'},
    'FailedField': {'failed': 'Failure'},
    'Entity': {'text': 'text', 'start': 'uint', 'end': 'uint', 'length': 'uint', 'kind': 'str', 'strength': 'number', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive'},
    'Endpoint': {'name': 'str', 'kind': 'str', 'record?': 'json', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive'},
    'Edge': {'relation': 'str', 'source': 'Endpoint', 'target': 'Endpoint', 'probability': 'probability', 'either?': '=true'},
    'EntityEdge': {'relation': 'str', 'source': 'Entity', 'target': 'Entity', 'probability': 'probability', 'either?': '=true'},
    'Recognition': {'entities': '[Entity]', 'relations?': '[EntityEdge]'},
    'PieceOdds': {'start': 'uint', 'end': 'uint', 'tags': '{probability}'},
    'NameOdds': {'start': 'uint', 'end': 'uint', 'kinds': '{probability}|null', 'edges': '{probability}|null'},
    'Span': {'start': 'uint', 'end': 'uint'},
    'PairOdds': {'relation': 'str', 'source': 'Span', 'target': 'Span', 'probability': 'probability'},
    'RecognitionAnswer': {'pieces': '[PieceOdds]', 'names': '[NameOdds]', 'pairs': '[PairOdds]'},
    'AnnotationSuccess': {'answer_id': 'AnswerId', 'value': 'SuccessValue', 'question': 'AtomicQuestion', 'answer': 'AtomicAnswer', 'threshold': 'threshold', 'request': 'Digest'},
    'AnnotationFailure': {'failure_id': 'FailureId', 'question': 'AtomicQuestion', 'failure': 'Failure', 'request': 'Digest'},
    'RelationSuccess': {'relation': 'str', 'reads': 'str', 'method': 'str', 'direction': 'str', 'source': 'Endpoint', 'target': 'Endpoint|null', 'request': 'Digest', 'answer_id': 'AnswerId', 'probability': 'probability', 'accepted': 'bool', 'answer?': 'AtomicAnswer'},
    'RelationFailure': {'relation': 'str', 'reads': 'str', 'method': 'str', 'direction': 'str', 'source': 'Endpoint', 'target': 'Endpoint|null', 'request': 'Digest', 'failure_id': 'FailureId', 'failure': 'Failure'},
    'RelationAnswer': {'questions': '[RelationEntry]'},
    'DecideResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'meta': 'Meta', 'value': 'bool|null', 'question': 'DecideQuestion', 'answer': 'YesNo', 'threshold': 'threshold', 'input?': 'json', 'position?': 'Position', 'input_file?': 'str', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive', 'source?': 'PhysicalSource', 'images?': '[NativeImage]', 'index?': 'uint'},
    'ChooseResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'meta': 'Meta', 'value': 'str|null', 'question': 'ChooseQuestion', 'answer': 'Choice', 'threshold': 'threshold', 'input?': 'json', 'position?': 'Position', 'input_file?': 'str', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive', 'source?': 'PhysicalSource', 'images?': '[NativeImage]', 'index?': 'uint'},
    'TagResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'meta': 'Meta', 'value': '[str]', 'question': 'TagQuestion', 'answer': 'Tags', 'threshold': 'threshold', 'input?': 'json', 'position?': 'Position', 'input_file?': 'str', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive', 'source?': 'PhysicalSource', 'index?': 'uint'},
    'ScoreResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'meta': 'Meta', 'value': 'number', 'question': 'ScoreQuestion', 'answer': 'Score', 'threshold': 'null', 'input?': 'json', 'position?': 'Position', 'input_file?': 'str', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive', 'source?': 'PhysicalSource', 'images?': '[NativeImage]', 'index?': 'uint'},
    'FilterResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'meta': 'Meta', 'value': 'bool', 'input': 'json', 'question': 'DecideQuestion', 'answer': 'YesNo', 'threshold': 'threshold', 'position?': 'Position', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive', 'source?': 'PhysicalSource', 'index?': 'uint'},
    'RankMemberResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'value': 'positive', 'question': 'DecideQuestion', 'answer': 'YesNo', 'threshold': 'null', 'meta': 'Meta', 'source?': 'PhysicalSource'},
    'RankMember': {'name': 'str', 'result': 'RankMemberResult'},
    'RankResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'meta': 'Meta', 'value': 'positive', 'input': 'json', 'question': 'AtomicQuestion', 'answer': 'AtomicAnswer', 'threshold': 'null', 'question_name?': 'str', 'position?': 'Position', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive', 'source?': 'PhysicalSource', 'index?': 'uint', 'members?': '[RankMember]'},
    'FindResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'meta': 'Meta', 'value': 'json', 'question': 'FindQuestion', 'answer': 'FindAnswer', 'threshold': 'null', 'position?': 'Position', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive', 'index?': 'uint|null', 'candidates?': '[FindCandidate]'},
    'AnnotateResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'meta': 'Meta', 'input': 'json', 'value': '{AnnotatedValue}', 'answers': '{AnnotationEntry}', 'position?': 'Position', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive', 'index?': 'uint', 'source?': 'PhysicalSource'},
    'RecognizeResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'meta': 'Meta', 'value': 'Recognition', 'question': 'RecognizeQuestion', 'answer': 'RecognitionAnswer', 'input?': 'json', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive', 'index?': 'uint', 'source?': 'PhysicalSource'},
    'RelateResult': {'schema': '=thinkthen.result/2', 'answer_id': 'AnswerId', 'meta': 'Meta', 'value': '[Edge]', 'question': 'RelateQuestion', 'answer': 'RelationAnswer', 'file?': 'str', 'first_line?': 'positive', 'last_line?': 'positive', 'input?': 'json', 'index?': 'uint'},
    'CallError': {'kind': 'error_kind', 'message': 'str', 'retryable': 'bool', 'facts?': 'Facts', 'attempts?': '[Attempt]', 'stopped?': 'Stopped'},
    'DecideSpec': {'decide': 'text', 'true?': 'description', 'false?': 'description', 'threshold?': 'threshold', 'model?': 'str', 'profile?': 'str', 'batch?': 'batch', 'on?': 'str|[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'ChooseSpec': {'choose': 'text', 'options': 'Labels', 'threshold?': 'probability', 'model?': 'str', 'profile?': 'str', 'batch?': 'batch', 'on?': 'str|[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'TagSpec': {'tag': 'text', 'labels': 'Labels', 'threshold?': 'probability', 'model?': 'str', 'profile?': 'str', 'batch?': 'batch', 'on?': 'str|[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'ScoreSpec': {'score': 'text', 'levels': 'Labels', 'model?': 'str', 'profile?': 'str', 'batch?': 'batch', 'on?': 'str|[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'FindSpec': {'find': 'text', 'none?': 'bool', 'model?': 'str', 'profile?': 'str', 'on?': 'str|[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'QuestionFile': {'path': 'str'},
    'QuestionSet': {'version': 'one', 'questions': '{AnnotationSpec}', 'batch?': 'batch', 'threshold?': 'threshold', 'profile?': 'str'},
    'RecognitionPlan': {'kinds?': 'Labels', 'instructions?': 'str', 'entity_definition?': 'str', 'relations?': '[PlanRule]'},
    'PlanRule': {'name': 'str', 'source': 'str', 'target': 'str', 'reads?': 'str', 'either?': 'bool', 'single?': 'bool'},
    'RecognitionSpec': {'version': 'one', 'recognize': 'RecognitionPlan', 'threshold?': 'probability', 'relation_threshold?': 'probability', 'model?': 'str', 'profile?': 'str', 'on?': 'str|[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'RelationPlan': {'relations': '[PlanRule]', 'fields?': 'RelateFields'},
    'RelationSpec': {'version': 'one', 'relate': 'RelationPlan', 'threshold?': 'probability', 'model?': 'str', 'profile?': 'str', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'Files': {'paths': '[str]', 'unit': 'unit', 'window?': 'positive', 'media?': 'media'},
    'TextInput': {'text': 'json'},
    'RecordInput': {'records': '[json]', 'context?': 'json'},
    'CandidateInput': {'units': '[json]', 'context?': 'json'},
    'ImageBytes': {'data': 'bytes', 'name?': 'str'},
    'ImageInput': {'images': '[ImageBytes]', 'text?': 'json'},
    'Controls': {'batch?': 'batch', 'context?': 'json', 'on?': 'str|[str]', 'threshold?': 'threshold', 'top?': 'positive', 'none?': 'bool', 'model?': 'str', 'attempts?': 'bool', 'deadline_ms?': 'uint'},
    'DecideMember': {'decide': 'text', 'true?': 'description', 'false?': 'description', 'threshold?': 'threshold', 'on?': 'str|[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'ChooseMember': {'choose': 'text', 'options': 'Labels', 'threshold?': 'probability', 'on?': 'str|[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'TagMember': {'tag': 'text', 'labels': 'Labels', 'threshold?': 'probability', 'on?': 'str|[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'ScoreMember': {'score': 'text', 'levels': 'Labels', 'on?': 'str|[str]', 'name?': 'str', 'wording_version?': 'positive', 'item_schema?': 'InputDeclaration', 'context_schema?': 'InputDeclaration'},
    'StringDeclaration': {'type': '=string'},
    'NumberDeclaration': {'type': '=number'},
    'BooleanDeclaration': {'type': '=boolean'},
    'ArrayDeclaration': {'type': '=array', 'items': 'StringDeclaration'},
    'ObjectDeclaration': {'type': '=object', 'properties': '{PropertyDeclaration}', 'required?': '[str]'},
    'PhysicalSource': {'file': 'str', 'first_line?': 'positive', 'last_line?': 'positive'},
    'NativeImage': {'media': 'image_media', 'base64': 'str', 'width': 'positive', 'height': 'positive'},
    'Stopped': {'cause': 'stop_cause', 'retryable': 'bool', 'status?': 'uint', 'at?': 'uint'},
    'NativeInput': {'original': 'json', 'location?': 'PhysicalSource', 'images': '[NativeImage]'}
}

_ALIASES["AnnotationSpec"] = ["DecideMember", "ChooseMember", "TagMember", "ScoreMember"]

_ALIASES.update(InputDeclaration=['StringDeclaration','ObjectDeclaration'], PropertyDeclaration=['StringDeclaration','NumberDeclaration','BooleanDeclaration','ArrayDeclaration'])
_ENUMS.update(image_media=['image/png','image/jpeg'], stop_cause=['usage','local','no_key','transport','status','too_large','reply','backend','cancelled','deadline','defect'])

@dataclass(frozen=True, repr=False, kw_only=True)
class NativeInput(Carrier):
    original: JsonValue
    images: tuple[NativeImage, ...]
    location: PhysicalSource | Absent = ABSENT

_MODELS["NativeInput"] = {'original': 'json', 'location?': 'PhysicalSource', 'images': '[NativeImage]'}

_MODELS["RelateResult"]["input?"]="json"

@dataclass(frozen=True, repr=False, kw_only=True)
class FindCandidate(Carrier):
    index: int | None
    input: JsonValue
    probability: float
    source: PhysicalSource | Absent = ABSENT

_MODELS["FindCandidate"] = {"index": "uint|null", "input": "json", "probability": "probability", "source?": "PhysicalSource"}
