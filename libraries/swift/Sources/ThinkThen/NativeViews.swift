import Foundation
import CThinkThen
// Each initializer copies every borrowed byte/array before the native owner is freed.
func nativeExtent(_ n: Int, _ width: Int, _ pointer: UnsafeRawPointer?) throws { guard n >= 0, n <= Int.max / max(1,width), n == 0 || pointer != nil else { throw NativeConversion.invalidExtent } }
public enum NativeConversion: Error { case invalidExtent, invalidPresence, invalidDiscriminator, invalidUTF8, missingResult }
func nativePresent(_ v: Int32) throws -> Bool { guard v == 0 || v == 1 else { throw NativeConversion.invalidPresence }; return v == 1 }
func nativeCopy(_ v: thinkthen_string_v1) throws -> String { try nativeExtent(v.len, 1, v.data); if v.len == 0 { return "" }; guard let s = String(bytes: UnsafeRawBufferPointer(start:v.data,count:v.len), encoding:.utf8) else { throw NativeConversion.invalidUTF8 }; return s }
public struct NativeAnnotateView: Sendable {
    public let common: NativeRow
    public let answers: [NativeMember]
    init(_ v: thinkthen_annotate_view_v1) throws {
        common = try nativeCopy(v.common)
        answers = try nativeCopy(v.answers)
    }
}
func nativeCopy(_ v: thinkthen_annotate_view_v1) throws -> NativeAnnotateView { try NativeAnnotateView(v) }
public struct NativeAnswer: Sendable {
    public let kind: UInt32
    public struct Values: Sendable {
        public var probability: Double? = nil
        public var choice: NativeNamedAnswer? = nil
        public var tag: [NativeProbability]? = nil
        public var score: NativeScoreAnswer? = nil
        public var find: NativeNamedAnswer? = nil
    }
    public let data: Values
    init(_ v: thinkthen_answer_v1) throws {
        kind = v.kind
        var values = Values()
        switch v.kind {
        case 1: values.probability = v.data.probability
        case 2: values.choice = try nativeCopy(v.data.choice)
        case 3: values.tag = try nativeCopy(v.data.tag)
        case 4: values.score = try nativeCopy(v.data.score)
        case 5: values.find = try nativeCopy(v.data.find)
        default: throw NativeConversion.invalidDiscriminator
        }
        data = values
    }
}
func nativeCopy(_ v: thinkthen_answer_v1) throws -> NativeAnswer { try NativeAnswer(v) }
public struct NativeAttempt: Sendable {
    public let ordinal: UInt64
    public let request_sha256: String
    public let wall_ms: UInt64
    public let outcome: UInt32
    public let sdk_request_id: String
    public let status: UInt16?
    public let server_ms: UInt64?
    public let request_id: String?
    init(_ v: thinkthen_attempt_v1) throws {
        ordinal = v.ordinal
        request_sha256 = try nativeCopy(v.request_sha256)
        wall_ms = v.wall_ms
        outcome = v.outcome
        sdk_request_id = try nativeCopy(v.sdk_request_id)
        status = try nativeCopy(v.status)
        server_ms = try nativeCopy(v.server_ms)
        request_id = try nativeCopy(v.request_id)
    }
}
func nativeCopy(_ v: thinkthen_attempt_v1) throws -> NativeAttempt { try NativeAttempt(v) }
func nativeCopy(_ v: thinkthen_attempts_v1) throws -> [NativeAttempt] { try nativeExtent(v.len,MemoryLayout<thinkthen_attempt_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeBatch: Sendable {
    public let kind: UInt32
    public let records: Int
    init(_ v: thinkthen_batch_v1) throws {
        kind = v.kind
        records = v.records
    }
}
func nativeCopy(_ v: thinkthen_batch_v1) throws -> NativeBatch { try NativeBatch(v) }
public struct NativeBatchWarning: Sendable {
    public let tuned_for: NativeBatch
    public let running: NativeBatch
    init(_ v: thinkthen_batch_warning_v1) throws {
        tuned_for = try nativeCopy(v.tuned_for)
        running = try nativeCopy(v.running)
    }
}
func nativeCopy(_ v: thinkthen_batch_warning_v1) throws -> NativeBatchWarning { try NativeBatchWarning(v) }
public struct NativeChoice: Sendable {
    public let name: String
    public let description: NativeContent?
    public let weight: Double?
    init(_ v: thinkthen_choice_v1) throws {
        name = try nativeCopy(v.name)
        description = try nativeCopy(v.description)
        weight = try nativeCopy(v.weight)
    }
}
func nativeCopy(_ v: thinkthen_choice_v1) throws -> NativeChoice { try NativeChoice(v) }
func nativeCopy(_ v: thinkthen_choices_v1) throws -> [NativeChoice] { try nativeExtent(v.len,MemoryLayout<thinkthen_choice_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeChooseView: Sendable {
    public let common: NativeRow
    public let value: String?
    init(_ v: thinkthen_choose_view_v1) throws {
        common = try nativeCopy(v.common)
        value = try nativeCopy(v.value)
    }
}
func nativeCopy(_ v: thinkthen_choose_view_v1) throws -> NativeChooseView { try NativeChooseView(v) }
public struct NativeContent: Sendable {
    public let kind: UInt32
    public let data: String
    init(_ v: thinkthen_content_v1) throws {
        kind = v.kind
        data = try nativeCopy(v.data)
    }
}
func nativeCopy(_ v: thinkthen_content_v1) throws -> NativeContent { try NativeContent(v) }
public struct NativeDecideValue: Sendable {
    public let kind: UInt32
    public struct Values: Sendable {
        public var boolean: Int32? = nil
        public var authored: NativeContent? = nil
    }
    public let data: Values
    init(_ v: thinkthen_decide_value_v1) throws {
        kind = v.kind
        var values = Values()
        switch v.kind {
        case 0: break
        case 1: values.boolean = v.data.boolean
        case 2: values.authored = try nativeCopy(v.data.authored)
        default: throw NativeConversion.invalidDiscriminator
        }
        data = values
    }
}
func nativeCopy(_ v: thinkthen_decide_value_v1) throws -> NativeDecideValue { try NativeDecideValue(v) }
public struct NativeDecideView: Sendable {
    public let common: NativeRow
    public let value: NativeDecideValue
    init(_ v: thinkthen_decide_view_v1) throws {
        common = try nativeCopy(v.common)
        value = try nativeCopy(v.value)
    }
}
func nativeCopy(_ v: thinkthen_decide_view_v1) throws -> NativeDecideView { try NativeDecideView(v) }
public struct NativeDetails: Sendable {
    public let question: NativeQuestionView?
    public let threshold: NativeRule?
    public let raw_pick: String?
    public let usage: NativeReportedUsage
    public let question_sources: [NativeSourceDetail]
    public let observations: [NativeObservationIdentity]
    public let inputs: [NativeInputView]
    init(_ v: thinkthen_details_v1) throws {
        question = try nativeCopy(v.question)
        threshold = try nativeCopy(v.threshold)
        raw_pick = try nativeCopy(v.raw_pick)
        usage = try nativeCopy(v.usage)
        question_sources = try nativeCopy(v.question_sources)
        observations = try nativeCopy(v.observations)
        inputs = try nativeCopy(v.inputs)
    }
}
func nativeCopy(_ v: thinkthen_details_v1) throws -> NativeDetails { try NativeDetails(v) }
public struct NativeEdge: Sendable {
    public let relation: String
    public let source: NativeEndpoint
    public let target: NativeEndpoint
    public let probability: Double
    public let either: Int32
    init(_ v: thinkthen_edge_v1) throws {
        relation = try nativeCopy(v.relation)
        source = try nativeCopy(v.source)
        target = try nativeCopy(v.target)
        probability = v.probability
        either = v.either
    }
}
func nativeCopy(_ v: thinkthen_edge_v1) throws -> NativeEdge { try NativeEdge(v) }
func nativeCopy(_ v: thinkthen_edges_v1) throws -> [NativeEdge] { try nativeExtent(v.len,MemoryLayout<thinkthen_edge_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeEndpoint: Sendable {
    public let name: String
    public let kind: String
    init(_ v: thinkthen_endpoint_v1) throws {
        name = try nativeCopy(v.name)
        kind = try nativeCopy(v.kind)
    }
}
func nativeCopy(_ v: thinkthen_endpoint_v1) throws -> NativeEndpoint { try NativeEndpoint(v) }
func nativeCopy(_ v: thinkthen_entities_v1) throws -> [NativeEntity] { try nativeExtent(v.len,MemoryLayout<thinkthen_entity_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeEntityEdge: Sendable {
    public let relation: String
    public let source: NativeEntity
    public let target: NativeEntity
    public let probability: Double
    public let either: Int32
    init(_ v: thinkthen_entity_edge_v1) throws {
        relation = try nativeCopy(v.relation)
        source = try nativeCopy(v.source)
        target = try nativeCopy(v.target)
        probability = v.probability
        either = v.either
    }
}
func nativeCopy(_ v: thinkthen_entity_edge_v1) throws -> NativeEntityEdge { try NativeEntityEdge(v) }
func nativeCopy(_ v: thinkthen_entity_edges_v1) throws -> [NativeEntityEdge] { try nativeExtent(v.len,MemoryLayout<thinkthen_entity_edge_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeEntity: Sendable {
    public let text: String
    public let start: Int
    public let end: Int
    public let length: Int
    public let kind: String
    public let strength: Double
    init(_ v: thinkthen_entity_v1) throws {
        text = try nativeCopy(v.text)
        start = v.start
        end = v.end
        length = v.length
        kind = try nativeCopy(v.kind)
        strength = v.strength
    }
}
func nativeCopy(_ v: thinkthen_entity_v1) throws -> NativeEntity { try NativeEntity(v) }
public struct NativeError: Sendable {
    public let code: Int32
    public let message: String
    public let retryable: Int32
    public let stopped: NativeStopped?
    init(_ v: thinkthen_error_v1) throws {
        code = v.code
        message = try nativeCopy(v.message)
        retryable = v.retryable
        stopped = try nativeCopy(v.stopped)
    }
}
func nativeCopy(_ v: thinkthen_error_v1) throws -> NativeError { try NativeError(v) }
public struct NativeFacts: Sendable {
    public let call_id: String
    public let cache_answers: UInt64
    public let estimated_cost_usd: String?
    public let input_tokens: UInt64?
    public let model: String?
    public let output_tokens: UInt64?
    public let records: UInt64
    public let requests_sent: UInt64
    public let seconds: Double
    public let command_ms: UInt64?
    init(_ v: thinkthen_facts_v1) throws {
        call_id = try nativeCopy(v.call_id)
        cache_answers = v.cache_answers
        estimated_cost_usd = try nativeCopy(v.estimated_cost_usd)
        input_tokens = try nativeCopy(v.input_tokens)
        model = try nativeCopy(v.model)
        output_tokens = try nativeCopy(v.output_tokens)
        records = v.records
        requests_sent = v.requests_sent
        seconds = v.seconds
        command_ms = try nativeCopy(v.command_ms)
    }
}
func nativeCopy(_ v: thinkthen_facts_v1) throws -> NativeFacts { try NativeFacts(v) }
public struct NativeFilterView: Sendable {
    public let common: NativeRow
    public let value: Int32
    init(_ v: thinkthen_filter_view_v1) throws {
        common = try nativeCopy(v.common)
        value = v.value
    }
}
func nativeCopy(_ v: thinkthen_filter_view_v1) throws -> NativeFilterView { try NativeFilterView(v) }
public struct NativeFindView: Sendable {
    public let common: NativeRow
    public let value: NativeContent?
    public let index: Int?
    init(_ v: thinkthen_find_view_v1) throws {
        common = try nativeCopy(v.common)
        value = try nativeCopy(v.value)
        index = try nativeCopy(v.index)
    }
}
func nativeCopy(_ v: thinkthen_find_view_v1) throws -> NativeFindView { try NativeFindView(v) }
public struct NativeImageView: Sendable {
    public let media: UInt32
    public let bytes: [UInt8]
    public let width: UInt32
    public let height: UInt32
    public let filename: String?
    init(_ v: thinkthen_image_view_v1) throws {
        media = v.media
        try nativeExtent(v.bytes_len,1,v.bytes)
        bytes = v.bytes_len == 0 ? [] : Array(UnsafeBufferPointer(start:v.bytes,count:v.bytes_len))
        width = v.width
        height = v.height
        filename = try nativeCopy(v.filename)
    }
}
func nativeCopy(_ v: thinkthen_image_view_v1) throws -> NativeImageView { try NativeImageView(v) }
func nativeCopy(_ v: thinkthen_image_views_v1) throws -> [NativeImageView] { try nativeExtent(v.len,MemoryLayout<thinkthen_image_view_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeInputDeclaration: Sendable {
    public let kind: UInt32
    public let properties: [NativeInputProperty]
    public let required: [String]
    init(_ v: thinkthen_input_declaration_v1) throws {
        kind = v.kind
        properties = try nativeCopy(v.properties)
        required = try nativeCopy(v.required)
    }
}
func nativeCopy(_ v: thinkthen_input_declaration_v1) throws -> NativeInputDeclaration { try NativeInputDeclaration(v) }
func nativeCopy(_ v: thinkthen_input_properties_v1) throws -> [NativeInputProperty] { try nativeExtent(v.len,MemoryLayout<thinkthen_input_property_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeInputProperty: Sendable {
    public let name: String
    public let kind: UInt32
    init(_ v: thinkthen_input_property_v1) throws {
        name = try nativeCopy(v.name)
        kind = v.kind
    }
}
func nativeCopy(_ v: thinkthen_input_property_v1) throws -> NativeInputProperty { try NativeInputProperty(v) }
public struct NativeInputView: Sendable {
    public let original: NativeContent?
    public let position: NativeLocation?
    public let images: [NativeImageView]?
    init(_ v: thinkthen_input_view_v1) throws {
        original = try nativeCopy(v.original)
        position = try nativeCopy(v.position)
        images = try nativeCopy(v.images)
    }
}
func nativeCopy(_ v: thinkthen_input_view_v1) throws -> NativeInputView { try NativeInputView(v) }
func nativeCopy(_ v: thinkthen_input_views_v1) throws -> [NativeInputView] { try nativeExtent(v.len,MemoryLayout<thinkthen_input_view_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeLocation: Sendable {
    public let file: String?
    public let first_line: Int?
    public let last_line: Int?
    init(_ v: thinkthen_location_v1) throws {
        file = try nativeCopy(v.file)
        first_line = try nativeCopy(v.first_line)
        last_line = try nativeCopy(v.last_line)
    }
}
func nativeCopy(_ v: thinkthen_location_v1) throws -> NativeLocation { try NativeLocation(v) }
public struct NativeMemberFailure: Sendable {
    public let failure_id: String
    public let cause: UInt32
    init(_ v: thinkthen_member_failure_v1) throws {
        failure_id = try nativeCopy(v.failure_id)
        cause = v.cause
    }
}
func nativeCopy(_ v: thinkthen_member_failure_v1) throws -> NativeMemberFailure { try NativeMemberFailure(v) }
public struct NativeMemberSuccess: Sendable {
    public let answer_id: String
    public let value: NativeMemberValue
    public let answer: NativeAnswer
    public let threshold: NativeRule
    init(_ v: thinkthen_member_success_v1) throws {
        answer_id = try nativeCopy(v.answer_id)
        value = try nativeCopy(v.value)
        answer = try nativeCopy(v.answer)
        threshold = try nativeCopy(v.threshold)
    }
}
func nativeCopy(_ v: thinkthen_member_success_v1) throws -> NativeMemberSuccess { try NativeMemberSuccess(v) }
public struct NativeMember: Sendable {
    public let name: String
    public let request: String
    public let question: NativeQuestionView
    public let state: UInt32
    public struct Values: Sendable {
        public var success: NativeMemberSuccess? = nil
        public var failure: NativeMemberFailure? = nil
    }
    public let data: Values
    init(_ v: thinkthen_member_v1) throws {
        name = try nativeCopy(v.name)
        request = try nativeCopy(v.request)
        question = try nativeCopy(v.question)
        state = v.state
        var values = Values()
        switch v.state {
        case 1: values.success = try nativeCopy(v.data.success)
        case 2: values.failure = try nativeCopy(v.data.failure)
        default: throw NativeConversion.invalidDiscriminator
        }
        data = values
    }
}
func nativeCopy(_ v: thinkthen_member_v1) throws -> NativeMember { try NativeMember(v) }
public struct NativeMemberValue: Sendable {
    public let kind: UInt32
    public struct Values: Sendable {
        public var decide: NativeDecideValue? = nil
        public var choose: String?? = nil
        public var tag: [String]? = nil
        public var score: Double? = nil
    }
    public let data: Values
    init(_ v: thinkthen_member_value_v1) throws {
        kind = v.kind
        var values = Values()
        switch v.kind {
        case 1: values.decide = try nativeCopy(v.data.decide)
        case 2: values.choose = try nativeCopy(v.data.choose)
        case 3: values.tag = try nativeCopy(v.data.tag)
        case 4: values.score = v.data.score
        default: throw NativeConversion.invalidDiscriminator
        }
        data = values
    }
}
func nativeCopy(_ v: thinkthen_member_value_v1) throws -> NativeMemberValue { try NativeMemberValue(v) }
func nativeCopy(_ v: thinkthen_members_v1) throws -> [NativeMember] { try nativeExtent(v.len,MemoryLayout<thinkthen_member_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeMeta: Sendable {
    public let tool: String
    public let question_sha256: String?
    public let questions_sha256: String?
    public let url: String
    public let model: String
    public let usage: NativeUsage?
    public let requests_sent: UInt64
    public let cached: Int32
    public let requests: [String]
    public let failed_questions: Int
    public let profile_warning: NativeProfileWarning?
    public let batch_setting: NativeBatch?
    public let batch_warning: NativeBatchWarning?
    public let context_sha256: String?
    public let attempts: [NativeAttempt]?
    public let origin: UInt32?
    public let question_sources: [NativeQuestionSource]
    public let observations: [NativeObservationIdentity]
    public let answered_by: String?
    init(_ v: thinkthen_meta_v1) throws {
        tool = try nativeCopy(v.tool)
        question_sha256 = try nativeCopy(v.question_sha256)
        questions_sha256 = try nativeCopy(v.questions_sha256)
        url = try nativeCopy(v.url)
        model = try nativeCopy(v.model)
        usage = try nativeCopy(v.usage)
        requests_sent = v.requests_sent
        cached = v.cached
        requests = try nativeCopy(v.requests)
        failed_questions = v.failed_questions
        profile_warning = try nativeCopy(v.profile_warning)
        batch_setting = try nativeCopy(v.batch_setting)
        batch_warning = try nativeCopy(v.batch_warning)
        context_sha256 = try nativeCopy(v.context_sha256)
        attempts = try nativeCopy(v.attempts)
        origin = try nativeCopy(v.origin)
        question_sources = try nativeCopy(v.question_sources)
        observations = try nativeCopy(v.observations)
        answered_by = try nativeCopy(v.answered_by)
    }
}
func nativeCopy(_ v: thinkthen_meta_v1) throws -> NativeMeta { try NativeMeta(v) }
public struct NativeName: Sendable {
    public let start: Int
    public let end: Int
    public let kinds: [NativeProbability]?
    public let edges: [NativeProbability]?
    init(_ v: thinkthen_name_v1) throws {
        start = v.start
        end = v.end
        kinds = try nativeCopy(v.kinds)
        edges = try nativeCopy(v.edges)
    }
}
func nativeCopy(_ v: thinkthen_name_v1) throws -> NativeName { try NativeName(v) }
public struct NativeNamedAnswer: Sendable {
    public let pick: String
    public let probabilities: [NativeProbability]
    public let confidence: Double?
    init(_ v: thinkthen_named_answer_v1) throws {
        pick = try nativeCopy(v.pick)
        probabilities = try nativeCopy(v.probabilities)
        confidence = try nativeCopy(v.confidence)
    }
}
func nativeCopy(_ v: thinkthen_named_answer_v1) throws -> NativeNamedAnswer { try NativeNamedAnswer(v) }
func nativeCopy(_ v: thinkthen_names_v1) throws -> [NativeName] { try nativeExtent(v.len,MemoryLayout<thinkthen_name_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
func nativeCopy(_ v: thinkthen_observation_identities_v1) throws -> [NativeObservationIdentity] { try nativeExtent(v.len,MemoryLayout<thinkthen_observation_identity_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeObservationIdentity: Sendable {
    public let kind: UInt32
    public struct Values: Sendable {
        public var observation_id: String? = nil
        public var failure_id: String? = nil
    }
    public let data: Values
    init(_ v: thinkthen_observation_identity_v1) throws {
        kind = v.kind
        var values = Values()
        switch v.kind {
        case 1: values.observation_id = try nativeCopy(v.data.observation_id)
        case 2: values.failure_id = try nativeCopy(v.data.failure_id)
        default: throw NativeConversion.invalidDiscriminator
        }
        data = values
    }
}
func nativeCopy(_ v: thinkthen_observation_identity_v1) throws -> NativeObservationIdentity { try NativeObservationIdentity(v) }
public struct NativeObservationSuccess: Sendable {
    public let answer_id: String
    public let observation_id: String
    public let value: NativeMemberValue
    public let probabilities: NativeObservedProbabilities
    public let confidence: Double?
    init(_ v: thinkthen_observation_success_v1) throws {
        answer_id = try nativeCopy(v.answer_id)
        observation_id = try nativeCopy(v.observation_id)
        value = try nativeCopy(v.value)
        probabilities = try nativeCopy(v.probabilities)
        confidence = try nativeCopy(v.confidence)
    }
}
func nativeCopy(_ v: thinkthen_observation_success_v1) throws -> NativeObservationSuccess { try NativeObservationSuccess(v) }
public struct NativeObservation: Sendable {
    public let kind: UInt32
    public struct Values: Sendable {
        public var question: NativeQuestionObservation? = nil
        public var row: NativeRowObservation? = nil
    }
    public let data: Values
    init(_ v: thinkthen_observation_v1) throws {
        kind = v.kind
        var values = Values()
        switch v.kind {
        case 1: values.question = try nativeCopy(v.data.question)
        case 2: values.row = try nativeCopy(v.data.row)
        default: throw NativeConversion.invalidDiscriminator
        }
        data = values
    }
}
func nativeCopy(_ v: thinkthen_observation_v1) throws -> NativeObservation { try NativeObservation(v) }
public struct NativeObservedProbabilities: Sendable {
    public let kind: UInt32
    public struct Values: Sendable {
        public var yes: Double? = nil
        public var named: [NativeProbability]? = nil
    }
    public let data: Values
    init(_ v: thinkthen_observed_probabilities_v1) throws {
        kind = v.kind
        var values = Values()
        switch v.kind {
        case 1: values.yes = v.data.yes
        case 2: values.named = try nativeCopy(v.data.named)
        default: throw NativeConversion.invalidDiscriminator
        }
        data = values
    }
}
func nativeCopy(_ v: thinkthen_observed_probabilities_v1) throws -> NativeObservedProbabilities { try NativeObservedProbabilities(v) }
func nativeCopy(_ v: thinkthen_optional_answer_v1) throws -> NativeAnswer? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_attempts_v1) throws -> [NativeAttempt]? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_batch_v1) throws -> NativeBatch? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_batch_warning_v1) throws -> NativeBatchWarning? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_content_v1) throws -> NativeContent? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_discriminator_v1) throws -> UInt32? { if try !nativePresent(v.present) { return nil }; return v.value }
func nativeCopy(_ v: thinkthen_optional_double_v1) throws -> Double? { if try !nativePresent(v.present) { return nil }; return v.value }
func nativeCopy(_ v: thinkthen_optional_endpoint_v1) throws -> NativeEndpoint? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_entity_edges_v1) throws -> [NativeEntityEdge]? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_error_v1) throws -> NativeError? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_facts_v1) throws -> NativeFacts? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_image_views_v1) throws -> [NativeImageView]? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_location_v1) throws -> NativeLocation? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_meta_v1) throws -> NativeMeta? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_probabilities_v1) throws -> [NativeProbability]? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_profile_warning_v1) throws -> NativeProfileWarning? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_question_v1) throws -> NativeQuestionView? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_rule_v1) throws -> NativeRule? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_size_v1) throws -> Int? { if try !nativePresent(v.present) { return nil }; return v.value }
func nativeCopy(_ v: thinkthen_optional_source_entity_edges_v1) throws -> [NativeSourceEntityEdge]? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_stopped_v1) throws -> NativeStopped? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_string_v1) throws -> String? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_u16_v1) throws -> UInt16? { if try !nativePresent(v.present) { return nil }; return v.value }
func nativeCopy(_ v: thinkthen_optional_u64_v1) throws -> UInt64? { if try !nativePresent(v.present) { return nil }; return v.value }
func nativeCopy(_ v: thinkthen_optional_usage_v1) throws -> NativeUsage? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
public struct NativePair: Sendable {
    public let relation: String
    public let source: NativePlace
    public let target: NativePlace
    public let probability: Double
    init(_ v: thinkthen_pair_v1) throws {
        relation = try nativeCopy(v.relation)
        source = try nativeCopy(v.source)
        target = try nativeCopy(v.target)
        probability = v.probability
    }
}
func nativeCopy(_ v: thinkthen_pair_v1) throws -> NativePair { try NativePair(v) }
func nativeCopy(_ v: thinkthen_pairs_v1) throws -> [NativePair] { try nativeExtent(v.len,MemoryLayout<thinkthen_pair_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativePiece: Sendable {
    public let start: Int
    public let end: Int
    public let tags: [NativeProbability]
    init(_ v: thinkthen_piece_v1) throws {
        start = v.start
        end = v.end
        tags = try nativeCopy(v.tags)
    }
}
func nativeCopy(_ v: thinkthen_piece_v1) throws -> NativePiece { try NativePiece(v) }
func nativeCopy(_ v: thinkthen_pieces_v1) throws -> [NativePiece] { try nativeExtent(v.len,MemoryLayout<thinkthen_piece_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativePlace: Sendable {
    public let start: Int
    public let end: Int
    init(_ v: thinkthen_place_v1) throws {
        start = v.start
        end = v.end
    }
}
func nativeCopy(_ v: thinkthen_place_v1) throws -> NativePlace { try NativePlace(v) }
func nativeCopy(_ v: thinkthen_probabilities_v1) throws -> [NativeProbability] { try nativeExtent(v.len,MemoryLayout<thinkthen_probability_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeProbability: Sendable {
    public let name: String
    public let probability: Double
    init(_ v: thinkthen_probability_v1) throws {
        name = try nativeCopy(v.name)
        probability = v.probability
    }
}
func nativeCopy(_ v: thinkthen_probability_v1) throws -> NativeProbability { try NativeProbability(v) }
public struct NativeProfileWarning: Sendable {
    public let tuned_for: String
    public let running: String
    init(_ v: thinkthen_profile_warning_v1) throws {
        tuned_for = try nativeCopy(v.tuned_for)
        running = try nativeCopy(v.running)
    }
}
func nativeCopy(_ v: thinkthen_profile_warning_v1) throws -> NativeProfileWarning { try NativeProfileWarning(v) }
public struct NativeQuestionAuthor: Sendable {
    public let name: String?
    public let wording_version: UInt64?
    public let item_schema: NativeInputDeclaration
    public let context_schema: NativeInputDeclaration
    init(_ v: thinkthen_question_author_v1) throws {
        name = try nativeCopy(v.name)
        wording_version = try nativeCopy(v.wording_version)
        item_schema = try nativeCopy(v.item_schema)
        context_schema = try nativeCopy(v.context_schema)
    }
}
func nativeCopy(_ v: thinkthen_question_author_v1) throws -> NativeQuestionAuthor { try NativeQuestionAuthor(v) }
public struct NativeQuestionMember: Sendable {
    public let name: String
    public let question: NativeQuestionView
    init(_ v: thinkthen_question_member_v1) throws {
        name = try nativeCopy(v.name)
        guard let pointer = v.question else { throw NativeConversion.invalidExtent }
        question = try nativeCopy(pointer.pointee)
    }
}
func nativeCopy(_ v: thinkthen_question_member_v1) throws -> NativeQuestionMember { try NativeQuestionMember(v) }
func nativeCopy(_ v: thinkthen_question_members_v1) throws -> [NativeQuestionMember] { try nativeExtent(v.len,MemoryLayout<thinkthen_question_member_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeQuestionObservation: Sendable {
    public let index: Int
    public let member: String?
    public let stage: UInt32?
    public let position: Int
    public let question_sha256: String
    public let model: String
    public let url: String
    public let requests: [String]
    public let requests_sent: UInt64
    public let cached: Int32
    public let failed_questions: Int
    public let usage: NativeUsage?
    public let question_sources: [NativeQuestionSource]
    public let state: UInt32
    public struct Values: Sendable {
        public var success: NativeObservationSuccess? = nil
        public var failure: NativeMemberFailure? = nil
    }
    public let data: Values
    init(_ v: thinkthen_question_observation_v1) throws {
        index = v.index
        member = try nativeCopy(v.member)
        stage = try nativeCopy(v.stage)
        position = v.position
        question_sha256 = try nativeCopy(v.question_sha256)
        model = try nativeCopy(v.model)
        url = try nativeCopy(v.url)
        requests = try nativeCopy(v.requests)
        requests_sent = v.requests_sent
        cached = v.cached
        failed_questions = v.failed_questions
        usage = try nativeCopy(v.usage)
        question_sources = try nativeCopy(v.question_sources)
        state = v.state
        var values = Values()
        switch v.state {
        case 1: values.success = try nativeCopy(v.data.success)
        case 2: values.failure = try nativeCopy(v.data.failure)
        default: throw NativeConversion.invalidDiscriminator
        }
        data = values
    }
}
func nativeCopy(_ v: thinkthen_question_observation_v1) throws -> NativeQuestionObservation { try NativeQuestionObservation(v) }
public struct NativeQuestionSource: Sendable {
    public let origin: UInt32
    public let answered_by: String
    init(_ v: thinkthen_question_source_v1) throws {
        origin = v.origin
        answered_by = try nativeCopy(v.answered_by)
    }
}
func nativeCopy(_ v: thinkthen_question_source_v1) throws -> NativeQuestionSource { try NativeQuestionSource(v) }
func nativeCopy(_ v: thinkthen_question_sources_v1) throws -> [NativeQuestionSource] { try nativeExtent(v.len,MemoryLayout<thinkthen_question_source_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public final class NativeQuestionView: Sendable {
    public let kind: UInt32
    public let text: NativeContent
    public let yes: NativeContent?
    public let no: NativeContent?
    public let choices: [NativeChoice]
    public let threshold: NativeRule
    public let relation_threshold: NativeRule
    public let model: String?
    public let profile: String?
    public let batch: Int?
    public let batch_max: Int32
    public let none: Int32
    public let on: [String]
    public let members: [NativeQuestionMember]
    public let kinds: [NativeChoice]
    public let relations: [NativeRelation]
    public let name_pointer: String?
    public let kind_pointer: String?
    init(_ v: thinkthen_question_view_v1) throws {
        kind = v.kind
        text = try nativeCopy(v.text)
        yes = try nativeCopy(v.yes)
        no = try nativeCopy(v.no)
        choices = try nativeCopy(v.choices)
        threshold = try nativeCopy(v.threshold)
        relation_threshold = try nativeCopy(v.relation_threshold)
        model = try nativeCopy(v.model)
        profile = try nativeCopy(v.profile)
        batch = try nativeCopy(v.batch)
        batch_max = v.batch_max
        none = v.none
        on = try nativeCopy(v.on)
        members = try nativeCopy(v.members)
        kinds = try nativeCopy(v.kinds)
        relations = try nativeCopy(v.relations)
        name_pointer = try nativeCopy(v.name_pointer)
        kind_pointer = try nativeCopy(v.kind_pointer)
    }
}
func nativeCopy(_ v: thinkthen_question_view_v1) throws -> NativeQuestionView { try NativeQuestionView(v) }
public struct NativeRankView: Sendable {
    public let common: NativeRow
    public let value: Int?
    public let question_name: String?
    init(_ v: thinkthen_rank_view_v1) throws {
        common = try nativeCopy(v.common)
        value = try nativeCopy(v.value)
        question_name = try nativeCopy(v.question_name)
    }
}
func nativeCopy(_ v: thinkthen_rank_view_v1) throws -> NativeRankView { try NativeRankView(v) }
public struct NativeRecognizeAnswer: Sendable {
    public let pieces: [NativePiece]
    public let names: [NativeName]
    public let pairs: [NativePair]
    init(_ v: thinkthen_recognize_answer_v1) throws {
        pieces = try nativeCopy(v.pieces)
        names = try nativeCopy(v.names)
        pairs = try nativeCopy(v.pairs)
    }
}
func nativeCopy(_ v: thinkthen_recognize_answer_v1) throws -> NativeRecognizeAnswer { try NativeRecognizeAnswer(v) }
public struct NativeRecognizeValue: Sendable {
    public let entities: [NativeEntity]
    public let relations: [NativeEntityEdge]?
    init(_ v: thinkthen_recognize_value_v1) throws {
        entities = try nativeCopy(v.entities)
        relations = try nativeCopy(v.relations)
    }
}
func nativeCopy(_ v: thinkthen_recognize_value_v1) throws -> NativeRecognizeValue { try NativeRecognizeValue(v) }
public struct NativeRecognizeView: Sendable {
    public let common: NativeRow
    public let value: NativeRecognizeValue
    public let answer: NativeRecognizeAnswer
    init(_ v: thinkthen_recognize_view_v1) throws {
        common = try nativeCopy(v.common)
        value = try nativeCopy(v.value)
        answer = try nativeCopy(v.answer)
    }
}
func nativeCopy(_ v: thinkthen_recognize_view_v1) throws -> NativeRecognizeView { try NativeRecognizeView(v) }
public struct NativeRelateView: Sendable {
    public let common: NativeRow
    public let value: [NativeEdge]
    public let questions: [NativeRelationAnswer]
    init(_ v: thinkthen_relate_view_v1) throws {
        common = try nativeCopy(v.common)
        value = try nativeCopy(v.value)
        questions = try nativeCopy(v.questions)
    }
}
func nativeCopy(_ v: thinkthen_relate_view_v1) throws -> NativeRelateView { try NativeRelateView(v) }
public struct NativeRelationAnswer: Sendable {
    public let relation: String
    public let reads: String
    public let method: UInt32
    public let direction: UInt32
    public let source: NativeEndpoint
    public let target: NativeEndpoint?
    public let request: String
    public let state: UInt32
    public struct Values: Sendable {
        public var success: NativeRelationSuccess? = nil
        public var failure: NativeMemberFailure? = nil
    }
    public let data: Values
    init(_ v: thinkthen_relation_answer_v1) throws {
        relation = try nativeCopy(v.relation)
        reads = try nativeCopy(v.reads)
        method = v.method
        direction = v.direction
        source = try nativeCopy(v.source)
        target = try nativeCopy(v.target)
        request = try nativeCopy(v.request)
        state = v.state
        var values = Values()
        switch v.state {
        case 1: values.success = try nativeCopy(v.data.success)
        case 2: values.failure = try nativeCopy(v.data.failure)
        default: throw NativeConversion.invalidDiscriminator
        }
        data = values
    }
}
func nativeCopy(_ v: thinkthen_relation_answer_v1) throws -> NativeRelationAnswer { try NativeRelationAnswer(v) }
func nativeCopy(_ v: thinkthen_relation_answers_v1) throws -> [NativeRelationAnswer] { try nativeExtent(v.len,MemoryLayout<thinkthen_relation_answer_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeRelationSuccess: Sendable {
    public let answer_id: String
    public let probability: Double
    public let accepted: Int32
    init(_ v: thinkthen_relation_success_v1) throws {
        answer_id = try nativeCopy(v.answer_id)
        probability = v.probability
        accepted = v.accepted
    }
}
func nativeCopy(_ v: thinkthen_relation_success_v1) throws -> NativeRelationSuccess { try NativeRelationSuccess(v) }
public struct NativeRelation: Sendable {
    public let name: String
    public let source: String
    public let target: String
    public let reads: String?
    public let either: Int32
    public let single: Int32
    init(_ v: thinkthen_relation_v1) throws {
        name = try nativeCopy(v.name)
        source = try nativeCopy(v.source)
        target = try nativeCopy(v.target)
        reads = try nativeCopy(v.reads)
        either = v.either
        single = v.single
    }
}
func nativeCopy(_ v: thinkthen_relation_v1) throws -> NativeRelation { try NativeRelation(v) }
func nativeCopy(_ v: thinkthen_relations_v1) throws -> [NativeRelation] { try nativeExtent(v.len,MemoryLayout<thinkthen_relation_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeReportedUsage: Sendable {
    public let present: Int32
    public let input_tokens: UInt64?
    public let output_tokens: UInt64?
    init(_ v: thinkthen_reported_usage_v1) throws {
        present = v.present
        input_tokens = try nativeCopy(v.input_tokens)
        output_tokens = try nativeCopy(v.output_tokens)
    }
}
func nativeCopy(_ v: thinkthen_reported_usage_v1) throws -> NativeReportedUsage { try NativeReportedUsage(v) }
public struct NativeRowObservation: Sendable {
    public let index: Int
    public let function: UInt32
    public struct Values: Sendable {
        public var decide: NativeDecideView? = nil
        public var choose: NativeChooseView? = nil
        public var tag: NativeTagView? = nil
        public var score: NativeScoreView? = nil
        public var filter: NativeFilterView? = nil
        public var rank: NativeRankView? = nil
        public var find: NativeFindView? = nil
        public var annotate: NativeAnnotateView? = nil
        public var recognize: NativeRecognizeView? = nil
        public var relate: NativeRelateView? = nil
    }
    public let data: Values
    init(_ v: thinkthen_row_observation_v1) throws {
        index = v.index
        function = v.function
        var values = Values()
        switch v.function {
        case 1: values.decide = try nativeCopy(v.data.decide)
        case 2: values.choose = try nativeCopy(v.data.choose)
        case 3: values.tag = try nativeCopy(v.data.tag)
        case 4: values.score = try nativeCopy(v.data.score)
        case 5: values.filter = try nativeCopy(v.data.filter)
        case 6: values.rank = try nativeCopy(v.data.rank)
        case 7: values.find = try nativeCopy(v.data.find)
        case 8: values.annotate = try nativeCopy(v.data.annotate)
        case 9: values.recognize = try nativeCopy(v.data.recognize)
        case 10: values.relate = try nativeCopy(v.data.relate)
        default: throw NativeConversion.invalidDiscriminator
        }
        data = values
    }
}
func nativeCopy(_ v: thinkthen_row_observation_v1) throws -> NativeRowObservation { try NativeRowObservation(v) }
public struct NativeRow: Sendable {
    public let answer_id: String
    public let input: NativeContent?
    public let question: NativeQuestionView?
    public let answer: NativeAnswer?
    public let threshold: NativeRule?
    public let position: NativeLocation?
    public let input_file: String?
    public let meta: NativeMeta
    public let images: [NativeImageView]?
    init(_ v: thinkthen_row_v1) throws {
        answer_id = try nativeCopy(v.answer_id)
        input = try nativeCopy(v.input)
        question = try nativeCopy(v.question)
        answer = try nativeCopy(v.answer)
        threshold = try nativeCopy(v.threshold)
        position = try nativeCopy(v.position)
        input_file = try nativeCopy(v.input_file)
        meta = try nativeCopy(v.meta)
        images = try nativeCopy(v.images)
    }
}
func nativeCopy(_ v: thinkthen_row_v1) throws -> NativeRow { try NativeRow(v) }
public struct NativeRule: Sendable {
    public let kind: UInt32
    public let low: Double
    public let high: Double
    init(_ v: thinkthen_rule_v1) throws {
        kind = v.kind
        low = v.low
        high = v.high
    }
}
func nativeCopy(_ v: thinkthen_rule_v1) throws -> NativeRule { try NativeRule(v) }
public struct NativeScoreAnswer: Sendable {
    public let level: String
    public let probabilities: [NativeProbability]
    public let confidence: Double?
    init(_ v: thinkthen_score_answer_v1) throws {
        level = try nativeCopy(v.level)
        probabilities = try nativeCopy(v.probabilities)
        confidence = try nativeCopy(v.confidence)
    }
}
func nativeCopy(_ v: thinkthen_score_answer_v1) throws -> NativeScoreAnswer { try NativeScoreAnswer(v) }
public struct NativeScoreView: Sendable {
    public let common: NativeRow
    public let value: Double
    init(_ v: thinkthen_score_view_v1) throws {
        common = try nativeCopy(v.common)
        value = v.value
    }
}
func nativeCopy(_ v: thinkthen_score_view_v1) throws -> NativeScoreView { try NativeScoreView(v) }
public struct NativeSourceDetail: Sendable {
    public let origin: UInt32
    public let answered_by: String
    public let batch_size: Int?
    init(_ v: thinkthen_source_detail_v1) throws {
        origin = v.origin
        answered_by = try nativeCopy(v.answered_by)
        batch_size = try nativeCopy(v.batch_size)
    }
}
func nativeCopy(_ v: thinkthen_source_detail_v1) throws -> NativeSourceDetail { try NativeSourceDetail(v) }
func nativeCopy(_ v: thinkthen_source_details_v1) throws -> [NativeSourceDetail] { try nativeExtent(v.len,MemoryLayout<thinkthen_source_detail_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeSourceEdge: Sendable {
    public let relation: String
    public let source: NativeSourceEndpoint
    public let target: NativeSourceEndpoint
    public let probability: Double
    public let either: Int32
    init(_ v: thinkthen_source_edge_v1) throws {
        relation = try nativeCopy(v.relation)
        source = try nativeCopy(v.source)
        target = try nativeCopy(v.target)
        probability = v.probability
        either = v.either
    }
}
func nativeCopy(_ v: thinkthen_source_edge_v1) throws -> NativeSourceEdge { try NativeSourceEdge(v) }
func nativeCopy(_ v: thinkthen_source_edges_v1) throws -> [NativeSourceEdge] { try nativeExtent(v.len,MemoryLayout<thinkthen_source_edge_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeSourceEndpoint: Sendable {
    public let ordinal: Int
    public let endpoint: NativeEndpoint
    public let record: NativeContent
    public let position: NativeLocation?
    init(_ v: thinkthen_source_endpoint_v1) throws {
        ordinal = v.ordinal
        endpoint = try nativeCopy(v.endpoint)
        record = try nativeCopy(v.record)
        position = try nativeCopy(v.position)
    }
}
func nativeCopy(_ v: thinkthen_source_endpoint_v1) throws -> NativeSourceEndpoint { try NativeSourceEndpoint(v) }
func nativeCopy(_ v: thinkthen_source_entities_v1) throws -> [NativeSourceEntity] { try nativeExtent(v.len,MemoryLayout<thinkthen_source_entity_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeSourceEntityEdge: Sendable {
    public let relation: String
    public let source: NativeSourceEntity
    public let target: NativeSourceEntity
    public let probability: Double
    public let either: Int32
    init(_ v: thinkthen_source_entity_edge_v1) throws {
        relation = try nativeCopy(v.relation)
        source = try nativeCopy(v.source)
        target = try nativeCopy(v.target)
        probability = v.probability
        either = v.either
    }
}
func nativeCopy(_ v: thinkthen_source_entity_edge_v1) throws -> NativeSourceEntityEdge { try NativeSourceEntityEdge(v) }
func nativeCopy(_ v: thinkthen_source_entity_edges_v1) throws -> [NativeSourceEntityEdge] { try nativeExtent(v.len,MemoryLayout<thinkthen_source_entity_edge_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeSourceEntity: Sendable {
    public let entity: NativeEntity
    public let position: NativeLocation?
    init(_ v: thinkthen_source_entity_v1) throws {
        entity = try nativeCopy(v.entity)
        position = try nativeCopy(v.position)
    }
}
func nativeCopy(_ v: thinkthen_source_entity_v1) throws -> NativeSourceEntity { try NativeSourceEntity(v) }
public struct NativeSourceRecognition: Sendable {
    public let present: Int32
    public let entities: [NativeSourceEntity]
    public let relations: [NativeSourceEntityEdge]?
    init(_ v: thinkthen_source_recognition_v1) throws {
        present = v.present
        entities = try nativeCopy(v.entities)
        relations = try nativeCopy(v.relations)
    }
}
func nativeCopy(_ v: thinkthen_source_recognition_v1) throws -> NativeSourceRecognition { try NativeSourceRecognition(v) }
public struct NativeSourceRelations: Sendable {
    public let present: Int32
    public let edges: [NativeSourceEdge]
    init(_ v: thinkthen_source_relations_v1) throws {
        present = v.present
        edges = try nativeCopy(v.edges)
    }
}
func nativeCopy(_ v: thinkthen_source_relations_v1) throws -> NativeSourceRelations { try NativeSourceRelations(v) }
public struct NativeStopped: Sendable {
    public let at: Int?
    public let cause: UInt32
    public let status: UInt16?
    public let retryable: Int32
    init(_ v: thinkthen_stopped_v1) throws {
        at = try nativeCopy(v.at)
        cause = v.cause
        status = try nativeCopy(v.status)
        retryable = v.retryable
    }
}
func nativeCopy(_ v: thinkthen_stopped_v1) throws -> NativeStopped { try NativeStopped(v) }
func nativeCopy(_ v: thinkthen_strings_v1) throws -> [String] { try nativeExtent(v.len,MemoryLayout<thinkthen_string_v1>.stride,v.data); if v.len == 0 { return [] }; return try UnsafeBufferPointer(start:v.data,count:v.len).map { try nativeCopy($0) } }
public struct NativeSummary: Sendable {
    public let state: UInt32
    public let schema: String
    public let answer_id: String?
    public let function: UInt32?
    public let count: Int
    public let observation_count: Int
    public let meta: NativeMeta?
    public let facts: NativeFacts?
    public let attempts: [NativeAttempt]?
    public let error: NativeError?
    init(_ v: thinkthen_summary_v1) throws {
        state = v.state
        schema = try nativeCopy(v.schema)
        answer_id = try nativeCopy(v.answer_id)
        function = try nativeCopy(v.function)
        count = v.count
        observation_count = v.observation_count
        meta = try nativeCopy(v.meta)
        facts = try nativeCopy(v.facts)
        attempts = try nativeCopy(v.attempts)
        error = try nativeCopy(v.error)
    }
}
func nativeCopy(_ v: thinkthen_summary_v1) throws -> NativeSummary { try NativeSummary(v) }
public struct NativeTagView: Sendable {
    public let common: NativeRow
    public let value: [String]
    init(_ v: thinkthen_tag_view_v1) throws {
        common = try nativeCopy(v.common)
        value = try nativeCopy(v.value)
    }
}
func nativeCopy(_ v: thinkthen_tag_view_v1) throws -> NativeTagView { try NativeTagView(v) }
public struct NativeUsage: Sendable {
    public let input_tokens: UInt64
    public let output_tokens: UInt64
    init(_ v: thinkthen_usage_v1) throws {
        input_tokens = v.input_tokens
        output_tokens = v.output_tokens
    }
}
func nativeCopy(_ v: thinkthen_usage_v1) throws -> NativeUsage { try NativeUsage(v) }
