import Foundation

struct CallProjection {
  let call: Call
  let receipts: [DataValue]
  let resolutions: [UInt64: DataValue]

  func value(_ index: Int, projection: String, resolved: Bool) throws -> DataValue? {
    let receipt = try jsonObject(receipts[index])
    if resolved {
      guard let judgment = resolutions[UInt64(index)] else {
        throw ValueConversionError("Missing normalized judgment")
      }
      if judgment == .null { return nil }
    }
    return try jsonRequired(receipt, projection)
  }

  func probability(_ index: Int, path: [String], pick: [String]?, settled: Bool) throws -> Double? {
    guard settled else { return nil }
    guard let probability = try callPath(receipts[index], path) else {
      throw ValueConversionError("Missing answer probability")
    }
    if let pick {
      guard let choice = try callPath(receipts[index], pick), choice != .null else { return nil }
      let selected = try String.read(choice)
      guard let value = try jsonObject(probability)[selected] else {
        throw ValueConversionError("Missing picked probability")
      }
      return try Double.read(value)
    }
    return try Double.read(probability)
  }
}

func callPath(_ value: DataValue, _ path: [String]) throws -> DataValue? {
  var cursor: DataValue? = value
  for key in path {
    guard let value = cursor else { return nil }
    cursor = try jsonObject(value)[key]
  }
  return cursor
}

extension Call {
  func project(
    _ function: String, collection: Bool, resolution: [String],
    guards: [([String], String)], indexPath: [String]
  ) throws -> CallProjection {
    var aggregate: DataValue?
    var rows: [DataValue] = []
    var resolutions: [UInt64: DataValue] = [:]
    for packet in packets {
      let json = packet.json
      if try callPath(json, ["function"]) == .string(function) {
        if try callPath(json, ["kind"]) == .string("aggregate") {
          aggregate = try callPath(json, ["value"])
        } else if try callPath(json, ["kind"]) == .string("row"),
          let value = try callPath(json, ["value"])
        {
          rows.append(value)
        }
      }
      if !resolution.isEmpty, try guards.allSatisfy({ try callPath(json, $0.0) == .string($0.1) }) {
        guard let judgment = try callPath(json, resolution),
          let index = try callPath(json, indexPath)
        else {
          throw ValueConversionError("Missing normalized observation")
        }
        resolutions[try UInt64.read(index)] = judgment
      }
    }
    if let aggregate {
      if collection { rows = try [DataValue].read(aggregate) } else { rows = [aggregate] }
    }
    return CallProjection(call: self, receipts: rows, resolutions: resolutions)
  }
}

func recognitionProbabilities(_ receipt: Recognition) throws -> [Double?] {
  switch receipt.value {
  case .modeProposals(let value): return value.proposals.map { $0.probability }
  case .entities(let value):
    guard case .namesPairsPiecesProposals(let answer) = receipt.answer else {
      throw ValueConversionError("Missing recognition span probabilities")
    }
    return try value.entities.map { entity in
      guard
        let proposal = answer.proposals.first(where: {
          $0.kept && $0.selected.value?.start == entity.start
            && $0.selected.value?.end == entity.end
            && $0.kind.value == entity.kind
        })
      else { throw ValueConversionError("Missing kept recognition proposal") }
      return proposal.spanProbability
    }
  }
}
