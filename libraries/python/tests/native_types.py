"""The installed public complete API's known fields compile as concrete types."""
from thinkthen import complete as c
from typing import assert_type
import thinkthen as tt

def retained_judge(judge: tt.Judge[bool | None], files: tt.FileSelection) -> None:
    assert_type(judge('note'), tt.Call[bool | None])
    assert_type(judge(files), tt.Call[list[tt.Located[bool | None]]])
    stream = judge(iter(['note']))
    assert_type(stream, tt.Stream[bool | None])
    assert_type(iter(stream), tt.Stream[bool | None])
    with stream as entered:
        assert_type(entered, tt.Stream[bool | None])

def known(engine:c.Engine, question:c.QuestionSource, inputs:c.Records|c.Files)->None:
    decision=engine.decide(question,inputs)
    yes:float=decision.results[0].answer.probability
    images:tuple[c.NativeImage,...]|c.Absent=decision.results[0].images
    choose=engine.choose(question,inputs)
    probability:float=choose.results[0].answer.probabilities['blue']
    tags=engine.tag(question,inputs).results[0].answer.probabilities
    score:float=engine.score(question,inputs).results[0].value
    accepted:bool=engine.filter(question,inputs).results[0].value
    ranked=engine.rank(question,inputs).results[0]
    place:int=ranked.value
    members:tuple[c.RankMember,...]|c.Absent=ranked.members
    if not isinstance(members,c.Absent):
        for member in members:
            result:c.RankMemberResult=member.result
            member_id:c.AnswerId=result.answer_id
            position:int=result.value
            member_probability:float=result.answer.probability
            member_question:c.DecideQuestion=result.question
            usage:c.Usage|c.Absent=result.meta.usage
            source:c.PhysicalSource|c.Absent=result.source
    found_call=engine.find(question,inputs)
    found:c.JsonValue=found_call.results[0].value
    selected:int|None|c.Absent=found_call.results[0].index
    candidates:tuple[c.FindCandidate,...]|c.Absent=found_call.results[0].candidates
    annotated=engine.annotate(question,inputs).results[0].answers
    entities=engine.recognize(question,inputs).results[0].value.entities
    end:int=entities[0].end
    location_end:int|c.Absent=entities[0].last_line
    edges=engine.relate(question,inputs).results[0].value
    for edge in edges:
        location:str|c.Absent=edge.source.file
    for row in engine.decide_batch(question,inputs):
        ordinal:int=row.ordinal
        image_width:int=row.input.images[0].width
        actual:float=row.result.answer.probability
    call:c.CallId=decision.facts.call_id
    answer:c.AnswerId=decision.results[0].answer_id
    requests:int=decision.facts.requests_sent
    request_bytes:int|c.Absent=decision.facts.largest_request_bytes
    estimated_tokens:int|None|c.Absent=decision.facts.largest_request_estimated_input_tokens
    estimate_method:str|c.Absent=decision.facts.token_estimate_method
    persistence:c.PersistenceObservation|c.Absent=decision.facts.usage_persistence
    if not isinstance(persistence,c.Absent):
        state:str=persistence.state
        observed_at:str=persistence.observed_at
        advice:str|c.Absent=persistence.advice
    author:str|c.Absent=decision.results[0].question.name

from thinkthen._native_results import NativeSessionPacketDecideRow, NativeSessionPacketTerminal

def generated_known(row: NativeSessionPacketDecideRow, terminal: NativeSessionPacketTerminal) -> None:
    schema: str = row.value.schema
    identifier: str = row.value.answer_id
    snapshots: list[object] = list(row.value.meta.observations)
    requests: int = terminal.facts.requests_sent
    failed: str = terminal.failure.error.kind
    present: bool = 'failure' in terminal
    raw: dict[str, object] = terminal.to_dict()
