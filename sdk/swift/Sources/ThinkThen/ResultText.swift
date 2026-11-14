import Foundation

extension JSONValue {
  var object: [String: JSONValue] {
    if case .object(let value) = self { return value }
    return [:]
  }
  var array: [JSONValue] {
    if case .array(let value) = self { return value }
    return []
  }
  var string: String? {
    if case .string(let value) = self { return value }
    return nil
  }
  var int: Int? {
    switch self {
    case .integer(let x): Int(exactly: x)
    case .unsigned(let x): Int(exactly: x)
    default: nil
    }
  }
}
func decodeJSON<T: Decodable>(_ type: T.Type, _ value: JSONValue?, _ decoder: JSONDecoder) throws
  -> T
{
  guard let value else { throw CallError.contract(message: "Missing native result field") }
  return try decoder.decode(type, from: JSONEncoder().encode(value))
}
func utf8Before(_ left: String, _ right: String) -> Bool {
  left.utf8.lexicographicallyPrecedes(right.utf8)
}
func expandedEntry(_ entry: JSONValue, parent: JSONValue) -> JSONValue {
  var own = parent.object
  for key in ["answer", "probability", "error", "detail"] { own.removeValue(forKey: key) }
  for (key, value) in entry.object { own[key] = value }
  return .object(own)
}
func firstFailedResult(_ value: JSONValue) -> JSONValue? {
  let fields = value.object
  if fields["status"]?.string == "failed" { return value }
  if fields["function"]?.string == "annotate" {
    let entries = fields["answer"]?.object ?? [:]
    for key in entries.keys.sorted(by: utf8Before) {
      if let entry = entries[key],
        let failed = firstFailedResult(expandedEntry(entry, parent: value))
      {
        return failed
      }
    }
  }
  return nil
}
func printLeaf(_ value: String) -> String {
  value.replacingOccurrences(of: "\r", with: " ").replacingOccurrences(of: "\n", with: " ")
    .replacingOccurrences(of: "\t", with: " ")
}
func printNumber(_ value: Double) -> String {
  let text = String(value)
  let parts = text.split(separator: "e", omittingEmptySubsequences: false)
  guard parts.count == 2, let exponent = Int(parts[1]) else {
    return text.hasSuffix(".0") ? String(text.dropLast(2)) : text
  }
  let negative = parts[0].hasPrefix("-")
  let mantissa = negative ? parts[0].dropFirst() : parts[0][...]
  let components = mantissa.split(separator: ".", omittingEmptySubsequences: false)
  let digits = components.joined()
  let point = components[0].count + exponent
  let expanded: String
  if point <= 0 {
    expanded = "0." + String(repeating: "0", count: -point) + digits
  } else if point >= digits.count {
    expanded = digits + String(repeating: "0", count: point - digits.count)
  } else {
    let index = digits.index(digits.startIndex, offsetBy: point)
    expanded = String(digits[..<index]) + "." + digits[index...]
  }
  return (negative ? "-" : "") + expanded
}
func printRow(_ value: JSONValue) -> String {
  let fields = value.object
  if fields["status"]?.string == "unsure" { return "unsure" }
  if fields["status"]?.string == "failed" {
    let failure = fields["error"]?.object ?? [:]
    return "failed: " + printLeaf(failure["code"]?.string ?? "") + ": "
      + printLeaf(failure["message"]?.string ?? "")
  }
  let answer = fields["answer"] ?? .null
  switch fields["function"]?.string {
  case "decide":
    if case .bool(let b) = answer { return b ? "true" : "false" }
    return ""
  case "choose": return printLeaf(answer.string ?? "")
  case "tag": return answer.array.map { printLeaf($0.string ?? "") }.joined(separator: "\t")
  case "score", "rank":
    switch answer {
    case .number(let x): return printNumber(x)
    case .integer(let x): return String(x)
    case .unsigned(let x): return String(x)
    default: return ""
    }
  case "find": return printLeaf(answer.object["text"]?.string ?? "")
  case "recognize":
    return answer.array.map { name in
      let n = name.object
      return
        "\(printLeaf(n["kind"]?.string ?? "")): \(printLeaf(n["text"]?.string ?? "")) (\(n["start"]?.int ?? 0)..\(n["end"]?.int ?? 0))"
    }.joined(separator: "\t")
  case "relate":
    return answer.array.map { relation in
      let r = relation.object
      return
        "\(printLeaf(r["source"]?.object["text"]?.string ?? "")) --\(printLeaf(r["relation"]?.string ?? ""))--> \(printLeaf(r["target"]?.object["text"]?.string ?? ""))"
    }.joined(separator: "\t")
  case "annotate":
    return answer.object.keys.sorted(by: utf8Before).map {
      printLeaf($0) + "=" + printRow(answer.object[$0] ?? .null)
    }.joined(separator: "\t")
  default: return ""
  }
}
