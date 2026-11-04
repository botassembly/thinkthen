import ThinkThen

// Feature: future call, record and annotation failures retain engine diagnostics.
@main struct MinorError {
  static func operation() throws {
    do {
      _ = try Client(Settings(backend: .stub))
      preconditionFailure("Future error was ignored")
    } catch {
      guard let failure = (error as? ThinkThenError)?.failure else { throw error }
      precondition(failure.code.rawValue == "future.code")
      let message = failure.message
      precondition(message == CommandLine.arguments[1])
    }
  }
  static func main() async throws {
    if CommandLine.arguments.last == "row" {
      let client = try Client(Settings(backend: .stub))
      let row = try await client.decide("Ready?", record: .missing)
      guard case .failed(let failure) = row.outcome else {
        preconditionFailure("Future record failure was ignored")
      }
      precondition(failure.code.rawValue == "future.code")
      precondition(failure.message == CommandLine.arguments[1])
    } else if CommandLine.arguments.last == "child" {
      let client = try Client(Settings(backend: .stub))
      let questions = AnnotationQuestions(["future": .decide(DecideQuestion("Ready?"))])
      let row = try await client.annotate(questions, record: .text("record"))
      guard let answer = row.answer, case .decide(.failed(let failure)) = answer["future"] else {
        preconditionFailure("Future child failure was ignored")
      }
      precondition(
        failure.code.rawValue == "future.problem" && failure.message == CommandLine.arguments[1])
    } else {
      try operation()
    }
  }
}
