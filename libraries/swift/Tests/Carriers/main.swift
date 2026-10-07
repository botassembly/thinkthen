import Foundation
let question = QuestionInput.asked(Question.asked(.choose, .text("α\r\nquestion")))
let choices = [Choice(name: "second", description: try .json("{\"detail\":false}"), weight: 0), Choice(name: "first", description: .text("description"), weight: nil)]
let image = ImageInput(media: .png, bytes: [0,255,1], filename: "one.png")
let record = RecordInput(original: nil, context: .text("context"), options: choices, images: [image,image])
let source = InputSource.records([record,record])
let builders: [(QuestionInput,InputSource,CallControls) -> CompleteRequest] = [
    Requests.decide, Requests.choose, Requests.tag, Requests.score, Requests.filter,
    Requests.rank, Requests.find, Requests.annotate, Requests.recognize, Requests.relate]
let functions: [Function] = [.decide,.choose,.tag,.score,.filter,.rank,.find,.annotate,.recognize,.relate]
for (index, builder) in builders.enumerated() {
    let request = builder(question, source, CallControls(context: nil, batch: nil, batchMax: false, attempts: true))
    let encoded = try JSONEncoder().encode(request)
    let copy = try JSONDecoder().decode(CompleteRequest.self, from: encoded)
    precondition(copy.function == functions[index] && copy.source.records?.count == 2)
    precondition(copy.source.records?[0].images[1].bytes == [0,255,1])
    precondition(copy.source.records?[0].original == nil && copy.source.records?[0].options[0].name == "second")
}
var original = source
let prepared = Requests.choose(question, original)
original.records?[0].images[0].bytes[0] = 9
precondition(prepared.source.records?[0].images[0].bytes[0] == 0)
let files = Requests.find(.questionFile("explicit.json"), .files(FileSource(paths: ["a","a"], unit: .window, window: 2)))
precondition(files.question.file == "explicit.json" && files.source.files?.paths == ["a","a"])
let span = NameSpan(start: 1, end: 2, kinds: nil, edges: [])
let copied = try JSONDecoder().decode(NameSpan.self, from: JSONEncoder().encode(span))
precondition(copied.kinds == nil && copied.edges?.isEmpty == true)
let facts = CallFacts(callId: try CallId(String(repeating: "a", count: 64)), cacheAnswers: 0, estimatedCostUsd: "0.000001", inputTokens: 0, model: nil, outputTokens: nil, records: 2, requestsSent: 0, seconds: 0, commandMs: nil)
let exact = try JSONDecoder().decode(CallFacts.self, from: JSONEncoder().encode(facts))
precondition(exact.estimatedCostUsd == "0.000001" && exact.inputTokens == 0 && exact.outputTokens == nil)
for value in [String(repeating: "A", count: 64), String(repeating: "a", count: 63), String(repeating: "a", count: 65)] {
    do { _ = try AnswerId(value); preconditionFailure("invalid identity accepted") } catch DescriptorError.invalidIdentity {}
    do { _ = try JSONDecoder().decode(AnswerId.self, from: JSONEncoder().encode(value)); preconditionFailure("invalid decoded identity accepted") } catch DescriptorError.invalidIdentity {}
}
let maximum = TokenUsage(inputTokens: UInt64.max, outputTokens: 0)
let maximumCopy = try JSONDecoder().decode(TokenUsage.self, from: JSONEncoder().encode(maximum))
precondition(maximumCopy.inputTokens == UInt64.max)
let answer = AtomicAnswer(kind: .choice, probability: nil, pick: "second", level: nil, probabilities: [Probability(name: "second", value: 0.7), Probability(name: "first", value: 0.3)], confidence: 0)
let ordered = try JSONDecoder().decode(AtomicAnswer.self, from: JSONEncoder().encode(answer))
precondition(ordered.probabilities.map(\.name) == ["second","first"] && ordered.confidence == 0)
let observed = try CompleteReaders.facts(#"{"call_id":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cache_answers":0,"estimated_cost_usd":"0.000001","input_tokens":0,"records":2,"requests_sent":0,"seconds":0}"#)
precondition(observed.inputTokens == 0 && observed.outputTokens == nil && observed.estimatedCostUsd == "0.000001")
do { _ = try CompleteReaders.facts("{\"cache_answers\":0}"); preconditionFailure("legacy facts accepted") } catch is DecodingError {}
