import Foundation
import ThinkThen

// Feature: concrete Swift questions preserve required domains across overloads.
func savedQuestions(_ client: Client) async throws {
  let record: Record = .text("evidence")
  let choice = ChooseQuestion("Topic?", choices: ["left", "right"])
  let tag = TagQuestion("Topics?", labels: ["only"])
  let score = ScoreQuestion("Urgent?", levels: ["low", "high"])
  let chosen: ChooseRow = try await client.choose(choice, record: record)
  let tagged: TagRow = try await client.tag(tag, record: record)
  let scored: ScoreRow = try await client.score(score, record: record)
  let choiceRows: ChooseRows = client.choose(choice, records: CollectionOfOne(record))
  let tagRows: TagRows = client.tag(tag, records: CollectionOfOne(record))
  let scoreRows: ScoreRows = client.score(score, records: CollectionOfOne(record))
  let choices = try await choiceRows.collect()
  let tags = try await tagRows.collect()
  let scores = try await scoreRows.collect()
  precondition(choices == [chosen] && tags == [tagged] && scores == [scored])
  let directChoice = try await client.choose("Topic?", choices: ["left", "right"], record: record)
  let directTag = try await client.tag("Topics?", labels: ["only"], record: record)
  let directScore = try await client.score("Urgent?", levels: ["low", "high"], record: record)
  precondition(directChoice == chosen && directTag == tagged && directScore == scored)
  let toolbox = Toolbox([
    "choice": .choose(choice), "rank": .rank(RankQuestion("Best?", levels: ["low", "high"])),
  ])
  let selected = try toolbox.choose(named: "choice")
  precondition(selected == choice)
  let rank: RankQuestion = try toolbox.rank(named: "rank")
  let ranked = try await client.rank(rank, records: [record]).collect()
  precondition(ranked.count == 1)
  do {
    _ = try toolbox.choose(named: "rank")
    preconditionFailure("wrong toolbox function was selected")
  } catch {
    precondition(error == ThinkThenError.toolboxSelection(name: "rank", expected: .choose))
  }
  let children = AnnotationQuestions(["topic": .choose(choice)])
  let annotated = try await client.annotate(children, record: record)
  guard case .choose(.answered(let answer, _, _)) = annotated.answer?["topic"] else {
    preconditionFailure("typed annotation child was lost")
  }
  precondition(["left", "right"].contains(answer))
  let plain = try await client.decide("Ready?", record: record)
  let named = try await client.decide("Ready?", record: record, name: "review")
  precondition(named.source.name == "review" && plain.source.name == nil)
  precondition(named.source.questionIdentity != plain.source.questionIdentity)
  precondition(named.answer == plain.answer)
}
