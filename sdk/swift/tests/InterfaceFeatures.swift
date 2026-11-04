import Foundation
import ThinkThen

// Feature: the ordinary filter returns original records and never loses failures.
func settledFilter(_ client: Client) async throws {
  let texts = (1...40).map { "Support message \($0)" }
  let valid = texts.map(Record.text)
  let question = FilterQuestion("Does this need a refund?")
  let full = try await client.filterResults(question, records: valid).collect()
  let expected = full.filter(\.kept).map(\.item)
  precondition(!expected.isEmpty && expected.count < valid.count)
  let kept = try await client.filter(question, records: valid).collect()
  precondition(kept == expected)
  let keptTexts: [String] = try await client.filter("Does this need a refund?", records: texts)
    .collect()
  precondition(keptTexts.map(Record.text) == expected)
  let inputs = [Record.missing] + valid + [.text("")]
  var delivered: [Record] = []
  do {
    for try await record in client.filter(question, records: inputs) { delivered.append(record) }
    preconditionFailure("filter silently dropped failed records")
  } catch let failure as RowFailure {
    precondition(failure.code == .recordMissing)
    precondition(failure.failedRows.map(\.position) == [1, inputs.count])
    precondition(failure.failedRows.last?.failure?.code == .recordEmpty)
  }
  let all = try await client.filterResults(question, records: inputs).collect()
  precondition(delivered == all.filter(\.kept).map(\.item))
  let visible = try await client.filter(question, records: inputs, failures: .row).collect()
  precondition(visible == all.filter { $0.kept || $0.row.isFailed })
  let uncertainQuestion = FilterQuestion(
    "Does this need a refund?", options: .init(threshold: .band(low: 0.2, high: 0.8)))
  let uncertain = try await client.filterResults(uncertainQuestion, records: valid).collect()
  precondition(uncertain.contains { $0.row.isUnsure })
  let certainKept = try await client.filter(uncertainQuestion, records: valid).collect()
  precondition(certainKept == uncertain.filter(\.kept).map(\.item))
  let entered = Signal()
  let released = Signal()
  let returned = Signal()
  let pending = client.filter(
    question, records: StubbornSource(entered: entered, released: released, returned: returned))
  let active = Task { try await pending.makeAsyncIterator().next() }
  await entered.wait()
  pending.cancel()
  do {
    _ = try await active.value
    preconditionFailure("filter cancellation did not stop the pending read")
  } catch { guard error is CancellationError else { throw error } }
  await released.fire()
  await returned.wait()

}

// Feature: ranked results retain their input records after sorting and truncation.
func settledRank(_ client: Client) async throws {
  try await checkRankOriginalKinds(client)
  let inputs = (1...12).map { Record.text("Support message \($0)") }
  let ranked = try await client.rank(
    "How urgent?", levels: ["routine", "urgent"], records: inputs, options: .init(top: 3)
  )
  .collect()
  precondition(ranked.count == 3)
  precondition(ranked.map(\.position) != [1, 2, 3])
  for result in ranked { precondition(result.item == inputs[result.position - 1]) }
  let ranks = try ranked.map { try $0.get() }
  precondition(ranks == [1, 2, 3])
  let mixed = try await client.rank(
    "How urgent?", levels: ["routine", "urgent"], records: inputs + [.missing],
    options: .init(top: 3)
  ).collect()
  precondition(mixed.count == 4 && mixed.last?.item == .missing)
  precondition(mixed.last?.failure?.code == .recordMissing)

}

// Feature: typed questions carry meanings into the engine and saved toolboxes.
func settledQuestionMeanings(_ client: Client) async throws {
  enum Queue: String, CaseIterable { case billing, shipping }
  let fromCases = ChooseQuestion("Which queue?", choices: Queue.self)
  precondition(fromCases.choices == ["billing", "shipping"])
  let described = ChooseQuestion(
    "Which queue?", choices: [Choice("billing", description: "Invoices"), "shipping"])
  let restored = try JSONDecoder().decode(
    ChooseQuestion.self, from: JSONEncoder().encode(described))
  precondition(restored == described)
  let selected = try await client.choose(described, record: "Invoice correction")
  let queue = try selected.get()
  precondition(["billing", "shipping"].contains(queue))
  let meaning = DecideQuestion("Needs help?", yesMeans: "Needs a person", noMeans: "Resolved")
  let undecorated = DecideQuestion("Needs help?")
  let first = try await client.decide(meaning, record: "Invoice correction")
  let second = try await client.decide(undecorated, record: "Invoice correction")
  precondition(first.source.questionIdentity != second.source.questionIdentity)
  let empty = client.decide("Ready?", records: [Record]())
  let noRows = try await empty.collect()
  precondition(noRows.isEmpty)
  do {
    _ = try await client.decide("Ready?", record: "  \n")
    preconditionFailure("single blank input returned an answer")
  } catch let failure as ThinkThenError {
    precondition(failure.failure?.code == .recordEmpty)
  }
}

// Feature: every many-record entry point accepts ordinary text collections.
func settledTextCollections(_ client: Client) async throws {
  checkTextSequenceLaziness(client)
  let texts = ["Morgan requests a refund.", "Alex works for Example Group."]
  let tags = try await client.tag("Which topic?", labels: ["billing"], records: texts).collect()
  let names = try await client.recognize(kinds: ["person"], records: texts).collect()
  let relations = try await client.relate(
    rules: [.init(source: "person", target: "organization", relation: "works_for")], records: texts
  )
  .collect()
  let questions = AnnotationQuestions(["refund": .decide(DecideQuestion("Needs a refund?"))])
  let annotations = try await client.annotate(questions, records: texts).collect()
  precondition([tags.count, names.count, relations.count, annotations.count] == [2, 2, 2, 2])
  for annotation in annotations {
    let answers = try annotation.get()
    let _: Bool = try answers.get("refund")
  }
  let stream = AsyncStream<String> { continuation in
    for text in texts { continuation.yield(text) }
    continuation.finish()
  }
  let streamed = try await client.annotate(questions, records: stream).collect()
  precondition(streamed == annotations)
}
