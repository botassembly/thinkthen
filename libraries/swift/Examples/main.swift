import Foundation
import ThinkThen

@main enum Example {
    static func definition(_ value: Definition) -> Question {
        .definition(QuestionDefinition(value: value))
    }
    static func main() async throws {
        let client = try Client()
        defer { client.close() }
        let question = Question.text("Is it?")
        let input = Input.text("text")
        let records = Input.records(InputRecords(items: [.text("alpha"), .text("beta")]))
        print(try await client.decideAnswer(question, input: input) as Any)
        let choices = OrderedMap<AuthoredDescription>([.init("billing", .null), .init("shipping", .null), .init("account", .null), .init("fraud", .null), .init("returns", .null)])
        print(try await client.chooseAnswer(definition(.choose(AuthoredChoose(choose: .string("Which?"), options: .value(.object(choices))))), input: input) as Any)
        print(try await client.tagAnswer(definition(.tag(AuthoredTag(labels: .value(.list(["a", "b"])), tag: .string("Which?")))), input: input))
        print(try await client.scoreAnswer(definition(.score(AuthoredScore(levels: .value(.list(["low", "high"])), score: .string("How?")))), input: input))
        let filtered: [JSONValue] = try await client.filterAnswer(question, input: records)
        print(filtered)
        let ranked: [UInt64] = try await client.rankAnswer(question, input: records)
        print(ranked)
        print(try await client.findAnswer(.text("Which?"), input: .units(InputUnits(items: [.text("alpha"), .text("beta")]))))
        let questions = OrderedMap<AnnotateQuestion>([.init("billing", .decide(AnnotateQuestionDecide(decide: .string("Is billing?")))), .init("shipping", .decide(AnnotateQuestionDecide(decide: .string("Is shipping?")))), .init("account", .decide(AnnotateQuestionDecide(decide: .string("Is account?")))), .init("fraud", .decide(AnnotateQuestionDecide(decide: .string("Is fraud?")))), .init("returns", .decide(AnnotateQuestionDecide(decide: .string("Is returns?"))))])
        print(try await client.annotateAnswer(definition(.questions(AnnotateDefinition(questions: questions))), input: .records(InputRecords(items: [.text("text")]))))
        let kinds = OrderedMap<AuthoredDescription>([.init("person", .null), .init("organization", .null), .init("location", .null), .init("product", .null), .init("event", .null)])
        print(try await client.recognizeAnswer(definition(.recognize(RecognizeDefinition(recognize: RecognizeQuestion(kinds: .value(kinds))))), input: .text("Ana Lima")))
        let relations = RelateQuestion(relations: [AuthoredRelation(name: "knows", source: .value("person"), target: .value("person"))])
        let entities = Input.entities(InputEntities(items: [
            Item(original: .value(.json(OriginalJson(value: .object(["name": .string("Ana"), "kind": .string("person")]))))),
            Item(original: .value(.json(OriginalJson(value: .object(["name": .string("Bob"), "kind": .string("person")])))))
        ]))
        let relation = try await client.relateAnswer(definition(.relate(AuthoredRelate(relate: relations))), input: entities)
        print(relation)
        if let edge = relation.value.first { print(try edge.sourceName().value as Any, try edge.targetName().value as Any) }
    }
}
