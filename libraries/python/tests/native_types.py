"""The installed public complete API's known fields compile as concrete types."""
from thinkthen import complete as c

def known(engine:c.Engine, question:c.QuestionSource, inputs:c.Records|c.Files)->None:
    decision=engine.decide(question,inputs)
    yes:float=decision.results[0].answer.probability
    choose=engine.choose(question,inputs)
    probability:float=choose.results[0].answer.probabilities['blue']
    tags=engine.tag(question,inputs).results[0].answer.probabilities
    score:float=engine.score(question,inputs).results[0].value
    accepted:bool=engine.filter(question,inputs).results[0].value
    place:int=engine.rank(question,inputs).results[0].value
    found:c.JsonValue=engine.find(question,inputs).results[0].value
    annotated=engine.annotate(question,inputs).results[0].answers
    entities=engine.recognize(question,inputs).results[0].value.entities
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
    author:str|c.Absent=decision.results[0].question.name
