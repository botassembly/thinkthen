import Foundation
import CThinkThen
#if os(Linux)
import Glibc
#else
import Darwin
#endif

public enum NativeRole: UInt32 { case atomic = 1, set, dynamicChoose, recognize, relate, rank, rankSet, find }
public enum NativeUnit: UInt32 { case line = 1, window, file, imageFile, jsonl }
public enum NativeInput { case text(String), json(String) }
// Stable call-scoped buffers for counted descriptors. Native constructors clone them.
public final class NativeBuffers {
    private var releases: [() -> Void] = []
    public init() {}
    public func array<T>(_ values: [T]) -> UnsafePointer<T>? {
        if values.isEmpty { return nil }
        let p = UnsafeMutablePointer<T>.allocate(capacity: values.count)
        p.initialize(from: values, count: values.count)
        releases.append { p.deinitialize(count:values.count); p.deallocate() }
        return UnsafePointer(p)
    }
    public func string(_ value: String) -> thinkthen_string_v1 {
        let bytes = Array(value.utf8).map { CChar(bitPattern:$0) }
        return thinkthen_string_v1(data:array(bytes),len:bytes.count)
    }
    public func strings(_ values: [String]) -> thinkthen_strings_v1 {
        let bytes = values.map(string)
        return thinkthen_strings_v1(data:array(bytes),len:bytes.count)
    }
    public func content(_ value: NativeInput) -> thinkthen_content_v1 {
        switch value {
        case .text(let text): return thinkthen_content_v1(kind:1,data:string(text))
        case .json(let json): return thinkthen_content_v1(kind:2,data:string(json))
        }
    }
    public func optional(_ value: NativeInput?) -> thinkthen_optional_content_v1 {
        guard let value else { return thinkthen_optional_content_v1() }
        return thinkthen_optional_content_v1(present:1,value:content(value))
    }
    deinit { for release in releases.reversed() { release() } }
}
public final class NativeQuestion {
    let handle: OpaquePointer
    init(_ handle: OpaquePointer) { self.handle = handle }
    deinit { thinkthen_question_free(handle) }
    public func author() throws -> NativeQuestionAuthor {
        var value = thinkthen_question_author_v1()
        try nativeViewOK(thinkthen_question_author(handle,&value))
        return try nativeCopy(value)
    }
}
public final class NativeImage {
    let handle: OpaquePointer
    init(_ handle: OpaquePointer) { self.handle = handle }
    deinit { thinkthen_image_free(handle) }
    public func view() throws -> NativeImageView {
        var value = thinkthen_image_view_v1(); try nativeViewOK(thinkthen_image_view(handle,&value))
        return try nativeCopy(value)
    }
}
public final class NativeSource {
    let handle: OpaquePointer
    init(_ handle: OpaquePointer) { self.handle = handle }
    deinit { thinkthen_source_free(handle) }
}
public struct NativeRecord {
    public var original: NativeInput?
    public var context: NativeInput?
    public var options: [thinkthen_choice_v1]
    public var images: [NativeImage]
    // Option descriptors borrow caller buffers through source construction only.
    public init(original: NativeInput?, context: NativeInput? = nil, options: [thinkthen_choice_v1] = [], images: [NativeImage] = []) {
        self.original = original; self.context = context; self.options = options; self.images = images
    }
}
public struct NativeControls {
    public var deadline: Int64 = -1
    public var cancel: CancelToken? = nil
    public var context: NativeInput? = nil
    public var batch: Int? = nil
    public var batchMax: Bool = false
    public var attempts: Bool = false
    public init() {}
    func descriptor(_ buffers: NativeBuffers) -> thinkthen_controls_v1 {
        var c = thinkthen_controls_v1(); c.deadline_ms = deadline; c.cancel = cancel?.handle
        c.context = buffers.optional(context)
        if let batch { c.batch = thinkthen_optional_size_v1(present:1,value:batch) }
        c.batch_max = batchMax ? 1 : 0; c.attempts = attempts ? 1 : 0; c.surface = buffers.string("swift")
        return c
    }
}
public struct NativeFailure: Error, CustomStringConvertible {
    public let snapshot: NativeResult
    public var code: Int32 { snapshot.summary.error?.code ?? 6 }
    public var kind: FailureKind { FailureKind(rawValue:code) ?? .defect }
    public var description: String { snapshot.summary.error?.message ?? "missing native failure" }
}
func nativeViewOK(_ code: Int32) throws { if code != 0 { throw NativeConversion.invalidDiscriminator } }
func nativeChecked(_ engine: OpaquePointer?, _ code: Int32) throws {
    if code == 0 { return }
    var raw: OpaquePointer? = nil; try nativeViewOK(thinkthen_error_complete(engine,&raw))
    guard let raw else { throw NativeConversion.missingResult }
    throw try NativeFailure(snapshot:NativeResult(taking:raw))
}

public enum UsagePersistence: Sendable {
    case disabled, pending, written, failed
    init(_ kind: UInt32) throws {
        switch kind {
        case UInt32(THINKTHEN_COMPLETE_USAGE_PERSISTENCE_DISABLED_V1): self = .disabled
        case UInt32(THINKTHEN_COMPLETE_USAGE_PERSISTENCE_PENDING_V1): self = .pending
        case UInt32(THINKTHEN_COMPLETE_USAGE_PERSISTENCE_WRITTEN_V1): self = .written
        case UInt32(THINKTHEN_COMPLETE_USAGE_PERSISTENCE_FAILED_V1): self = .failed
        default: throw NativeConversion.invalidDiscriminator
        }
    }
}
public struct UsageStatus: Sendable {
    public let state: UsagePersistence
    public let advice: String?
}

extension Engine {
    public func usagePersistence() throws -> UsageStatus { try usageStatus(thinkthen_engine_usage_persistence_v1) }
    public func finishUsageStatus() throws -> UsageStatus { try usageStatus(thinkthen_engine_finish_usage_status_v1) }
    private func usageStatus(_ operation: (OpaquePointer?, UnsafeMutablePointer<thinkthen_complete_usage_persistence_v1>?, UnsafeMutablePointer<thinkthen_complete_utf8_v1>?) -> Int32) throws -> UsageStatus {
        let h = try open(); var state = thinkthen_complete_usage_persistence_v1(); var advice = thinkthen_complete_utf8_v1()
        try nativeChecked(h,operation(h,&state,&advice))
        let copied = try nativeCopy(thinkthen_string_v1(data:advice.data,len:advice.len))
        return UsageStatus(state:try UsagePersistence(state.kind),advice:advice.data == nil ? nil : copied)
    }
    public func question(_ spec: thinkthen_question_spec_v1, author: thinkthen_question_author_v1? = nil, task: thinkthen_recognition_task_v1? = nil) throws -> NativeQuestion {
        let h = try open(); var spec = spec; let author = author; var out: OpaquePointer? = nil
        let code = withUnsafePointer(to:&spec) { s in
            let construct: (UnsafePointer<thinkthen_question_author_v1>?) -> Int32 = { a in
                if var task = task { return withUnsafePointer(to:&task) { thinkthen_question_new_recognition_v1(h,s,a,$0,&out) } }
                return thinkthen_question_new_authored(h,s,a,&out)
            }
            return author.map { value in var value = value; return withUnsafePointer(to:&value) { construct($0) } } ?? construct(nil)
        }
        try nativeChecked(h,code); guard let out else { throw NativeConversion.missingResult }; return NativeQuestion(out)
    }
    public func parseQuestion(role: NativeRole, json: String) throws -> NativeQuestion {
        let h = try open(); let buffers = NativeBuffers(); var out: OpaquePointer? = nil
        try withExtendedLifetime(buffers) { try nativeChecked(h,thinkthen_question_parse(h,role.rawValue,buffers.string(json),&out)) }
        guard let out else { throw NativeConversion.missingResult }; return NativeQuestion(out)
    }
    public func loadQuestion(path: String) throws -> NativeQuestion {
        let h = try open(); let buffers = NativeBuffers(); var out: OpaquePointer? = nil
        try withExtendedLifetime(buffers) { try nativeChecked(h,thinkthen_question_load(h,buffers.string(path),&out)) }
        guard let out else { throw NativeConversion.missingResult }; return NativeQuestion(out)
    }
    public func namedQuestion(role: NativeRole, name: String) throws -> NativeQuestion {
        let h = try open(); let buffers = NativeBuffers(); var out: OpaquePointer? = nil
        try withExtendedLifetime(buffers) { try nativeChecked(h,thinkthen_question_load_named(h,role.rawValue,buffers.string(name),&out)) }
        guard let out else { throw NativeConversion.missingResult }; return NativeQuestion(out)
    }
    public func referenceQuestion(role: NativeRole, reference: String) throws -> NativeQuestion {
        let h = try open(); let buffers = NativeBuffers(); var out: OpaquePointer? = nil
        try withExtendedLifetime(buffers) { try nativeChecked(h,thinkthen_question_load_reference(h,role.rawValue,buffers.string(reference),&out)) }
        guard let out else { throw NativeConversion.missingResult }; return NativeQuestion(out)
    }
    public func image(bytes: [UInt8], media: UInt32, filename: String? = nil) throws -> NativeImage {
        let h = try open(); let buffers = NativeBuffers(); var out: OpaquePointer? = nil
        var name = thinkthen_optional_string_v1(); if let filename { name = thinkthen_optional_string_v1(present:1,value:buffers.string(filename)) }
        try withExtendedLifetime(buffers) { try nativeChecked(h,thinkthen_image_clone(h,buffers.array(bytes),bytes.count,media,name,&out)) }
        guard let out else { throw NativeConversion.missingResult }; return NativeImage(out)
    }
    public func records(_ records: [NativeRecord]) throws -> NativeSource {
        let h = try open(); let buffers = NativeBuffers(); var out: OpaquePointer? = nil
        let values = records.map { row -> thinkthen_record_v1 in
            var v = thinkthen_record_v1(); v.original = buffers.optional(row.original); v.context = buffers.optional(row.context)
            v.options = thinkthen_choices_v1(data:buffers.array(row.options),len:row.options.count)
            let handles = row.images.map { Optional($0.handle) }
            v.images = thinkthen_images_v1(data:buffers.array(handles),len:handles.count); return v
        }
        try withExtendedLifetime((buffers,records)) { try nativeChecked(h,thinkthen_source_records(h,buffers.array(values),values.count,&out)) }
        guard let out else { throw NativeConversion.missingResult }; return NativeSource(out)
    }
    public func fileSource(paths: [String], unit: NativeUnit, window: Int = 0, imageReader: Bool = false) throws -> NativeSource {
        let h = try open(); let buffers = NativeBuffers(); var out: OpaquePointer? = nil
        var spec = thinkthen_source_spec_v1(paths:buffers.strings(paths),unit:unit.rawValue,window:window)
        try withExtendedLifetime(buffers) { try nativeChecked(h,imageReader ? thinkthen_source_image_files(h,&spec,&out) : thinkthen_source_files(h,&spec,&out)) }
        guard let out else { throw NativeConversion.missingResult }; return NativeSource(out)
    }
    func nativeCall(_ q: NativeQuestion, _ s: NativeSource, _ controls: NativeControls, _ call: (OpaquePointer?,OpaquePointer?,OpaquePointer?,UnsafePointer<thinkthen_controls_v1>?,UnsafeMutablePointer<OpaquePointer?>?) -> Int32) throws -> NativeResult {
        let h = try open(); let buffers = NativeBuffers(); var c = controls.descriptor(buffers); var out: OpaquePointer? = nil
        try withExtendedLifetime((buffers,q,s,controls.cancel)) { try nativeChecked(h,call(h,q.handle,s.handle,&c,&out)) }
        guard let out else { throw NativeConversion.missingResult }; return try NativeResult(taking:out)
    }
    public func decide(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeResult { try nativeCall(question,source,controls,thinkthen_decide_complete) }
    public func choose(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeResult { try nativeCall(question,source,controls,thinkthen_choose_complete) }
    public func tag(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeResult { try nativeCall(question,source,controls,thinkthen_tag_complete) }
    public func score(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeResult { try nativeCall(question,source,controls,thinkthen_score_complete) }
    public func filter(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeResult { try nativeCall(question,source,controls,thinkthen_filter_complete) }
    public func rank(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeResult { try nativeCall(question,source,controls,thinkthen_rank_complete) }
    public func find(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeResult { try nativeCall(question,source,controls,thinkthen_find_complete) }
    public func annotate(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeResult { try nativeCall(question,source,controls,thinkthen_annotate_complete) }
    public func recognize(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeResult { try nativeCall(question,source,controls,thinkthen_recognize_complete) }
    public func relate(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeResult { try nativeCall(question,source,controls,thinkthen_relate_complete) }
}

public final class NativeLazyBatch {
    private let engine: EngineOwner
    private let thread = pthread_self()
    private var handle: OpaquePointer?
    typealias Start = (OpaquePointer?,OpaquePointer?,OpaquePointer?,UnsafePointer<thinkthen_controls_v1>?,UnsafeMutablePointer<OpaquePointer?>?) -> Int32
    init(_ engine: Engine, _ question: NativeQuestion, _ source: NativeSource, _ controls: NativeControls, _ start: Start) throws {
        _ = try engine.open(); guard let owner = engine.owner else { throw NativeConversion.missingResult }; self.engine = owner
        let buffers = NativeBuffers(); var c = controls.descriptor(buffers); var out: OpaquePointer? = nil
        try withExtendedLifetime((buffers,question,source,controls.cancel)) { try nativeChecked(owner.handle,start(owner.handle,question.handle,source.handle,&c,&out)) }
        guard let out else { throw NativeConversion.missingResult }; handle = out
    }
    private func creatingThread() { precondition(pthread_equal(thread,pthread_self()) != 0,"batch belongs to its creating thread") }
    public func close() { creatingThread(); if let handle { thinkthen_batch_free(handle); self.handle = nil } }
    deinit { close() }
    public func next() throws -> NativeResult? {
        creatingThread(); guard let handle else { throw NativeConversion.missingResult }; var out: OpaquePointer? = nil
        try nativeChecked(engine.handle,thinkthen_batch_next(handle,&out)); return try out.map { try NativeResult(taking:$0) }
    }
    public func facts() throws -> NativeResult {
        creatingThread(); guard let handle else { throw NativeConversion.missingResult }; var out: OpaquePointer? = nil
        try nativeChecked(engine.handle,thinkthen_batch_facts(handle,&out)); guard let out else { throw NativeConversion.missingResult }; return try NativeResult(taking:out)
    }
}
extension Engine {
    public func decideBatch(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeLazyBatch { try NativeLazyBatch(self,question,source,controls,thinkthen_decide_batch_start) }
    public func chooseBatch(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeLazyBatch { try NativeLazyBatch(self,question,source,controls,thinkthen_choose_batch_start) }
    public func tagBatch(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeLazyBatch { try NativeLazyBatch(self,question,source,controls,thinkthen_tag_batch_start) }
    public func scoreBatch(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeLazyBatch { try NativeLazyBatch(self,question,source,controls,thinkthen_score_batch_start) }
    public func filterBatch(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeLazyBatch { try NativeLazyBatch(self,question,source,controls,thinkthen_filter_batch_start) }
    public func annotateBatch(_ question: NativeQuestion, source: NativeSource, controls: NativeControls = NativeControls()) throws -> NativeLazyBatch { try NativeLazyBatch(self,question,source,controls,thinkthen_annotate_batch_start) }
}
