import CThinkThen
public struct NativeResult: Sendable {
    public let summary: NativeSummary
    public let rows: [NativeRowObservation]
    public let observations: [NativeObservation]
    public let details: [NativeDetails]
    public let observationDetails: [NativeDetails]
    public let authors: [NativeQuestionAuthor]
    public let observationAuthors: [NativeQuestionAuthor]
    public let memberAuthors: [[NativeQuestionAuthor]]
    public let rankMembers: [[NativeRankView]]
    public let locatedRecognition: [NativeSourceRecognition]
    public let locatedRelations: [NativeSourceRelations]
    init(taking raw: OpaquePointer) throws {
        defer { thinkthen_result_free(raw) }
        var s = thinkthen_summary_v1(); try nativeViewOK(thinkthen_result_summary(raw,&s)); summary = try nativeCopy(s)
        try nativeExtent(s.count,MemoryLayout<NativeRowObservation>.stride,UnsafeRawPointer(raw))
        try nativeExtent(s.observation_count,MemoryLayout<NativeObservation>.stride,UnsafeRawPointer(raw))
        var events: [NativeObservation] = [], eventDetails: [NativeDetails] = [], eventAuthors: [NativeQuestionAuthor] = []
        for i in 0..<s.observation_count {
            var v = thinkthen_observation_v1(); try nativeViewOK(thinkthen_result_observation(raw,i,&v)); events.append(try nativeCopy(v))
            var d = thinkthen_details_v1(); try nativeViewOK(thinkthen_result_observation_details(raw,i,&d)); eventDetails.append(try nativeCopy(d))
            var a = thinkthen_question_author_v1(); try nativeViewOK(thinkthen_result_observation_author(raw,i,&a)); eventAuthors.append(try nativeCopy(a))
        }
        observations = events; observationDetails = eventDetails; observationAuthors = eventAuthors
        var values: [NativeRowObservation] = [], ds: [NativeDetails] = [], authors: [NativeQuestionAuthor] = []
        var mas: [[NativeQuestionAuthor]] = [], rms: [[NativeRankView]] = []
        var recs: [NativeSourceRecognition] = [], rels: [NativeSourceRelations] = []
        for i in 0..<s.count {
            var row = thinkthen_row_observation_v1(); row.function = s.function.value
            switch row.function {
            case 1: try nativeViewOK(thinkthen_result_decide(raw,i,&row.data.decide))
            case 2: try nativeViewOK(thinkthen_result_choose(raw,i,&row.data.choose))
            case 3: try nativeViewOK(thinkthen_result_tag(raw,i,&row.data.tag))
            case 4: try nativeViewOK(thinkthen_result_score(raw,i,&row.data.score))
            case 5: try nativeViewOK(thinkthen_result_filter(raw,i,&row.data.filter))
            case 6: try nativeViewOK(thinkthen_result_rank(raw,i,&row.data.rank))
            case 7: try nativeViewOK(thinkthen_result_find(raw,i,&row.data.find))
            case 8: try nativeViewOK(thinkthen_result_annotate(raw,i,&row.data.annotate))
            case 9: try nativeViewOK(thinkthen_result_recognize(raw,i,&row.data.recognize))
            case 10: try nativeViewOK(thinkthen_result_relate(raw,i,&row.data.relate))
            default: throw NativeConversion.invalidDiscriminator
            }
            values.append(try nativeCopy(row))
            var d = thinkthen_details_v1(); try nativeViewOK(thinkthen_result_details(raw,i,&d)); ds.append(try nativeCopy(d))
            var a = thinkthen_question_author_v1(); try nativeViewOK(thinkthen_result_question_author(raw,i,&a)); authors.append(try nativeCopy(a))
            var members: [NativeQuestionAuthor] = []
            if row.function == 8 { for j in 0..<row.data.annotate.answers.len {
                try nativeViewOK(thinkthen_result_member_author(raw,i,j,&a)); members.append(try nativeCopy(a))
            } }
            mas.append(members)
            var ranks: [NativeRankView] = []
            if row.function == 6 {
                var count = 0; try nativeViewOK(thinkthen_result_rank_member_count(raw,i,&count))
                try nativeExtent(count,MemoryLayout<NativeRankView>.stride,UnsafeRawPointer(raw))
                for j in 0..<count { var v = thinkthen_rank_view_v1(); try nativeViewOK(thinkthen_result_rank_member(raw,i,j,&v)); ranks.append(try nativeCopy(v)) }
            }
            rms.append(ranks)
            var rec = thinkthen_source_recognition_v1()
            if row.function == 9 { try nativeViewOK(thinkthen_result_source_recognition(raw,i,&rec)) }; recs.append(try nativeCopy(rec))
            var rel = thinkthen_source_relations_v1()
            if row.function == 10 { try nativeViewOK(thinkthen_result_source_relations(raw,i,&rel)) }; rels.append(try nativeCopy(rel))
        }
        rows = values; details = ds; self.authors = authors; memberAuthors = mas; rankMembers = rms
        locatedRecognition = recs; locatedRelations = rels
    }
    public func decide(_ index: Int) throws -> NativeDecideView {
        guard rows.indices.contains(index), let value = rows[index].data.decide else { throw NativeConversion.invalidDiscriminator }; return value
    }
    public func choose(_ index: Int) throws -> NativeChooseView {
        guard rows.indices.contains(index), let value = rows[index].data.choose else { throw NativeConversion.invalidDiscriminator }; return value
    }
    public func tag(_ index: Int) throws -> NativeTagView {
        guard rows.indices.contains(index), let value = rows[index].data.tag else { throw NativeConversion.invalidDiscriminator }; return value
    }
    public func score(_ index: Int) throws -> NativeScoreView {
        guard rows.indices.contains(index), let value = rows[index].data.score else { throw NativeConversion.invalidDiscriminator }; return value
    }
    public func filter(_ index: Int) throws -> NativeFilterView {
        guard rows.indices.contains(index), let value = rows[index].data.filter else { throw NativeConversion.invalidDiscriminator }; return value
    }
    public func rank(_ index: Int) throws -> NativeRankView {
        guard rows.indices.contains(index), let value = rows[index].data.rank else { throw NativeConversion.invalidDiscriminator }; return value
    }
    public func find(_ index: Int) throws -> NativeFindView {
        guard rows.indices.contains(index), let value = rows[index].data.find else { throw NativeConversion.invalidDiscriminator }; return value
    }
    public func annotate(_ index: Int) throws -> NativeAnnotateView {
        guard rows.indices.contains(index), let value = rows[index].data.annotate else { throw NativeConversion.invalidDiscriminator }; return value
    }
    public func recognize(_ index: Int) throws -> NativeRecognizeView {
        guard rows.indices.contains(index), let value = rows[index].data.recognize else { throw NativeConversion.invalidDiscriminator }; return value
    }
    public func relate(_ index: Int) throws -> NativeRelateView {
        guard rows.indices.contains(index), let value = rows[index].data.relate else { throw NativeConversion.invalidDiscriminator }; return value
    }
}
