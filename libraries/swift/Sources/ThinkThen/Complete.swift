import Foundation
// Host descriptors; Codable is a host codec, not the native request/result codec.
public enum DescriptorError: Error { case invalidIdentity, invalidJSON }
public enum Function: String, Codable, Sendable { case `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize`, `relate` }
public enum CompleteFailureKind: Int, Codable, Sendable { case usage=1, backend, deadline, local, cancelled, defect }
public enum ContentKind: String, Codable, Sendable { case `text`, `json` }
public enum RuleKind: String, Codable, Sendable { case `default`, `null`, `cut`, `band` }
public enum Media: String, Codable, Sendable { case `jpeg`, `png` }
public enum SourceUnit: String, Codable, Sendable { case `line`, `window`, `file`, `imageFile` }
public enum ValueKind: String, Codable, Sendable { case `null`, `boolean`, `authored` }
public enum AtomicKind: String, Codable, Sendable { case `yesNo`, `choice`, `tag`, `score`, `find` }
public enum MemberState: String, Codable, Sendable { case `success`, `failure` }
public enum MemberCause: String, Codable, Sendable { case `missingAnswer`, `wrongKind`, `missingProbability`, `invalidProbability`, `invalidDistribution`, `unexpectedProbability` }
public enum Origin: String, Codable, Sendable { case `live`, `cache`, `replay`, `proxy`, `memory` }
public enum AttemptOutcome: String, Codable, Sendable { case `ok`, `status`, `transport` }
public enum RelationMethod: String, Codable, Sendable { case `yesNo`, `choice` }
public enum Direction: String, Codable, Sendable { case `sourceToTarget`, `either` }
public enum Stage: String, Codable, Sendable { case `boundary`, `kind`, `edge`, `relation` }
public enum IdentityKind: String, Codable, Sendable { case `observation`, `failure` }
public enum StopCause: String, Codable, Sendable { case `usage`, `local`, `noKey`, `transport`, `status`, `tooLarge`, `reply`, `backend`, `cancelled`, `defect`, `deadline` }
public enum BatchKind: String, Codable, Sendable { case `records`, `max` }
public enum EventKind: String, Codable, Sendable { case `question`, `row` }
public struct CallId: Codable, Equatable, Sendable {
    public let value: String
    public init(_ value: String) throws {
        guard value.utf8.count == 64, value.utf8.allSatisfy({ (48...57).contains($0) || (97...102).contains($0) }) else { throw DescriptorError.invalidIdentity }
        self.value = value
    }
    public init(from decoder: Decoder) throws { try self.init(decoder.singleValueContainer().decode(String.self)) }
    public func encode(to encoder: Encoder) throws { var c = encoder.singleValueContainer(); try c.encode(value) }
}
public struct SdkRequestId: Codable, Equatable, Sendable {
    public let value: String
    public init(_ value: String) throws {
        guard value.utf8.count == 64, value.utf8.allSatisfy({ (48...57).contains($0) || (97...102).contains($0) }) else { throw DescriptorError.invalidIdentity }
        self.value = value
    }
    public init(from decoder: Decoder) throws { try self.init(decoder.singleValueContainer().decode(String.self)) }
    public func encode(to encoder: Encoder) throws { var c = encoder.singleValueContainer(); try c.encode(value) }
}
public struct ObservationId: Codable, Equatable, Sendable {
    public let value: String
    public init(_ value: String) throws {
        guard value.utf8.count == 64, value.utf8.allSatisfy({ (48...57).contains($0) || (97...102).contains($0) }) else { throw DescriptorError.invalidIdentity }
        self.value = value
    }
    public init(from decoder: Decoder) throws { try self.init(decoder.singleValueContainer().decode(String.self)) }
    public func encode(to encoder: Encoder) throws { var c = encoder.singleValueContainer(); try c.encode(value) }
}
public struct FailureId: Codable, Equatable, Sendable {
    public let value: String
    public init(_ value: String) throws {
        guard value.utf8.count == 64, value.utf8.allSatisfy({ (48...57).contains($0) || (97...102).contains($0) }) else { throw DescriptorError.invalidIdentity }
        self.value = value
    }
    public init(from decoder: Decoder) throws { try self.init(decoder.singleValueContainer().decode(String.self)) }
    public func encode(to encoder: Encoder) throws { var c = encoder.singleValueContainer(); try c.encode(value) }
}
public struct AnswerId: Codable, Equatable, Sendable {
    public let value: String
    public init(_ value: String) throws {
        guard value.utf8.count == 64, value.utf8.allSatisfy({ (48...57).contains($0) || (97...102).contains($0) }) else { throw DescriptorError.invalidIdentity }
        self.value = value
    }
    public init(from decoder: Decoder) throws { try self.init(decoder.singleValueContainer().decode(String.self)) }
    public func encode(to encoder: Encoder) throws { var c = encoder.singleValueContainer(); try c.encode(value) }
}
public struct Digest: Codable, Equatable, Sendable {
    public let value: String
    public init(_ value: String) throws {
        guard value.utf8.count == 64, value.utf8.allSatisfy({ (48...57).contains($0) || (97...102).contains($0) }) else { throw DescriptorError.invalidIdentity }
        self.value = value
    }
    public init(from decoder: Decoder) throws { try self.init(decoder.singleValueContainer().decode(String.self)) }
    public func encode(to encoder: Encoder) throws { var c = encoder.singleValueContainer(); try c.encode(value) }
}
public struct Content: Codable, Sendable {
    public var kind: ContentKind; public var data: String
    public init(kind: ContentKind, data: String) { self.kind = kind; self.data = data }
}
public struct Rule: Codable, Sendable {
    public var kind: RuleKind; public var low: Double; public var high: Double
    public init(kind: RuleKind, low: Double, high: Double) { self.kind = kind; self.low = low; self.high = high }
}
public struct Choice: Codable, Sendable {
    public var name: String; public var description: Content?; public var weight: Double?
    public init(name: String, description: Content?, weight: Double?) { self.name = name; self.description = description; self.weight = weight }
}
public struct Relation: Codable, Sendable {
    public var name: String; public var source: String; public var target: String; public var reads: String?; public var either: Bool; public var single: Bool
    public init(name: String, source: String, target: String, reads: String?, either: Bool, single: Bool) { self.name = name; self.source = source; self.target = target; self.reads = reads; self.either = either; self.single = single }
}
public struct QuestionMember: Codable, Sendable {
    public var name: String; public var question: Question
    public init(name: String, question: Question) { self.name = name; self.question = question }
}
public struct Question: Codable, Sendable {
    public var instructions: String? = nil; public var entityDefinition: String? = nil
    public var kind: Function; public var text: Content; public var yes: Content?; public var no: Content?; public var choices: [Choice]; public var threshold: Rule; public var relationThreshold: Rule; public var model: String?; public var profile: String?; public var batch: UInt64?; public var batchMax: Bool; public var none: Bool; public var on: [String]; public var members: [QuestionMember]; public var kinds: [Choice]; public var relations: [Relation]; public var namePointer: String?; public var kindPointer: String?
    public init(kind: Function, text: Content, yes: Content?, no: Content?, choices: [Choice], threshold: Rule, relationThreshold: Rule, model: String?, profile: String?, batch: UInt64?, batchMax: Bool, none: Bool, on: [String], members: [QuestionMember], kinds: [Choice], relations: [Relation], namePointer: String?, kindPointer: String?) { self.kind = kind; self.text = text; self.yes = yes; self.no = no; self.choices = choices; self.threshold = threshold; self.relationThreshold = relationThreshold; self.model = model; self.profile = profile; self.batch = batch; self.batchMax = batchMax; self.none = none; self.on = on; self.members = members; self.kinds = kinds; self.relations = relations; self.namePointer = namePointer; self.kindPointer = kindPointer }
}
public struct ImageInput: Codable, Sendable {
    public var media: Media; public var bytes: [UInt8]; public var filename: String?
    public init(media: Media, bytes: [UInt8], filename: String?) { self.media = media; self.bytes = bytes; self.filename = filename }
}
public struct ImageView: Codable, Sendable {
    public var media: Media; public var bytes: [UInt8]; public var width: UInt32; public var height: UInt32; public var filename: String?
    public init(media: Media, bytes: [UInt8], width: UInt32, height: UInt32, filename: String?) { self.media = media; self.bytes = bytes; self.width = width; self.height = height; self.filename = filename }
}
public struct RecordInput: Codable, Sendable {
    public var original: Content?; public var context: Content?; public var options: [Choice]; public var images: [ImageInput]
    public init(original: Content?, context: Content?, options: [Choice], images: [ImageInput]) { self.original = original; self.context = context; self.options = options; self.images = images }
}
public struct FileSource: Codable, Sendable {
    public var paths: [String]; public var unit: SourceUnit; public var window: UInt64
    public init(paths: [String], unit: SourceUnit, window: UInt64) { self.paths = paths; self.unit = unit; self.window = window }
}
public struct CallControls: Codable, Sendable {
    public var context: Content?; public var batch: UInt64?; public var batchMax: Bool; public var attempts: Bool
    public init(context: Content?, batch: UInt64?, batchMax: Bool, attempts: Bool) { self.context = context; self.batch = batch; self.batchMax = batchMax; self.attempts = attempts }
}
public struct Probability: Codable, Sendable {
    public var name: String; public var value: Double
    public init(name: String, value: Double) { self.name = name; self.value = value }
}
public struct DecideValue: Codable, Sendable {
    public var kind: ValueKind; public var boolean: Bool; public var authored: Content?
    public init(kind: ValueKind, boolean: Bool, authored: Content?) { self.kind = kind; self.boolean = boolean; self.authored = authored }
}
public struct AtomicAnswer: Codable, Sendable {
    public var kind: AtomicKind; public var probability: Double?; public var pick: String?; public var level: String?; public var probabilities: [Probability]; public var confidence: Double?
    public init(kind: AtomicKind, probability: Double?, pick: String?, level: String?, probabilities: [Probability], confidence: Double?) { self.kind = kind; self.probability = probability; self.pick = pick; self.level = level; self.probabilities = probabilities; self.confidence = confidence }
}
public struct Location: Codable, Sendable {
    public var file: String?; public var firstLine: UInt64?; public var lastLine: UInt64?
    public init(file: String?, firstLine: UInt64?, lastLine: UInt64?) { self.file = file; self.firstLine = firstLine; self.lastLine = lastLine }
}
public struct MemberValue: Codable, Sendable {
    public var function: Function; public var decide: DecideValue?; public var choose: String?; public var tag: [String]?; public var score: Double?
    public init(function: Function, decide: DecideValue?, choose: String?, tag: [String]?, score: Double?) { self.function = function; self.decide = decide; self.choose = choose; self.tag = tag; self.score = score }
}
public struct MemberFailure: Codable, Sendable {
    public var failureId: FailureId; public var cause: MemberCause
    public init(failureId: FailureId, cause: MemberCause) { self.failureId = failureId; self.cause = cause }
}
public struct MemberSuccess: Codable, Sendable {
    public var answerId: AnswerId; public var value: MemberValue; public var answer: AtomicAnswer; public var threshold: Rule
    public init(answerId: AnswerId, value: MemberValue, answer: AtomicAnswer, threshold: Rule) { self.answerId = answerId; self.value = value; self.answer = answer; self.threshold = threshold }
}
public struct AnnotationMember: Codable, Sendable {
    public var name: String; public var request: Digest; public var question: Question; public var state: MemberState; public var success: MemberSuccess?; public var failure: MemberFailure?
    public init(name: String, request: Digest, question: Question, state: MemberState, success: MemberSuccess?, failure: MemberFailure?) { self.name = name; self.request = request; self.question = question; self.state = state; self.success = success; self.failure = failure }
}
public struct Entity: Codable, Sendable {
    public var text: String; public var start: UInt64; public var end: UInt64; public var length: UInt64; public var kind: String; public var strength: Double
    public init(text: String, start: UInt64, end: UInt64, length: UInt64, kind: String, strength: Double) { self.text = text; self.start = start; self.end = end; self.length = length; self.kind = kind; self.strength = strength }
}
public struct EntityEdge: Codable, Sendable {
    public var relation: String; public var source: Entity; public var target: Entity; public var probability: Double; public var either: Bool
    public init(relation: String, source: Entity, target: Entity, probability: Double, either: Bool) { self.relation = relation; self.source = source; self.target = target; self.probability = probability; self.either = either }
}
public struct Place: Codable, Sendable {
    public var start: UInt64; public var end: UInt64
    public init(start: UInt64, end: UInt64) { self.start = start; self.end = end }
}
public struct Piece: Codable, Sendable {
    public var start: UInt64; public var end: UInt64; public var tags: [Probability]
    public init(start: UInt64, end: UInt64, tags: [Probability]) { self.start = start; self.end = end; self.tags = tags }
}
public struct NameSpan: Codable, Sendable {
    public var start: UInt64; public var end: UInt64; public var kinds: [Probability]?; public var edges: [Probability]?
    public init(start: UInt64, end: UInt64, kinds: [Probability]?, edges: [Probability]?) { self.start = start; self.end = end; self.kinds = kinds; self.edges = edges }
}
public struct PairSpan: Codable, Sendable {
    public var relation: String; public var source: Place; public var target: Place; public var probability: Double
    public init(relation: String, source: Place, target: Place, probability: Double) { self.relation = relation; self.source = source; self.target = target; self.probability = probability }
}
public struct RecognizeValue: Codable, Sendable {
    public var entities: [Entity]; public var relations: [EntityEdge]?
    public init(entities: [Entity], relations: [EntityEdge]?) { self.entities = entities; self.relations = relations }
}
public struct RecognizeAnswer: Codable, Sendable {
    public var pieces: [Piece]; public var names: [NameSpan]; public var pairs: [PairSpan]
    public init(pieces: [Piece], names: [NameSpan], pairs: [PairSpan]) { self.pieces = pieces; self.names = names; self.pairs = pairs }
}
public struct Endpoint: Codable, Sendable {
    public var name: String; public var kind: String
    public init(name: String, kind: String) { self.name = name; self.kind = kind }
}
public struct Edge: Codable, Sendable {
    public var relation: String; public var source: Endpoint; public var target: Endpoint; public var probability: Double; public var either: Bool
    public init(relation: String, source: Endpoint, target: Endpoint, probability: Double, either: Bool) { self.relation = relation; self.source = source; self.target = target; self.probability = probability; self.either = either }
}
public struct RelationSuccess: Codable, Sendable {
    public var answerId: AnswerId; public var probability: Double; public var accepted: Bool
    public init(answerId: AnswerId, probability: Double, accepted: Bool) { self.answerId = answerId; self.probability = probability; self.accepted = accepted }
}
public struct RelationAnswer: Codable, Sendable {
    public var relation: String; public var reads: String; public var method: RelationMethod; public var direction: Direction; public var source: Endpoint; public var target: Endpoint?; public var request: Digest; public var state: MemberState; public var success: RelationSuccess?; public var failure: MemberFailure?
    public init(relation: String, reads: String, method: RelationMethod, direction: Direction, source: Endpoint, target: Endpoint?, request: Digest, state: MemberState, success: RelationSuccess?, failure: MemberFailure?) { self.relation = relation; self.reads = reads; self.method = method; self.direction = direction; self.source = source; self.target = target; self.request = request; self.state = state; self.success = success; self.failure = failure }
}
public struct TokenUsage: Codable, Sendable {
    public var inputTokens: UInt64; public var outputTokens: UInt64
    public init(inputTokens: UInt64, outputTokens: UInt64) { self.inputTokens = inputTokens; self.outputTokens = outputTokens }
}
public struct QuestionSource: Codable, Sendable {
    public var origin: Origin; public var answeredBy: String
    public init(origin: Origin, answeredBy: String) { self.origin = origin; self.answeredBy = answeredBy }
}
public struct ObservationIdentity: Codable, Sendable {
    public var kind: IdentityKind; public var observationId: ObservationId?; public var failureId: FailureId?
    public init(kind: IdentityKind, observationId: ObservationId?, failureId: FailureId?) { self.kind = kind; self.observationId = observationId; self.failureId = failureId }
}
public struct ProfileWarning: Codable, Sendable {
    public var tunedFor: String; public var running: String
    public init(tunedFor: String, running: String) { self.tunedFor = tunedFor; self.running = running }
}
public struct BatchSetting: Codable, Sendable {
    public var kind: BatchKind; public var records: UInt64
    public init(kind: BatchKind, records: UInt64) { self.kind = kind; self.records = records }
}
public struct BatchWarning: Codable, Sendable {
    public var tunedFor: BatchSetting; public var running: BatchSetting
    public init(tunedFor: BatchSetting, running: BatchSetting) { self.tunedFor = tunedFor; self.running = running }
}
public struct Attempt: Codable, Sendable {
    public var ordinal: UInt64; public var requestSha256: Digest; public var wallMs: UInt64; public var outcome: AttemptOutcome; public var sdkRequestId: SdkRequestId; public var status: UInt16?; public var serverMs: UInt64?; public var requestId: String?
    public init(ordinal: UInt64, requestSha256: Digest, wallMs: UInt64, outcome: AttemptOutcome, sdkRequestId: SdkRequestId, status: UInt16?, serverMs: UInt64?, requestId: String?) { self.ordinal = ordinal; self.requestSha256 = requestSha256; self.wallMs = wallMs; self.outcome = outcome; self.sdkRequestId = sdkRequestId; self.status = status; self.serverMs = serverMs; self.requestId = requestId }
}
public struct Meta: Codable, Sendable {
    public var tool: String; public var questionSha256: Digest?; public var questionsSha256: Digest?; public var url: String; public var model: String; public var usage: TokenUsage?; public var requestsSent: UInt64; public var cached: Bool; public var requests: [Digest]; public var failedQuestions: UInt64; public var profileWarning: ProfileWarning?; public var batchSetting: BatchSetting?; public var batchWarning: BatchWarning?; public var contextSha256: Digest?; public var attempts: [Attempt]?; public var origin: Origin?; public var questionSources: [QuestionSource]; public var observations: [ObservationIdentity]; public var answeredBy: String?
    public init(tool: String, questionSha256: Digest?, questionsSha256: Digest?, url: String, model: String, usage: TokenUsage?, requestsSent: UInt64, cached: Bool, requests: [Digest], failedQuestions: UInt64, profileWarning: ProfileWarning?, batchSetting: BatchSetting?, batchWarning: BatchWarning?, contextSha256: Digest?, attempts: [Attempt]?, origin: Origin?, questionSources: [QuestionSource], observations: [ObservationIdentity], answeredBy: String?) { self.tool = tool; self.questionSha256 = questionSha256; self.questionsSha256 = questionsSha256; self.url = url; self.model = model; self.usage = usage; self.requestsSent = requestsSent; self.cached = cached; self.requests = requests; self.failedQuestions = failedQuestions; self.profileWarning = profileWarning; self.batchSetting = batchSetting; self.batchWarning = batchWarning; self.contextSha256 = contextSha256; self.attempts = attempts; self.origin = origin; self.questionSources = questionSources; self.observations = observations; self.answeredBy = answeredBy }
}
public struct CallFacts: Codable, Sendable {
    public var callId: CallId; public var cacheAnswers: UInt64; public var estimatedCostUsd: String?; public var inputTokens: UInt64?; public var model: String?; public var outputTokens: UInt64?; public var records: UInt64; public var requestsSent: UInt64; public var seconds: Double; public var commandMs: UInt64?
    public init(callId: CallId, cacheAnswers: UInt64, estimatedCostUsd: String?, inputTokens: UInt64?, model: String?, outputTokens: UInt64?, records: UInt64, requestsSent: UInt64, seconds: Double, commandMs: UInt64?) { self.callId = callId; self.cacheAnswers = cacheAnswers; self.estimatedCostUsd = estimatedCostUsd; self.inputTokens = inputTokens; self.model = model; self.outputTokens = outputTokens; self.records = records; self.requestsSent = requestsSent; self.seconds = seconds; self.commandMs = commandMs }
}
public struct Stopped: Codable, Sendable {
    public var at: UInt64?; public var cause: StopCause; public var status: UInt16?; public var retryable: Bool
    public init(at: UInt64?, cause: StopCause, status: UInt16?, retryable: Bool) { self.at = at; self.cause = cause; self.status = status; self.retryable = retryable }
}
public struct CompleteError: Codable, Sendable {
    public var code: CompleteFailureKind; public var message: String; public var retryable: Bool; public var stopped: Stopped?; public var facts: CallFacts?; public var attempts: [Attempt]?
    public init(code: CompleteFailureKind, message: String, retryable: Bool, stopped: Stopped?, facts: CallFacts?, attempts: [Attempt]?) { self.code = code; self.message = message; self.retryable = retryable; self.stopped = stopped; self.facts = facts; self.attempts = attempts }
}
public struct CommonRow: Codable, Sendable {
    public var answerId: AnswerId; public var input: Content?; public var question: Question?; public var answer: AtomicAnswer?; public var threshold: Rule?; public var position: Location?; public var inputFile: String?; public var meta: Meta; public var images: [ImageView]?
    public init(answerId: AnswerId, input: Content?, question: Question?, answer: AtomicAnswer?, threshold: Rule?, position: Location?, inputFile: String?, meta: Meta, images: [ImageView]?) { self.answerId = answerId; self.input = input; self.question = question; self.answer = answer; self.threshold = threshold; self.position = position; self.inputFile = inputFile; self.meta = meta; self.images = images }
}
public struct DecideRow: Codable, Sendable {
    public var common: CommonRow; public var value: DecideValue
    public init(common: CommonRow, value: DecideValue) { self.common = common; self.value = value }
}
public struct ChooseRow: Codable, Sendable {
    public var common: CommonRow; public var value: String?
    public init(common: CommonRow, value: String?) { self.common = common; self.value = value }
}
public struct TagRow: Codable, Sendable {
    public var common: CommonRow; public var value: [String]
    public init(common: CommonRow, value: [String]) { self.common = common; self.value = value }
}
public struct ScoreRow: Codable, Sendable {
    public var common: CommonRow; public var value: Double
    public init(common: CommonRow, value: Double) { self.common = common; self.value = value }
}
public struct FilterRow: Codable, Sendable {
    public var common: CommonRow; public var value: Bool
    public init(common: CommonRow, value: Bool) { self.common = common; self.value = value }
}
public struct RankRow: Codable, Sendable {
    public var common: CommonRow; public var value: UInt64?; public var questionName: String?
    public init(common: CommonRow, value: UInt64?, questionName: String?) { self.common = common; self.value = value; self.questionName = questionName }
}
public struct FindRow: Codable, Sendable {
    public var common: CommonRow; public var value: Content?; public var index: UInt64?
    public init(common: CommonRow, value: Content?, index: UInt64?) { self.common = common; self.value = value; self.index = index }
}
public struct AnnotateRow: Codable, Sendable {
    public var common: CommonRow; public var answers: [AnnotationMember]
    public init(common: CommonRow, answers: [AnnotationMember]) { self.common = common; self.answers = answers }
}
public struct RecognizeRow: Codable, Sendable {
    public var common: CommonRow; public var value: RecognizeValue; public var answer: RecognizeAnswer
    public init(common: CommonRow, value: RecognizeValue, answer: RecognizeAnswer) { self.common = common; self.value = value; self.answer = answer }
}
public struct RelateRow: Codable, Sendable {
    public var common: CommonRow; public var value: [Edge]; public var questions: [RelationAnswer]
    public init(common: CommonRow, value: [Edge], questions: [RelationAnswer]) { self.common = common; self.value = value; self.questions = questions }
}
public struct RowValue: Codable, Sendable {
    public var function: Function; public var decide: DecideRow?; public var choose: ChooseRow?; public var tag: TagRow?; public var score: ScoreRow?; public var filter: FilterRow?; public var rank: RankRow?; public var find: FindRow?; public var annotate: AnnotateRow?; public var recognize: RecognizeRow?; public var relate: RelateRow?
    public init(function: Function, decide: DecideRow?, choose: ChooseRow?, tag: TagRow?, score: ScoreRow?, filter: FilterRow?, rank: RankRow?, find: FindRow?, annotate: AnnotateRow?, recognize: RecognizeRow?, relate: RelateRow?) { self.function = function; self.decide = decide; self.choose = choose; self.tag = tag; self.score = score; self.filter = filter; self.rank = rank; self.find = find; self.annotate = annotate; self.recognize = recognize; self.relate = relate }
}
public struct ObservedProbabilities: Codable, Sendable {
    public var yes: Double?; public var named: [Probability]?
    public init(yes: Double?, named: [Probability]?) { self.yes = yes; self.named = named }
}
public struct ObservationSuccess: Codable, Sendable {
    public var answerId: AnswerId; public var observationId: ObservationId; public var value: MemberValue; public var probabilities: ObservedProbabilities; public var confidence: Double?
    public init(answerId: AnswerId, observationId: ObservationId, value: MemberValue, probabilities: ObservedProbabilities, confidence: Double?) { self.answerId = answerId; self.observationId = observationId; self.value = value; self.probabilities = probabilities; self.confidence = confidence }
}
public struct QuestionObservation: Codable, Sendable {
    public var index: UInt64; public var member: String?; public var stage: Stage?; public var position: UInt64; public var questionSha256: Digest; public var model: String; public var url: String; public var requests: [Digest]; public var requestsSent: UInt64; public var cached: Bool; public var failedQuestions: UInt64; public var usage: TokenUsage?; public var questionSources: [QuestionSource]; public var state: MemberState; public var success: ObservationSuccess?; public var failure: MemberFailure?
    public init(index: UInt64, member: String?, stage: Stage?, position: UInt64, questionSha256: Digest, model: String, url: String, requests: [Digest], requestsSent: UInt64, cached: Bool, failedQuestions: UInt64, usage: TokenUsage?, questionSources: [QuestionSource], state: MemberState, success: ObservationSuccess?, failure: MemberFailure?) { self.index = index; self.member = member; self.stage = stage; self.position = position; self.questionSha256 = questionSha256; self.model = model; self.url = url; self.requests = requests; self.requestsSent = requestsSent; self.cached = cached; self.failedQuestions = failedQuestions; self.usage = usage; self.questionSources = questionSources; self.state = state; self.success = success; self.failure = failure }
}
public struct RowObservation: Codable, Sendable {
    public var index: UInt64; public var value: RowValue
    public init(index: UInt64, value: RowValue) { self.index = index; self.value = value }
}
public struct ObservationEvent: Codable, Sendable {
    public var kind: EventKind; public var question: QuestionObservation?; public var row: RowObservation?
    public init(kind: EventKind, question: QuestionObservation?, row: RowObservation?) { self.kind = kind; self.question = question; self.row = row }
}
public struct QuestionInput: Codable, Sendable {
    public var question: Question?; public var file: String?
    public init(question: Question?, file: String?) { self.question = question; self.file = file }
}
public struct InputSource: Codable, Sendable {
    public var records: [RecordInput]?; public var files: FileSource?
    public init(records: [RecordInput]?, files: FileSource?) { self.records = records; self.files = files }
}
public struct CompleteRequest: Codable, Sendable {
    public var function: Function; public var question: QuestionInput; public var source: InputSource; public var controls: CallControls
    public init(function: Function, question: QuestionInput, source: InputSource, controls: CallControls) { self.function = function; self.question = question; self.source = source; self.controls = controls }
}
public struct CompleteSummary: Codable, Sendable {
    public var count: UInt64; public var observationCount: UInt64
    public var schema: String; public var answerId: AnswerId; public var function: Function; public var meta: Meta; public var facts: CallFacts; public var attempts: [Attempt]?
    public init(count: UInt64, observationCount: UInt64, schema: String, answerId: AnswerId, function: Function, meta: Meta, facts: CallFacts, attempts: [Attempt]?) { self.count = count; self.observationCount = observationCount; self.schema = schema; self.answerId = answerId; self.function = function; self.meta = meta; self.facts = facts; self.attempts = attempts }
}
public extension Content {
    static func text(_ value: String) -> Content { Content(kind: .text, data: value) }
    static func json(_ value: String) throws -> Content {
        guard (try? JSONSerialization.jsonObject(with: Data(value.utf8), options: .fragmentsAllowed)) != nil else { throw DescriptorError.invalidJSON }
        return Content(kind: .json, data: value)
    }
}
public extension Question {
    static func asked(_ kind: Function, _ text: Content) -> Question {
        Question(kind: kind, text: text, yes: nil, no: nil, choices: [], threshold: Rule(kind: .default, low: 0, high: 0), relationThreshold: Rule(kind: .default, low: 0, high: 0), model: nil, profile: nil, batch: nil, batchMax: false, none: false, on: [], members: [], kinds: [], relations: [], namePointer: nil, kindPointer: nil)
    }
}
public enum Requests {
    public static func decide(_ question: QuestionInput, _ source: InputSource, controls: CallControls = CallControls(context: nil, batch: nil, batchMax: false, attempts: false)) -> CompleteRequest { CompleteRequest(function: .decide, question: question, source: source, controls: controls) }
    public static func choose(_ question: QuestionInput, _ source: InputSource, controls: CallControls = CallControls(context: nil, batch: nil, batchMax: false, attempts: false)) -> CompleteRequest { CompleteRequest(function: .choose, question: question, source: source, controls: controls) }
    public static func tag(_ question: QuestionInput, _ source: InputSource, controls: CallControls = CallControls(context: nil, batch: nil, batchMax: false, attempts: false)) -> CompleteRequest { CompleteRequest(function: .tag, question: question, source: source, controls: controls) }
    public static func score(_ question: QuestionInput, _ source: InputSource, controls: CallControls = CallControls(context: nil, batch: nil, batchMax: false, attempts: false)) -> CompleteRequest { CompleteRequest(function: .score, question: question, source: source, controls: controls) }
    public static func filter(_ question: QuestionInput, _ source: InputSource, controls: CallControls = CallControls(context: nil, batch: nil, batchMax: false, attempts: false)) -> CompleteRequest { CompleteRequest(function: .filter, question: question, source: source, controls: controls) }
    public static func rank(_ question: QuestionInput, _ source: InputSource, controls: CallControls = CallControls(context: nil, batch: nil, batchMax: false, attempts: false)) -> CompleteRequest { CompleteRequest(function: .rank, question: question, source: source, controls: controls) }
    public static func find(_ question: QuestionInput, _ source: InputSource, controls: CallControls = CallControls(context: nil, batch: nil, batchMax: false, attempts: false)) -> CompleteRequest { CompleteRequest(function: .find, question: question, source: source, controls: controls) }
    public static func annotate(_ question: QuestionInput, _ source: InputSource, controls: CallControls = CallControls(context: nil, batch: nil, batchMax: false, attempts: false)) -> CompleteRequest { CompleteRequest(function: .annotate, question: question, source: source, controls: controls) }
    public static func recognize(_ question: QuestionInput, _ source: InputSource, controls: CallControls = CallControls(context: nil, batch: nil, batchMax: false, attempts: false)) -> CompleteRequest { CompleteRequest(function: .recognize, question: question, source: source, controls: controls) }
    public static func relate(_ question: QuestionInput, _ source: InputSource, controls: CallControls = CallControls(context: nil, batch: nil, batchMax: false, attempts: false)) -> CompleteRequest { CompleteRequest(function: .relate, question: question, source: source, controls: controls) }
}
public extension QuestionInput {
    static func asked(_ value: Question) -> QuestionInput { QuestionInput(question: value, file: nil) }
    static func questionFile(_ path: String) -> QuestionInput { QuestionInput(question: nil, file: path) }
}
public extension InputSource {
    static func records(_ values: [RecordInput]) -> InputSource { InputSource(records: values, files: nil) }
    static func files(_ value: FileSource) -> InputSource { InputSource(records: nil, files: value) }
}
// This reader requires observed result/2 facts. It never upgrades legacy facts.
public enum CompleteReaders {
    public static func facts(_ json: String) throws -> CallFacts {
        let decoder = JSONDecoder()
        decoder.keyDecodingStrategy = .convertFromSnakeCase
        let value = try decoder.decode(CallFacts.self, from: Data(json.utf8))
        guard value.seconds.isFinite, value.seconds >= 0 else { throw DescriptorError.invalidJSON }
        if let cost = value.estimatedCostUsd {
            let bytes = Array(cost.utf8)
            guard bytes.count >= 8, bytes[bytes.count-7] == 46,
                  bytes.enumerated().allSatisfy({ $0.offset == bytes.count-7 || (48...57).contains($0.element) }) else { throw DescriptorError.invalidJSON }
        }
        return value
    }
}
public struct CompleteResult: Codable, Sendable {
    public var summary: CompleteSummary
    public var rows: [RowValue]
    public var observations: [ObservationEvent]
    public init(summary: CompleteSummary, rows: [RowValue], observations: [ObservationEvent]) {
        self.summary = summary; self.rows = rows; self.observations = observations
    }
}
