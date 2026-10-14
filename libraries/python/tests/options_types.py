"""Installed caller controls retain native request types on every call family."""
from pathlib import Path
from typing import Literal, assert_type
import thinkthen as tt
from thinkthen._named_generated import RecognitionSeedSpan, RecognitionExampleText, RecognitionExampleEntity, RecognitionStageContext

def controls() -> None:
    span: RecognitionSeedSpan = {"start": 0, "end": 3, "kind": "person"}
    entity: RecognitionExampleEntity = {"start": 0, "end": 3, "kind": "person"}
    example: RecognitionExampleText = {"text": "Ada", "entities": [entity]}
    stages: RecognitionStageContext = {"boundary": "", "kind_edge": "People", "relation": "Links"}
    engine = tt.Engine(cache=False, timeout=500, record=Path("recording"),
                       max_requests_total=0, max_estimated_input_tokens_total=5000,
                       backend=None, model=None, batch="max")
    done: tt.Result[object] = engine.decide("Late?", "text", attempts=True, batch=2,
                                          deadline_ms=-1, details=True, model="local",
                                          context="", threshold=.5, max_requests_total=0)
    engine.choose("Team?", "text", options=["a", "b"], options_field="/options")
    engine.choose("Team?", "text", options=Literal["billing", "shipping"])
    engine.tag("Kinds?", "text", labels=Literal["billing", "shipping"])
    engine.score("Urgency?", "text", levels=["low", "high"])
    engine.filter("Late?", ["text"], field=["/body"], files_only=True)
    engine.rank("Late?", ["text"], top=2)
    engine.find("Best?", ["text"], none=True)
    engine.annotate({}, ["text"], context_field="/context")
    engine.recognize("Ada", kinds=["person"], seed_spans=[span], examples=[example],
                     stage_context=stages, mode="boundary_only", snippet_pieces=2,
                     examples_field="/examples", seed_spans_field="/seeds", relation_threshold=.5)
    engine.relate([("Ada", "person")], relations={"knows": ("person", "person")})
    tt.decide("Late?", "text", token=tt.CancelToken())

async def awaiting(engine: tt.Engine) -> None:
    done: tt.Result[object] = await engine.asyncio.decide("Late?", "text", batch=1)
    await engine.asyncio.recognize("Ada", mode="boundary_only", kinds=["person"])


def preview(engine: tt.Engine) -> None:
    plan: tt.Plan = engine.plan("decide", "Late?", ["note"], batch=1)
    records: int = plan.records
    largest: int = plan.largest_request_bytes
    estimate: int = plan.estimated_input_tokens.lower
    text: str | None = plan.first_body_utf8


def answers(engine: tt.Engine) -> None:
    from thinkthen._native_results import NativeAtomicDouble, NativeRecognize, NativeRecognition
    assert_type(engine.score("Urgency?", "note", levels=["low", "high"]).value,
                float | list[float] | NativeAtomicDouble | list[NativeAtomicDouble])
    assert_type(engine.recognize("Ada", kinds=["person"]).value,
                NativeRecognize | list[NativeRecognize] | NativeRecognition | list[NativeRecognition])

async def async_answers(engine: tt.Engine) -> None:
    from thinkthen._native_results import NativeAtomicDouble
    score = await engine.asyncio.score("Urgency?", "note", levels=["low", "high"])
    assert_type(score.value, float | list[float] | NativeAtomicDouble | list[NativeAtomicDouble])
