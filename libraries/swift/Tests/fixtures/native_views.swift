import Foundation
extension NativeAttempt: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(ordinal,forKey:NativeKey("ordinal"))
        try c.encode(request_sha256,forKey:NativeKey("request_sha256"))
        try c.encode(wall_ms,forKey:NativeKey("wall_ms"))
        try c.encode(outcome,forKey:NativeKey("outcome"))
        try c.encode(sdk_request_id,forKey:NativeKey("sdk_request_id"))
        try c.encode(status,forKey:NativeKey("status"))
        try c.encode(server_ms,forKey:NativeKey("server_ms"))
        try c.encode(request_id,forKey:NativeKey("request_id"))
    }
}
extension NativeBatch: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(records,forKey:NativeKey("records"))
    }
}
extension NativeBatchWarning: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(tuned_for,forKey:NativeKey("tuned_for"))
        try c.encode(running,forKey:NativeKey("running"))
    }
}
extension NativeContent: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeChoice: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(name,forKey:NativeKey("name"))
        try c.encode(description,forKey:NativeKey("description"))
        try c.encode(weight,forKey:NativeKey("weight"))
    }
}
extension NativeDecideValue: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeDecideValue.Values: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(boolean,forKey:NativeKey("boolean"))
        try c.encode(authored,forKey:NativeKey("authored"))
    }
}
extension NativeEndpoint: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(name,forKey:NativeKey("name"))
        try c.encode(kind,forKey:NativeKey("kind"))
    }
}
extension NativeEdge: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(relation,forKey:NativeKey("relation"))
        try c.encode(source,forKey:NativeKey("source"))
        try c.encode(target,forKey:NativeKey("target"))
        try c.encode(probability,forKey:NativeKey("probability"))
        try c.encode(either,forKey:NativeKey("either"))
    }
}
extension NativeEntity: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(text,forKey:NativeKey("text"))
        try c.encode(start,forKey:NativeKey("start"))
        try c.encode(end,forKey:NativeKey("end"))
        try c.encode(length,forKey:NativeKey("length"))
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(strength,forKey:NativeKey("strength"))
    }
}
extension NativeEntityEdge: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(relation,forKey:NativeKey("relation"))
        try c.encode(source,forKey:NativeKey("source"))
        try c.encode(target,forKey:NativeKey("target"))
        try c.encode(probability,forKey:NativeKey("probability"))
        try c.encode(either,forKey:NativeKey("either"))
    }
}
extension NativeFacts: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(call_id,forKey:NativeKey("call_id"))
        try c.encode(cache_answers,forKey:NativeKey("cache_answers"))
        try c.encode(estimated_cost_usd,forKey:NativeKey("estimated_cost_usd"))
        try c.encode(input_tokens,forKey:NativeKey("input_tokens"))
        try c.encode(model,forKey:NativeKey("model"))
        try c.encode(output_tokens,forKey:NativeKey("output_tokens"))
        try c.encode(records,forKey:NativeKey("records"))
        try c.encode(requests_sent,forKey:NativeKey("requests_sent"))
        try c.encode(seconds,forKey:NativeKey("seconds"))
        try c.encode(command_ms,forKey:NativeKey("command_ms"))
    }
}
extension NativeImageView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(media,forKey:NativeKey("media"))
        try c.encode(bytes,forKey:NativeKey("bytes"))
        try c.encode(width,forKey:NativeKey("width"))
        try c.encode(height,forKey:NativeKey("height"))
        try c.encode(filename,forKey:NativeKey("filename"))
    }
}
extension NativeInputDeclaration: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(properties,forKey:NativeKey("properties"))
        try c.encode(required,forKey:NativeKey("required"))
    }
}
extension NativeInputProperty: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(name,forKey:NativeKey("name"))
        try c.encode(kind,forKey:NativeKey("kind"))
    }
}
extension NativeLocation: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(file,forKey:NativeKey("file"))
        try c.encode(first_line,forKey:NativeKey("first_line"))
        try c.encode(last_line,forKey:NativeKey("last_line"))
    }
}
extension NativeInputView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(original,forKey:NativeKey("original"))
        try c.encode(position,forKey:NativeKey("position"))
        try c.encode(images,forKey:NativeKey("images"))
    }
}
extension NativeMemberFailure: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(failure_id,forKey:NativeKey("failure_id"))
        try c.encode(cause,forKey:NativeKey("cause"))
    }
}
extension NativeMemberValue: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeMemberValue.Values: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(decide,forKey:NativeKey("decide"))
        try c.encode(choose,forKey:NativeKey("choose"))
        try c.encode(tag,forKey:NativeKey("tag"))
        try c.encode(score,forKey:NativeKey("score"))
    }
}
extension NativeName: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(start,forKey:NativeKey("start"))
        try c.encode(end,forKey:NativeKey("end"))
        try c.encode(kinds,forKey:NativeKey("kinds"))
        try c.encode(edges,forKey:NativeKey("edges"))
    }
}
extension NativeNamedAnswer: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(pick,forKey:NativeKey("pick"))
        try c.encode(probabilities,forKey:NativeKey("probabilities"))
        try c.encode(confidence,forKey:NativeKey("confidence"))
    }
}
extension NativeObservationIdentity: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeObservationIdentity.Values: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(observation_id,forKey:NativeKey("observation_id"))
        try c.encode(failure_id,forKey:NativeKey("failure_id"))
    }
}
extension NativeObservedProbabilities: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeObservedProbabilities.Values: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(yes,forKey:NativeKey("yes"))
        try c.encode(named,forKey:NativeKey("named"))
    }
}
extension NativeObservationSuccess: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(answer_id,forKey:NativeKey("answer_id"))
        try c.encode(observation_id,forKey:NativeKey("observation_id"))
        try c.encode(value,forKey:NativeKey("value"))
        try c.encode(probabilities,forKey:NativeKey("probabilities"))
        try c.encode(confidence,forKey:NativeKey("confidence"))
    }
}
extension NativePiece: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(start,forKey:NativeKey("start"))
        try c.encode(end,forKey:NativeKey("end"))
        try c.encode(tags,forKey:NativeKey("tags"))
    }
}
extension NativePlace: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(start,forKey:NativeKey("start"))
        try c.encode(end,forKey:NativeKey("end"))
    }
}
extension NativePair: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(relation,forKey:NativeKey("relation"))
        try c.encode(source,forKey:NativeKey("source"))
        try c.encode(target,forKey:NativeKey("target"))
        try c.encode(probability,forKey:NativeKey("probability"))
    }
}
extension NativeProbability: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(name,forKey:NativeKey("name"))
        try c.encode(probability,forKey:NativeKey("probability"))
    }
}
extension NativeProfileWarning: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(tuned_for,forKey:NativeKey("tuned_for"))
        try c.encode(running,forKey:NativeKey("running"))
    }
}
extension NativeQuestionAuthor: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(name,forKey:NativeKey("name"))
        try c.encode(wording_version,forKey:NativeKey("wording_version"))
        try c.encode(item_schema,forKey:NativeKey("item_schema"))
        try c.encode(context_schema,forKey:NativeKey("context_schema"))
    }
}
extension NativeQuestionMember: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(name,forKey:NativeKey("name"))
        try c.encode(question,forKey:NativeKey("question"))
    }
}
extension NativeQuestionSource: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(origin,forKey:NativeKey("origin"))
        try c.encode(answered_by,forKey:NativeKey("answered_by"))
    }
}
extension NativeRecognizeAnswer: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(pieces,forKey:NativeKey("pieces"))
        try c.encode(names,forKey:NativeKey("names"))
        try c.encode(pairs,forKey:NativeKey("pairs"))
    }
}
extension NativeRecognizeValue: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(entities,forKey:NativeKey("entities"))
        try c.encode(relations,forKey:NativeKey("relations"))
    }
}
extension NativeRelationSuccess: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(answer_id,forKey:NativeKey("answer_id"))
        try c.encode(probability,forKey:NativeKey("probability"))
        try c.encode(accepted,forKey:NativeKey("accepted"))
    }
}
extension NativeRelationAnswer: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(relation,forKey:NativeKey("relation"))
        try c.encode(reads,forKey:NativeKey("reads"))
        try c.encode(method,forKey:NativeKey("method"))
        try c.encode(direction,forKey:NativeKey("direction"))
        try c.encode(source,forKey:NativeKey("source"))
        try c.encode(target,forKey:NativeKey("target"))
        try c.encode(request,forKey:NativeKey("request"))
        try c.encode(state,forKey:NativeKey("state"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeRelationAnswer.Values: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(success,forKey:NativeKey("success"))
        try c.encode(failure,forKey:NativeKey("failure"))
    }
}
extension NativeRelation: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(name,forKey:NativeKey("name"))
        try c.encode(source,forKey:NativeKey("source"))
        try c.encode(target,forKey:NativeKey("target"))
        try c.encode(reads,forKey:NativeKey("reads"))
        try c.encode(either,forKey:NativeKey("either"))
        try c.encode(single,forKey:NativeKey("single"))
    }
}
extension NativeReportedUsage: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(present,forKey:NativeKey("present"))
        try c.encode(input_tokens,forKey:NativeKey("input_tokens"))
        try c.encode(output_tokens,forKey:NativeKey("output_tokens"))
    }
}
extension NativeRule: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(low,forKey:NativeKey("low"))
        try c.encode(high,forKey:NativeKey("high"))
    }
}
extension NativeQuestionView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(text,forKey:NativeKey("text"))
        try c.encode(yes,forKey:NativeKey("yes"))
        try c.encode(no,forKey:NativeKey("no"))
        try c.encode(choices,forKey:NativeKey("choices"))
        try c.encode(threshold,forKey:NativeKey("threshold"))
        try c.encode(relation_threshold,forKey:NativeKey("relation_threshold"))
        try c.encode(model,forKey:NativeKey("model"))
        try c.encode(profile,forKey:NativeKey("profile"))
        try c.encode(batch,forKey:NativeKey("batch"))
        try c.encode(batch_max,forKey:NativeKey("batch_max"))
        try c.encode(none,forKey:NativeKey("none"))
        try c.encode(on,forKey:NativeKey("on"))
        try c.encode(members,forKey:NativeKey("members"))
        try c.encode(kinds,forKey:NativeKey("kinds"))
        try c.encode(relations,forKey:NativeKey("relations"))
        try c.encode(name_pointer,forKey:NativeKey("name_pointer"))
        try c.encode(kind_pointer,forKey:NativeKey("kind_pointer"))
    }
}
extension NativeDetails: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(question,forKey:NativeKey("question"))
        try c.encode(threshold,forKey:NativeKey("threshold"))
        try c.encode(raw_pick,forKey:NativeKey("raw_pick"))
        try c.encode(usage,forKey:NativeKey("usage"))
        try c.encode(question_sources,forKey:NativeKey("question_sources"))
        try c.encode(observations,forKey:NativeKey("observations"))
        try c.encode(inputs,forKey:NativeKey("inputs"))
    }
}
extension NativeScoreAnswer: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(level,forKey:NativeKey("level"))
        try c.encode(probabilities,forKey:NativeKey("probabilities"))
        try c.encode(confidence,forKey:NativeKey("confidence"))
    }
}
extension NativeAnswer: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeAnswer.Values: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(probability,forKey:NativeKey("probability"))
        try c.encode(choice,forKey:NativeKey("choice"))
        try c.encode(tag,forKey:NativeKey("tag"))
        try c.encode(score,forKey:NativeKey("score"))
        try c.encode(find,forKey:NativeKey("find"))
    }
}
extension NativeMemberSuccess: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(answer_id,forKey:NativeKey("answer_id"))
        try c.encode(value,forKey:NativeKey("value"))
        try c.encode(answer,forKey:NativeKey("answer"))
        try c.encode(threshold,forKey:NativeKey("threshold"))
    }
}
extension NativeMember: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(name,forKey:NativeKey("name"))
        try c.encode(request,forKey:NativeKey("request"))
        try c.encode(question,forKey:NativeKey("question"))
        try c.encode(state,forKey:NativeKey("state"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeMember.Values: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(success,forKey:NativeKey("success"))
        try c.encode(failure,forKey:NativeKey("failure"))
    }
}
extension NativeSourceDetail: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(origin,forKey:NativeKey("origin"))
        try c.encode(answered_by,forKey:NativeKey("answered_by"))
        try c.encode(batch_size,forKey:NativeKey("batch_size"))
    }
}
extension NativeSourceEndpoint: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(ordinal,forKey:NativeKey("ordinal"))
        try c.encode(endpoint,forKey:NativeKey("endpoint"))
        try c.encode(record,forKey:NativeKey("record"))
        try c.encode(position,forKey:NativeKey("position"))
    }
}
extension NativeSourceEdge: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(relation,forKey:NativeKey("relation"))
        try c.encode(source,forKey:NativeKey("source"))
        try c.encode(target,forKey:NativeKey("target"))
        try c.encode(probability,forKey:NativeKey("probability"))
        try c.encode(either,forKey:NativeKey("either"))
    }
}
extension NativeSourceEntity: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(entity,forKey:NativeKey("entity"))
        try c.encode(position,forKey:NativeKey("position"))
    }
}
extension NativeSourceEntityEdge: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(relation,forKey:NativeKey("relation"))
        try c.encode(source,forKey:NativeKey("source"))
        try c.encode(target,forKey:NativeKey("target"))
        try c.encode(probability,forKey:NativeKey("probability"))
        try c.encode(either,forKey:NativeKey("either"))
    }
}
extension NativeSourceRecognition: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(present,forKey:NativeKey("present"))
        try c.encode(entities,forKey:NativeKey("entities"))
        try c.encode(relations,forKey:NativeKey("relations"))
    }
}
extension NativeSourceRelations: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(present,forKey:NativeKey("present"))
        try c.encode(edges,forKey:NativeKey("edges"))
    }
}
extension NativeStopped: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(at,forKey:NativeKey("at"))
        try c.encode(cause,forKey:NativeKey("cause"))
        try c.encode(status,forKey:NativeKey("status"))
        try c.encode(retryable,forKey:NativeKey("retryable"))
    }
}
extension NativeError: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(code,forKey:NativeKey("code"))
        try c.encode(message,forKey:NativeKey("message"))
        try c.encode(retryable,forKey:NativeKey("retryable"))
        try c.encode(stopped,forKey:NativeKey("stopped"))
    }
}
extension NativeUsage: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(input_tokens,forKey:NativeKey("input_tokens"))
        try c.encode(output_tokens,forKey:NativeKey("output_tokens"))
    }
}
extension NativeMeta: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(tool,forKey:NativeKey("tool"))
        try c.encode(question_sha256,forKey:NativeKey("question_sha256"))
        try c.encode(questions_sha256,forKey:NativeKey("questions_sha256"))
        try c.encode(url,forKey:NativeKey("url"))
        try c.encode(model,forKey:NativeKey("model"))
        try c.encode(usage,forKey:NativeKey("usage"))
        try c.encode(requests_sent,forKey:NativeKey("requests_sent"))
        try c.encode(cached,forKey:NativeKey("cached"))
        try c.encode(requests,forKey:NativeKey("requests"))
        try c.encode(failed_questions,forKey:NativeKey("failed_questions"))
        try c.encode(profile_warning,forKey:NativeKey("profile_warning"))
        try c.encode(batch_setting,forKey:NativeKey("batch_setting"))
        try c.encode(batch_warning,forKey:NativeKey("batch_warning"))
        try c.encode(context_sha256,forKey:NativeKey("context_sha256"))
        try c.encode(attempts,forKey:NativeKey("attempts"))
        try c.encode(origin,forKey:NativeKey("origin"))
        try c.encode(question_sources,forKey:NativeKey("question_sources"))
        try c.encode(observations,forKey:NativeKey("observations"))
        try c.encode(answered_by,forKey:NativeKey("answered_by"))
    }
}
extension NativeQuestionObservation: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(index,forKey:NativeKey("index"))
        try c.encode(member,forKey:NativeKey("member"))
        try c.encode(stage,forKey:NativeKey("stage"))
        try c.encode(position,forKey:NativeKey("position"))
        try c.encode(question_sha256,forKey:NativeKey("question_sha256"))
        try c.encode(model,forKey:NativeKey("model"))
        try c.encode(url,forKey:NativeKey("url"))
        try c.encode(requests,forKey:NativeKey("requests"))
        try c.encode(requests_sent,forKey:NativeKey("requests_sent"))
        try c.encode(cached,forKey:NativeKey("cached"))
        try c.encode(failed_questions,forKey:NativeKey("failed_questions"))
        try c.encode(usage,forKey:NativeKey("usage"))
        try c.encode(question_sources,forKey:NativeKey("question_sources"))
        try c.encode(state,forKey:NativeKey("state"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeQuestionObservation.Values: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(success,forKey:NativeKey("success"))
        try c.encode(failure,forKey:NativeKey("failure"))
    }
}
extension NativeRow: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(answer_id,forKey:NativeKey("answer_id"))
        try c.encode(input,forKey:NativeKey("input"))
        try c.encode(question,forKey:NativeKey("question"))
        try c.encode(answer,forKey:NativeKey("answer"))
        try c.encode(threshold,forKey:NativeKey("threshold"))
        try c.encode(position,forKey:NativeKey("position"))
        try c.encode(input_file,forKey:NativeKey("input_file"))
        try c.encode(meta,forKey:NativeKey("meta"))
        try c.encode(images,forKey:NativeKey("images"))
    }
}
extension NativeAnnotateView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(common,forKey:NativeKey("common"))
        try c.encode(answers,forKey:NativeKey("answers"))
    }
}
extension NativeChooseView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(common,forKey:NativeKey("common"))
        try c.encode(value,forKey:NativeKey("value"))
    }
}
extension NativeDecideView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(common,forKey:NativeKey("common"))
        try c.encode(value,forKey:NativeKey("value"))
    }
}
extension NativeFilterView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(common,forKey:NativeKey("common"))
        try c.encode(value,forKey:NativeKey("value"))
    }
}
extension NativeFindView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(common,forKey:NativeKey("common"))
        try c.encode(value,forKey:NativeKey("value"))
        try c.encode(index,forKey:NativeKey("index"))
    }
}
extension NativeRankView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(common,forKey:NativeKey("common"))
        try c.encode(value,forKey:NativeKey("value"))
        try c.encode(question_name,forKey:NativeKey("question_name"))
    }
}
extension NativeRecognizeView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(common,forKey:NativeKey("common"))
        try c.encode(value,forKey:NativeKey("value"))
        try c.encode(answer,forKey:NativeKey("answer"))
    }
}
extension NativeRelateView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(common,forKey:NativeKey("common"))
        try c.encode(value,forKey:NativeKey("value"))
        try c.encode(questions,forKey:NativeKey("questions"))
    }
}
extension NativeScoreView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(common,forKey:NativeKey("common"))
        try c.encode(value,forKey:NativeKey("value"))
    }
}
extension NativeSummary: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(state,forKey:NativeKey("state"))
        try c.encode(schema,forKey:NativeKey("schema"))
        try c.encode(answer_id,forKey:NativeKey("answer_id"))
        try c.encode(function,forKey:NativeKey("function"))
        try c.encode(count,forKey:NativeKey("count"))
        try c.encode(observation_count,forKey:NativeKey("observation_count"))
        try c.encode(meta,forKey:NativeKey("meta"))
        try c.encode(facts,forKey:NativeKey("facts"))
        try c.encode(attempts,forKey:NativeKey("attempts"))
        try c.encode(error,forKey:NativeKey("error"))
    }
}
extension NativeTagView: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(common,forKey:NativeKey("common"))
        try c.encode(value,forKey:NativeKey("value"))
    }
}
extension NativeRowObservation: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(index,forKey:NativeKey("index"))
        try c.encode(function,forKey:NativeKey("function"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeRowObservation.Values: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(decide,forKey:NativeKey("decide"))
        try c.encode(choose,forKey:NativeKey("choose"))
        try c.encode(tag,forKey:NativeKey("tag"))
        try c.encode(score,forKey:NativeKey("score"))
        try c.encode(filter,forKey:NativeKey("filter"))
        try c.encode(rank,forKey:NativeKey("rank"))
        try c.encode(find,forKey:NativeKey("find"))
        try c.encode(annotate,forKey:NativeKey("annotate"))
        try c.encode(recognize,forKey:NativeKey("recognize"))
        try c.encode(relate,forKey:NativeKey("relate"))
    }
}
extension NativeObservation: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(kind,forKey:NativeKey("kind"))
        try c.encode(data,forKey:NativeKey("data"))
    }
}
extension NativeObservation.Values: Encodable {
    public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
        try c.encode(question,forKey:NativeKey("question"))
        try c.encode(row,forKey:NativeKey("row"))
    }
}
struct NativeKey: CodingKey { let stringValue: String; var intValue: Int? { nil }; init(_ v: String) { stringValue = v }; init?(stringValue: String) { self.stringValue = stringValue }; init?(intValue: Int) { return nil } }
extension NativeResult: Encodable { public func encode(to encoder: Encoder) throws { var c = encoder.container(keyedBy: NativeKey.self)
try c.encode(summary,forKey:NativeKey("summary"))
try c.encode(rows,forKey:NativeKey("rows"))
try c.encode(observations,forKey:NativeKey("observations"))
try c.encode(details,forKey:NativeKey("details"))
try c.encode(observationDetails,forKey:NativeKey("observation_details"))
try c.encode(authors,forKey:NativeKey("authors"))
try c.encode(observationAuthors,forKey:NativeKey("observation_authors"))
try c.encode(memberAuthors,forKey:NativeKey("member_authors"))
try c.encode(rankMembers,forKey:NativeKey("rank_members"))
try c.encode(locatedRecognition,forKey:NativeKey("located_recognition"))
try c.encode(locatedRelations,forKey:NativeKey("located_relations"))
} }
