"""The installed public complete API's known fields compile as concrete types."""
from thinkthen import complete as c

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
    author:str|c.Absent=decision.results[0].question.name
