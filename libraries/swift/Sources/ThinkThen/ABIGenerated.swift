// Generated from the compiler-derived C header. Do not edit.
import CThinkThen

public enum QuestionGrammar: UInt32, Sendable {
  case `atomic` = 1
  case `dynamicChoose` = 3
  case `find` = 8
  case `rankSet` = 7
  case `rank` = 6
  case `recognize` = 4
  case `relate` = 5
  case `set` = 2
}
public enum UsagePersistenceState: UInt32, Sendable {
  case `disabled` = 1
  case `failed` = 4
  case `pending` = 2
  case `written` = 3
}
func nativeFailureKind(_ code: Int32) -> OwnedFailureKind {
  switch code {
  case 2: return .`backend`
  case 5: return .`cancelled`
  case 3: return .`deadline`
  case 6: return .`defect`
  case 4: return .`local`
  case 1: return .`usage`
  default: return .unknown(String(code))
  }
}
extension Client {
  public func finishUsageStatus() throws -> OwnedUsagePersistenceStatus {
    try usage(thinkthen_engine_finish_usage_status_v1)
  }
  public func usagePersistence() throws -> OwnedUsagePersistenceStatus {
    try usage(thinkthen_engine_usage_persistence_v1)
  }
}
