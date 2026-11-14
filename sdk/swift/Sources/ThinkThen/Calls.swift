import Foundation

extension Client {
  func evaluate<Q: SavedQuestion & FunctionShape, T: Encodable & Sendable>(
    _ question: Q, input: any RecordSource, item: T.Type, name: String?
  ) -> Rows<Row<Q, T>> {
    rows(question, source: input, name: name) { wire, retained in
      guard let original = retained as? T else {
        throw CallError.contract(message: "Result lost its original record")
      }
      return try wire.decode(Q.self, item: original)
    }
  }
}
