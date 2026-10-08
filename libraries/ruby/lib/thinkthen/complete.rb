# frozen_string_literal: true
# Validated result/2 carriers used by the public native complete calls.
require "json"
module ThinkThen
  module Complete
    ABSENT = Object.new.freeze
    class Carrier
      def inspect = "<#{self.class.name}: content withheld>"
    end
    class Identity < String
      def initialize(value)
        raise ArgumentError, "invalid identity" unless value.is_a?(String) && /\A[0-9a-f]{64}\z/.match?(value)
        super(value)
        freeze
      end
      def inspect = "<#{self.class.name}>"
    end
    class CallId < Identity; end
    class SdkRequestId < Identity; end
    class ObservationId < Identity; end
    class FailureId < Identity; end
    class AnswerId < Identity; end
    class Digest < Identity; end
    MODELS = JSON.parse(<<~JSON).freeze
{
  "Position": {"file?":"str","first?":"positive","last?":"positive","images?":"[str]"},
  "Usage": {"input_tokens?":"uint","output_tokens?":"uint"},
  "ProfileWarning": {"tuned_for":"str","running":"str"},
  "BatchWarning": {"tuned_for":"batch","running":"batch"},
  "Attempt": {"ordinal":"positive","request_sha256":"Digest","wall_ms":"uint","outcome":"outcome","sdk_request_id":"SdkRequestId","status?":"uint","server_ms?":"uint","request_id?":"str"},
  "Facts": {"call_id":"CallId","records":"uint","requests_sent":"uint","cache_answers":"uint","seconds":"number","input_tokens?":"uint","output_tokens?":"uint","model?":"str","estimated_cost_usd?":"cost","command_ms?":"uint","attempts?":"[Attempt]","held_model_mismatch?":"bool"},
  "QuestionSource": {"origin":"origin","answered_by":"str","batch_size?":"positive"},
  "Observed": {"observation_id":"ObservationId"},
  "FailedObservation": {"failure_id":"FailureId"},
  "Meta": {"tool":"str","url":"str","model":"str","requests_sent":"uint","cached":"bool","requests":"[Digest]","failed_questions":"uint","origin":"origin|null","question_sources":"[QuestionSource]","observations":"[Observation]","question_sha256?":"Digest","questions_sha256?":"Digest","answered_by?":"str","usage?":"Usage","profile_warning?":"ProfileWarning","batch_setting?":"batch","batch_warning?":"BatchWarning","context_sha256?":"Digest","attempts?":"[Attempt]"},
  "YesNo": {"kind":"=yes_no","probability":"probability"},
  "Choice": {"kind":"=choice","pick":"str","probabilities":"{probability}","confidence?":"probability"},
  "Tags": {"kind":"=tag","probabilities":"{probability}"},
  "Score": {"kind":"=score","level":"str","probabilities":"{probability}","confidence?":"probability"},
  "FindAnswer": {"kind":"=find","pick":"str","probabilities":"{probability}","confidence?":"probability"},
  "DecideQuestion": {"verb":"=decide","text":"text","true?":"description","false?":"description","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ChooseQuestion": {"verb":"=choose","text":"text","options":"[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "TagQuestion": {"verb":"=tag","text":"text","labels":"[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ScoreQuestion": {"verb":"=score","text":"text","levels":"[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "FindQuestion": {"verb":"=find","text":"text","none":"bool","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "RelationRule": {"name":"str","source":"str","target":"str","reads":"str","either":"bool","single?":"bool"},
  "RelateFields": {"name":"str","kind":"str"},
  "RelateQuestion": {"verb":"=relate","fields":"RelateFields|null","relations":"[RelationRule]","threshold":"threshold","profile?":"str","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "RecognizeQuestion": {"verb":"=recognize","kinds":"{description}","instructions?":"str","entity_definition?":"str","relations?":"[RelationRule]","threshold":"threshold","relation_threshold":"threshold","on?":"str|[str]","profile?":"str","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "Failure": {"kind":"=backend","cause":"cause"},
  "FailedField": {"failed":"Failure"},
  "Entity": {"text":"text","start":"uint","end":"uint","length":"uint","kind":"str","strength":"number","file?":"str","first_line?":"positive","last_line?":"positive"},
  "Endpoint": {"name":"str","kind":"str","record?":"json","file?":"str","first_line?":"positive","last_line?":"positive"},
  "Edge": {"relation":"str","source":"Endpoint","target":"Endpoint","probability":"probability","either?":"=true"},
  "EntityEdge": {"relation":"str","source":"Entity","target":"Entity","probability":"probability","either?":"=true"},
  "Recognition": {"entities":"[Entity]","relations?":"[EntityEdge]"},
  "PieceOdds": {"start":"uint","end":"uint","tags":"{probability}"},
  "NameOdds": {"start":"uint","end":"uint","kinds":"{probability}|null","edges":"{probability}|null"},
  "Span": {"start":"uint","end":"uint"},
  "PairOdds": {"relation":"str","source":"Span","target":"Span","probability":"probability"},
  "RecognitionAnswer": {"pieces":"[PieceOdds]","names":"[NameOdds]","pairs":"[PairOdds]"},
  "AnnotationSuccess": {"answer_id":"AnswerId","value":"SuccessValue","question":"AtomicQuestion","answer":"AtomicAnswer","threshold":"threshold","request":"Digest"},
  "AnnotationFailure": {"failure_id":"FailureId","question":"AtomicQuestion","failure":"Failure","request":"Digest"},
  "RelationSuccess": {"relation":"str","reads":"str","method":"str","direction":"str","source":"Endpoint","target":"Endpoint|null","request":"Digest","answer_id":"AnswerId","probability":"probability","accepted":"bool","answer?":"AtomicAnswer"},
  "RelationFailure": {"relation":"str","reads":"str","method":"str","direction":"str","source":"Endpoint","target":"Endpoint|null","request":"Digest","failure_id":"FailureId","failure":"Failure"},
  "RelationAnswer": {"questions":"[RelationEntry]"},
  "DecideResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"bool|description","question":"DecideQuestion","answer":"YesNo","threshold":"threshold","input?":"json","position?":"Position","input_file?":"str","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","images?":"[NativeImage]","index?":"uint"},
  "ChooseResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"str|null","question":"ChooseQuestion","answer":"Choice","threshold":"threshold","input?":"json","position?":"Position","input_file?":"str","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","images?":"[NativeImage]","index?":"uint"},
  "TagResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"[str]","question":"TagQuestion","answer":"Tags","threshold":"threshold","input?":"json","position?":"Position","input_file?":"str","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","index?":"uint"},
  "ScoreResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"number","question":"ScoreQuestion","answer":"Score","threshold":"null","input?":"json","position?":"Position","input_file?":"str","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","images?":"[NativeImage]","index?":"uint"},
  "FilterResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"bool","input":"json","question":"DecideQuestion","answer":"YesNo","threshold":"threshold","position?":"Position","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","index?":"uint"},
  "RankMemberResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","value":"positive","question":"DecideQuestion","answer":"YesNo","threshold":"null","meta":"Meta","source?":"PhysicalSource"},
  "RankMember": {"name":"str","result":"RankMemberResult"},
  "RankResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"positive","input":"json","question":"AtomicQuestion","answer":"AtomicAnswer","threshold":"null","question_name?":"str","position?":"Position","file?":"str","first_line?":"positive","last_line?":"positive","source?":"PhysicalSource","index?":"uint","members?":"[RankMember]"},
  "FindResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"json","question":"FindQuestion","answer":"FindAnswer","threshold":"null","position?":"Position","file?":"str","first_line?":"positive","last_line?":"positive","index?":"uint|null","candidates?":"[FindCandidate]"},
  "AnnotateResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","input":"json","value":"{AnnotatedValue}","answers":"{AnnotationEntry}","position?":"Position","file?":"str","first_line?":"positive","last_line?":"positive","index?":"uint","source?":"PhysicalSource"},
  "RecognizeResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"Recognition","question":"RecognizeQuestion","answer":"RecognitionAnswer","input?":"json","file?":"str","first_line?":"positive","last_line?":"positive","index?":"uint","source?":"PhysicalSource"},
  "RelateResult": {"schema":"=thinkthen.result/2","answer_id":"AnswerId","meta":"Meta","value":"[Edge]","question":"RelateQuestion","answer":"RelationAnswer","file?":"str","first_line?":"positive","last_line?":"positive","input?":"json","index?":"uint"},
  "CallError": {"kind":"error_kind","message":"str","retryable":"bool","facts?":"Facts","attempts?":"[Attempt]","stopped?":"Stopped"},
  "DecideSpec": {"decide":"text","true?":"description","false?":"description","threshold?":"threshold","model?":"str","profile?":"str","batch?":"batch","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ChooseSpec": {"choose":"text","options":"Labels","threshold?":"probability","model?":"str","profile?":"str","batch?":"batch","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "TagSpec": {"tag":"text","labels":"Labels","threshold?":"probability","model?":"str","profile?":"str","batch?":"batch","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ScoreSpec": {"score":"text","levels":"Labels","model?":"str","profile?":"str","batch?":"batch","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "FindSpec": {"find":"text","none?":"bool","model?":"str","profile?":"str","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "QuestionFile": {"path":"str"},
  "QuestionSet": {"version":"one","questions":"{AnnotationSpec}","batch?":"batch","threshold?":"threshold","profile?":"str"},
  "RecognitionPlan": {"kinds?":"Labels","instructions?":"str","entity_definition?":"str","relations?":"[PlanRule]"},
  "PlanRule": {"name":"str","source":"str","target":"str","reads?":"str","either?":"bool","single?":"bool"},
  "RecognitionSpec": {"version":"one","recognize":"RecognitionPlan","threshold?":"probability","relation_threshold?":"probability","model?":"str","profile?":"str","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "RelationPlan": {"relations":"[PlanRule]","fields?":"RelateFields"},
  "RelationSpec": {"version":"one","relate":"RelationPlan","threshold?":"probability","model?":"str","profile?":"str","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "Files": {"paths":"[str]","unit":"unit","window?":"positive","media?":"media"},
  "TextInput": {"text":"json"},
  "RecordInput": {"records":"[json]","context?":"json"},
  "CandidateInput": {"units":"[json]","context?":"json"},
  "ImageBytes": {"data":"bytes","name?":"str"},
  "ImageInput": {"images":"[ImageBytes]","text?":"json"},
  "Controls": {"batch?":"batch","context?":"json","on?":"str|[str]","threshold?":"threshold","top?":"positive","none?":"bool","model?":"str","attempts?":"bool","deadline_ms?":"uint"},
  "DecideMember": {"decide":"text","true?":"description","false?":"description","threshold?":"threshold","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ChooseMember": {"choose":"text","options":"Labels","threshold?":"probability","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "TagMember": {"tag":"text","labels":"Labels","threshold?":"probability","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "ScoreMember": {"score":"text","levels":"Labels","on?":"str|[str]","name?":"str","wording_version?":"positive","item_schema?":"InputDeclaration","context_schema?":"InputDeclaration"},
  "StringDeclaration": {"type":"=string"},
  "NumberDeclaration": {"type":"=number"},
  "BooleanDeclaration": {"type":"=boolean"},
  "ArrayDeclaration": {"type":"=array","items":"StringDeclaration"},
  "ObjectDeclaration": {"type":"=object","properties":"{PropertyDeclaration}","required?":"[str]"},
  "FindCandidate": {"index":"uint|null","input":"json","probability":"probability","source?":"PhysicalSource"},
  "PhysicalSource": {"file":"str","first_line?":"positive","last_line?":"positive"},
  "NativeImage": {"media":"image_media","base64":"str","width":"positive","height":"positive"},
  "Stopped": {"cause":"stop_cause","retryable":"bool","status?":"uint","at?":"uint"},
  "NativeInput": {"original":"json","location?":"PhysicalSource","images":"[NativeImage]"}
}
    JSON
    ALIASES = JSON.parse(<<~JSON).freeze
      {
        "InputDeclaration": ["StringDeclaration","ObjectDeclaration"], "PropertyDeclaration": ["StringDeclaration","NumberDeclaration","BooleanDeclaration","ArrayDeclaration"],
        "AnnotationSpec": ["DecideMember", "ChooseMember", "TagMember", "ScoreMember"],
        "SuccessValue": ["bool", "null", "str", "number", "[str]"],
        "Observation": ["Observed", "FailedObservation"],
        "AtomicAnswer": ["YesNo", "Choice", "Tags", "Score", "FindAnswer"],
        "AtomicQuestion": ["DecideQuestion", "ChooseQuestion", "TagQuestion", "ScoreQuestion", "FindQuestion"],
        "AnnotationEntry": ["AnnotationSuccess", "AnnotationFailure"],
        "RelationEntry": ["RelationSuccess", "RelationFailure"],
        "Result": ["DecideResult", "ChooseResult", "TagResult", "ScoreResult", "FilterResult", "RankResult", "FindResult", "AnnotateResult", "RecognizeResult", "RelateResult"],
        "AnnotatedValue": ["bool", "null", "str", "number", "[str]", "FailedField"],
        "Labels": ["[str]", "{description}"],
        "QuestionSpec": ["DecideSpec", "ChooseSpec", "TagSpec", "ScoreSpec"],
        "RankSpec": ["DecideSpec", "ScoreSpec", "QuestionSet", "QuestionFile"],
        "Selection": ["TextInput", "RecordInput", "CandidateInput", "ImageInput", "Files"]
      }
    JSON
    ENUMS = JSON.parse(<<~JSON).freeze
      {
        "image_media": ["image/png","image/jpeg"], "stop_cause": ["usage","local","no_key","transport","status","too_large","reply","backend","cancelled","deadline","defect"],
        "origin": ["live", "cache", "replay", "proxy", "memory"],
        "outcome": ["ok", "status", "transport"],
        "cause": ["missing_answer", "wrong_kind", "missing_probability", "invalid_probability", "invalid_distribution", "unexpected_probability"],
        "error_kind": ["usage", "backend", "local", "cancelled", "deadline", "defect"],
        "unit": ["line", "window", "file"],
        "media": ["text", "image"]
      }
    JSON
    IDS = ['CallId', 'SdkRequestId', 'ObservationId', 'FailureId', 'AnswerId', 'Digest'].freeze
    def self.invalid
      raise ArgumentError, "invalid complete result"
    end

    def self.json(value)
      case value
      when NilClass, TrueClass, FalseClass, Integer then value
      when Float then value.finite? ? value : invalid
      when String then value.dup.freeze
      when Array then value.map { |v| json(v) }.freeze
      when Hash
        invalid unless value.keys.all? { |k| k.is_a?(String) }
        value.to_h { |k, v| [k.dup.freeze, json(v)] }.freeze
      else invalid
      end
    end

    def self.decode(kind, value)
      if ALIASES.key?(kind) || kind.include?("|")
        (ALIASES[kind] || kind.split("|")).each do |variant|
          begin
            return decode(variant, value)
          rescue ArgumentError
            next
          end
        end
        return invalid
      end
      if kind.start_with?("[")
        invalid unless value.is_a?(Array)
        return value.map { |v| decode(kind[1...-1], v) }.freeze
      end
      if kind.start_with?("{")
        invalid unless value.is_a?(Hash) && value.keys.all? { |k| k.is_a?(String) }
        return value.to_h { |k, v| [k.dup.freeze, decode(kind[1...-1], v)] }.freeze
      end
      if MODELS.key?(kind)
        shape = MODELS[kind]
        invalid unless value.is_a?(Hash) && (value.keys - shape.keys.map { |k| k.delete_suffix("?") }).empty?
        held = {}
        shape.each do |key, type|
          name = key.delete_suffix("?")
          unless value.key?(name)
            invalid unless key.end_with?("?")
            held[name.to_sym] = ABSENT
            next
          end
          held[name.to_sym] = decode(type, value[name])
        end
        check(kind, value)
        return const_get(kind).new(**held).freeze
      end
      return value == 1 && value.is_a?(Integer) ? value : invalid if kind == "one"
      return value.is_a?(String) ? value.b.dup.freeze : invalid if kind == "bytes"
      return ENUMS[kind].include?(value) ? value.dup.freeze : invalid if ENUMS.key?(kind)
      return const_get(kind).new(value) if IDS.include?(kind)
      if kind.start_with?("=")
        literal = kind == "=true" ? true : kind[1..]
        return value == literal ? json(value) : invalid
      end
      if ["json", "description", "text"].include?(kind)
        invalid if ["description", "text"].include?(kind) && !value.nil? && ![String, Array, Hash].any? { |t| value.is_a?(t) }
        invalid if kind == "text" && value.nil?
        return json(value)
      end
      return value.nil? ? nil : invalid if kind == "null"
      return [true, false].include?(value) ? value : invalid if kind == "bool"
      return value.is_a?(String) ? value.dup.freeze : invalid if kind == "str"
      if ["uint", "positive", "batch"].include?(kind)
        return "max".freeze if kind == "batch" && value == "max"
        return value.is_a?(Integer) && value >= (kind == "uint" ? 0 : 1) ? value : invalid
      end
      if ["number", "probability", "threshold"].include?(kind)
        if value.is_a?(Numeric) && value.finite? && (kind == "number" || (value <= 1 && (kind == "probability" ? value >= 0 : value > 0)))
          return value
        end
        if kind == "threshold"
          return nil if value.nil?
          if value.is_a?(String) && /\A(?:0(?:\.[0-9]+)?|1(?:\.0+)?):(?:0(?:\.[0-9]+)?|1(?:\.0+)?)\z/.match?(value)
            low, high = value.split(":").map(&:to_f)
            return value.dup.freeze if low < high
          end
        end
        return invalid
      end
      return value.dup.freeze if kind == "cost" && value.is_a?(String) && /\A[0-9]+\.[0-9]{6}\z/.match?(value)
      invalid
    end

    def self.check(kind, v)
      if %w[ChooseSpec TagSpec ChooseMember TagMember RecognitionSpec RelationSpec TagResult FilterResult RecognizeQuestion RelateQuestion].include?(kind)
        %w[threshold relation_threshold].each do |key|
          invalid if v.key?(key) && (!v[key].is_a?(Numeric) || v[key] <= 0 || v[key] > 1)
        end
      end
      invalid if kind == "ChooseResult" && !v["threshold"].nil? && !v["threshold"].is_a?(Numeric)
      if kind == "Meta"
        invalid unless v["requests"].size == v["question_sources"].size && v["requests"].size == v["observations"].size
        invalid if v.key?("question_sha256") == v.key?("questions_sha256")
        sources = v["question_sources"]
        invalid unless sources.all? { |s| %w[live cache replay].include?(s["origin"]) }
        if sources.empty?
          invalid unless v["origin"].nil? && v["cached"] == false && v["requests_sent"] == 0 && !v.key?("answered_by")
        else
          origins = sources.map { |s| s["origin"] }
          origin = origins.include?("live") ? "live" : origins.include?("replay") ? "replay" : "cache"
          invalid unless v["origin"] == origin && v["cached"] == (origin != "live")
          names = sources.map { |s| s["answered_by"] }.uniq
          invalid if names.size == 1 ? v["answered_by"] != names.first : v.key?("answered_by")
        end
        invalid unless v["failed_questions"] == v["observations"].count { |o| o.key?("failure_id") }
      end
      if %w[Span NameOdds PieceOdds Entity].include?(kind)
        invalid if v["end"] < v["start"] || (kind == "Entity" && v["length"] != v["end"] - v["start"])
      end
      if kind == "Position"
        invalid if v.key?("first") != v.key?("last") || (v.key?("first") && (!v.key?("file") || v["last"] < v["first"]))
      end
      if %w[Entity Endpoint].include?(kind)
        invalid if v.key?("first_line") != v.key?("last_line") || (v.key?("first_line") && (!v.key?("file") || v["last_line"] < v["first_line"]))
      end
      invalid if kind == "Usage" && v.empty?
      invalid if kind == "RankResult" && v.key?("members") && (v["members"].empty? || !v.key?("question_name") || v["question"]["verb"] != "decide" || v["answer"]["kind"] != "yes_no")
      invalid if kind == "RankResult" && !%w[yes_no score].include?(v["answer"]["kind"])
      invalid if %w[DecideResult FilterResult].include?(kind) && v["threshold"].nil?
      invalid if kind.end_with?("Result") && (kind == "AnnotateResult") != v["meta"].key?("questions_sha256")
    end

    def self.to_json_value(value)
      case value
      when Struct
        value.to_h.reject { |_, v| v.equal?(ABSENT) }.to_h { |k, v| [k.to_s, to_json_value(v)] }
      when Hash then value.to_h { |k, v| [k, to_json_value(v)] }
      when Array then value.map { |v| to_json_value(v) }
      else value
      end
    end
  end
end

module ThinkThen
  module Complete
    MODELS.each do |name, shape|
      members = shape.keys.map { |key| key.delete_suffix("?").to_sym }
      defaults = shape.keys.select { |key| key.end_with?("?") }.to_h { |key| [key.delete_suffix("?").to_sym, ABSENT] }
      const_set(name, Struct.new(*members, keyword_init: true) do
        define_method(:initialize) { |**given| super(**defaults.merge(given)) }
        define_method(:inspect) { "<#{name}: content withheld>" }
      end) unless const_defined?(name, false)
    end
  end
end
