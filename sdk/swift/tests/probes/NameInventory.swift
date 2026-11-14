import Foundation

// The raw saved-definition loader needs exact names before a Swift dictionary.
let json =
  #"{"é":{"function":"decide","question":"First?"},"e\u0301":{"function":"decide","question":"Second?"}}"#
let object = try JSONSerialization.jsonObject(with: Data(json.utf8)) as! NSDictionary
let names = object.allKeys.map { $0 as! String }
print("Foundation exact names:", names.count)
print("Swift representable names:", Set(names).count)
