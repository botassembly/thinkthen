import CThinkThen
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
public struct NativeContent: Sendable {
    public let kind: UInt32
    public let data: String
    init(_ v: thinkthen_content_v1) throws {
        kind = v.kind
        data = try nativeCopy(v.data)
    }
}
func nativeCopy(_ v: thinkthen_content_v1) throws -> NativeContent { try NativeContent(v) }
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
func nativeCopy(_ v: thinkthen_optional_content_v1) throws -> NativeContent? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_question_v1) throws -> NativeQuestionView? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
func nativeCopy(_ v: thinkthen_optional_rule_v1) throws -> NativeRule? { if try !nativePresent(v.present) { return nil }; return try nativeCopy(v.value) }
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
