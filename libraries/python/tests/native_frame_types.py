"""Complete dataframe callers access the same concrete native carriers."""
from thinkthen import frames, complete as c


def known(engine: frames.Engine, question: c.QuestionSource, series: object) -> None:
    decision: frames.FrameCompleted[c.DecideResult] = engine.decide(question, series)
    native: c.Completed[c.DecideResult] = decision.native
    yes: float = decision.results[0].answer.probability
    images: tuple[c.NativeImage, ...] | c.Absent = decision.results[0].images
    probability: float = engine.choose(question, series).results[0].answer.probabilities['blue']
    tags = engine.tag(question, series).results[0].answer.probabilities
    score: float = engine.score(question, series).results[0].value
    accepted: bool = engine.filter(question, series).results[0].value
    place: int = engine.rank(question, series).results[0].value
    selected: int | None | c.Absent = engine.find(question, series).results[0].index
    candidates = engine.find(question, series).results[0].candidates
    members = engine.annotate(question, series).results[0].answers
    entities = engine.recognize(question, series).results[0].value.entities
    edges = engine.relate(question, series).results[0].value
    position: int | None = decision.positions[0]
    call: c.CallId = decision.facts.call_id
    answer: c.AnswerId = decision.results[0].answer_id
    for row in engine.decide_batch(question, series):
        width: int = row.input.images[0].width
        actual: float = row.result.answer.probability
