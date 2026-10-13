import Foundation
import ThinkThen

@main enum Example {
    static func definition(_ value: InputRequestDefinition) -> InputRequestQuestion {
        .definition(InputRequestQuestionDefinition(value: value))
    }
    static func main() async throws {
        let client = try Client()
        defer { client.close() }
        let question = InputRequestQuestion.text("Is it?")
        let input = InputRequestInput.text("text")
        let records = InputRequestInput.records(InputRequestInputRecords(items: [.text("alpha"), .text("beta")]))
        print(try await client.decideAnswer(question, input: input) as Any)
        let choices = InputOrderedMap<InputAuthoredDescription>([.init("b", .alternative3(.null)), .init("a", .alternative3(.null))])
        print(try await client.chooseAnswer(definition(.alternative1(InputRequestDefinitionAuthoredChoose(choose: .alternative0("Which?"), options: .value(.alternative1(choices))))), input: input) as Any)
        print(try await client.tagAnswer(definition(.alternative2(InputRequestDefinitionAuthoredTag(labels: .value(.alternative0(["a", "b"])), tag: .alternative0("Which?")))), input: input))
        print(try await client.scoreAnswer(definition(.alternative3(InputRequestDefinitionAuthoredScore(levels: .value(.alternative0(["low", "high"])), score: .alternative0("How?")))), input: input))
        print(try await client.filterAnswer(question, input: records))
        print(try await client.rankAnswer(question, input: records))
        print(try await client.findAnswer(.text("Which?"), input: .units(InputRequestInputUnits(items: [.text("alpha"), .text("beta")]))))
        let questions = InputOrderedMap<InputAnnotateQuestion>([.init("ok", .alternative0(InputAnnotateQuestionDecide(decide: .alternative0("Is it?"))))])
        print(try await client.annotateAnswer(definition(.alternative7(InputRequestDefinitionAnnotateDefinition(questions: questions))), input: .records(InputRequestInputRecords(items: [.text("text")]))))
        let kinds = InputOrderedMap<InputAuthoredDescription>([.init("person", .alternative3(.null))])
        print(try await client.recognizeAnswer(definition(.alternative6(InputRequestDefinitionRecognizeDefinition(recognize: InputRecognizeQuestion(kinds: .value(kinds))))), input: .text("Ana Lima")))
        let relations = InputRelateQuestion(relations: [InputAuthoredRelation(name: "knows", source: .value("person"), target: .value("person"))])
        let entities = InputRequestInput.entities(InputRequestInputEntities(items: [
            InputRequestItem(original: .value(.json(InputRequestOriginalJson(value: .object(["name": .string("Ana"), "kind": .string("person")]))))),
            InputRequestItem(original: .value(.json(InputRequestOriginalJson(value: .object(["name": .string("Bob"), "kind": .string("person")])))))
        ]))
        let relation = try await client.relateAnswer(definition(.alternative4(InputRequestDefinitionAuthoredRelate(relate: relations))), input: entities)
        print(relation)
        if let edge = relation.value.first { print(try edge.sourceName().value as Any, try edge.targetName().value as Any) }
    }
}
