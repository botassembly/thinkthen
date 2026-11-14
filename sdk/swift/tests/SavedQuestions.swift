import Foundation
import ThinkThen

// Feature: concrete Swift questions preserve required domains across overloads.
func savedQuestions(_ client: Client) async throws {
  let record: Record = .text("evidence")
  let choice = ChooseQuestion("Topic?", choices: ["left", "right"])
  let tag = TagQuestion("Topics?", labels: ["only"])
  let score = ScoreQuestion("Urgent?", levels: ["low", "high"])
  let chosen: ChooseRow<String, Record> = try await client.choose(choice, record: record)
  let tagged: TagRow<String, Record> = try await client.tag(tag, record: record)
  let scored: ScoreRow<Record> = try await client.score(score, record: record)
  let choiceRows: ChooseRows<String, Record> = client.choose(
    choice, records: CollectionOfOne(record))
  let tagRows: TagRows<String, Record> = client.tag(tag, records: CollectionOfOne(record))
  let scoreRows: ScoreRows<Record> = client.score(score, records: CollectionOfOne(record))
  let choices = try await choiceRows.collect()
  let tags = try await tagRows.collect()
  let scores = try await scoreRows.collect()
  precondition(choices == [chosen] && tags == [tagged] && scores == [scored])
  let directChoice = try await client.choose("Topic?", record: record, choices: ["left", "right"])
  let directTag = try await client.tag("Topics?", record: record, labels: ["only"])
  let directScore = try await client.score("Urgent?", record: record, levels: ["low", "high"])
  precondition(directChoice == chosen && directTag == tagged && directScore == scored)
  let toolbox = Toolbox([
    "choice": choice, "rank": RankQuestion("Best?", levels: ["low", "high"]),
  ])
  let selected = try toolbox.choose("choice")
  precondition(selected == choice)
  let rank: RankQuestion = try toolbox.rank("rank")
  let ranked = try await client.rank(rank, records: [record]).collect()
  precondition(ranked.count == 1)
  do {
    _ = try toolbox.choose("rank")
    preconditionFailure("wrong toolbox function was selected")
  } catch is UsageError {}
  let children = AnnotationQuestions(["topic": choice])
  let annotated = try await client.annotate(children, record: record)
  let answer = try annotated.answer.choose("topic").answer
  precondition(["left", "right"].contains(answer))
  let plain = try await client.decide("Ready?", record: record)
  let named = try await client.decide("Ready?", record: record, name: "review")
  precondition(named.source.name == "review" && plain.source.name == nil)
  precondition(named.source.questionIdentity != plain.source.questionIdentity)
  precondition(named.isAnswered && plain.isAnswered)
  _ = try named.answer
  _ = try plain.answer
}
