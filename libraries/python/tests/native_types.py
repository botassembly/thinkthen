"""Installed callers access generated known fields without a hand-copied reader."""
import thinkthen as tt
from thinkthen._native_results import NativeSessionPacketDecideRow, NativeSessionPacketTerminal, NativeAnswerYesNo, NativeAnswerChoice, NativeAnswerTag

def known(engine: tt.Engine, question: object, source: object) -> None:
    decision=engine.decide(question,source)
    answer=decision.results[0].answer
    assert isinstance(answer,NativeAnswerYesNo)
    probability: float=answer.probability
    identity: str=decision.results[0].answer_id
    original: object=decision.results[0].input
    chosen=engine.choose(question,source).results[0].answer
    assert isinstance(chosen,NativeAnswerChoice)
    choice: float=chosen.probabilities['blue']
    tagged=engine.tag(question,source).results[0].answer
    assert isinstance(tagged,NativeAnswerTag)
    tags: dict[str,float]=tagged.probabilities
    score: float=engine.score(question,source).results[0].value
    kept: bool=engine.filter(question,source).results[0].value
    position: int=engine.rank(question,source).results[0].value
    selected: object=engine.find(question,source).results[0].value
    fields=engine.annotate(question,source).results[0].answers
    names=engine.recognize(source,question).results[0].value
    edges=engine.relate(source,question).results[0].value

def generated_known(row: NativeSessionPacketDecideRow, terminal: NativeSessionPacketTerminal) -> None:
    schema: str=row.value.schema
    identity: str=row.value.answer_id
    requests: int=terminal.facts.requests_sent
    failure: str=terminal.failure.error.kind
    present: bool='failure' in terminal
    raw: dict[str, object]=row.value.to_dict()
