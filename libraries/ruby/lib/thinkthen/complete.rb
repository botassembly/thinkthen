# frozen_string_literal: true
# Private result/2 carriers; no execution adapter or installed complete door.
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
    Position = Struct.new(:file, :first, :last, :images, keyword_init: true) do
      def initialize(**given)
        super(**{file: ABSENT, first: ABSENT, last: ABSENT, images: ABSENT}.merge(given))
      end
      def inspect = "<Position: content withheld>"
    end
    Usage = Struct.new(:input_tokens, :output_tokens, keyword_init: true) do
      def inspect = "<Usage: content withheld>"
    end
    ProfileWarning = Struct.new(:tuned_for, :running, keyword_init: true) do
      def inspect = "<ProfileWarning: content withheld>"
    end
    BatchWarning = Struct.new(:tuned_for, :running, keyword_init: true) do
      def inspect = "<BatchWarning: content withheld>"
    end
    Attempt = Struct.new(:ordinal, :request_sha256, :wall_ms, :outcome, :sdk_request_id, :status, :server_ms, :request_id, keyword_init: true) do
      def initialize(**given)
        super(**{status: ABSENT, server_ms: ABSENT, request_id: ABSENT}.merge(given))
      end
      def inspect = "<Attempt: content withheld>"
    end
    Facts = Struct.new(:call_id, :records, :requests_sent, :cache_answers, :seconds, :input_tokens, :output_tokens, :model, :estimated_cost_usd, :command_ms, keyword_init: true) do
      def initialize(**given)
        super(**{input_tokens: ABSENT, output_tokens: ABSENT, model: ABSENT, estimated_cost_usd: ABSENT, command_ms: ABSENT}.merge(given))
      end
      def inspect = "<Facts: content withheld>"
    end
    QuestionSource = Struct.new(:origin, :answered_by, keyword_init: true) do
      def inspect = "<QuestionSource: content withheld>"
    end
    Observed = Struct.new(:observation_id, keyword_init: true) do
      def inspect = "<Observed: content withheld>"
    end
    FailedObservation = Struct.new(:failure_id, keyword_init: true) do
      def inspect = "<FailedObservation: content withheld>"
    end
    Meta = Struct.new(:tool, :url, :model, :requests_sent, :cached, :requests, :failed_questions, :origin, :question_sources, :observations, :question_sha256, :questions_sha256, :answered_by, :usage, :profile_warning, :batch_setting, :batch_warning, :context_sha256, :attempts, keyword_init: true) do
      def initialize(**given)
        super(**{question_sha256: ABSENT, questions_sha256: ABSENT, answered_by: ABSENT, usage: ABSENT, profile_warning: ABSENT, batch_setting: ABSENT, batch_warning: ABSENT, context_sha256: ABSENT, attempts: ABSENT}.merge(given))
      end
      def inspect = "<Meta: content withheld>"
    end
    YesNo = Struct.new(:kind, :probability, keyword_init: true) do
      def inspect = "<YesNo: content withheld>"
    end
    Choice = Struct.new(:kind, :pick, :probabilities, :confidence, keyword_init: true) do
      def initialize(**given)
        super(**{confidence: ABSENT}.merge(given))
      end
      def inspect = "<Choice: content withheld>"
    end
    Tags = Struct.new(:kind, :probabilities, keyword_init: true) do
      def inspect = "<Tags: content withheld>"
    end
    Score = Struct.new(:kind, :level, :probabilities, :confidence, keyword_init: true) do
      def initialize(**given)
        super(**{confidence: ABSENT}.merge(given))
      end
      def inspect = "<Score: content withheld>"
    end
    FindAnswer = Struct.new(:kind, :pick, :probabilities, :confidence, keyword_init: true) do
      def initialize(**given)
        super(**{confidence: ABSENT}.merge(given))
      end
      def inspect = "<FindAnswer: content withheld>"
    end
    DecideQuestion = Struct.new(:verb, :text, :true, :false, keyword_init: true) do
      def initialize(**given)
        super(**{true: ABSENT, false: ABSENT}.merge(given))
      end
      def inspect = "<DecideQuestion: content withheld>"
    end
    ChooseQuestion = Struct.new(:verb, :text, :options, keyword_init: true) do
      def inspect = "<ChooseQuestion: content withheld>"
    end
    TagQuestion = Struct.new(:verb, :text, :labels, keyword_init: true) do
      def inspect = "<TagQuestion: content withheld>"
    end
    ScoreQuestion = Struct.new(:verb, :text, :levels, keyword_init: true) do
      def inspect = "<ScoreQuestion: content withheld>"
    end
    FindQuestion = Struct.new(:verb, :text, :none, keyword_init: true) do
      def inspect = "<FindQuestion: content withheld>"
    end
    RelationRule = Struct.new(:name, :source, :target, :reads, :either, :single, keyword_init: true) do
      def initialize(**given)
        super(**{single: ABSENT}.merge(given))
      end
      def inspect = "<RelationRule: content withheld>"
    end
    RelateFields = Struct.new(:name, :kind, keyword_init: true) do
      def inspect = "<RelateFields: content withheld>"
    end
    RelateQuestion = Struct.new(:verb, :fields, :relations, :threshold, :profile, keyword_init: true) do
      def initialize(**given)
        super(**{profile: ABSENT}.merge(given))
      end
      def inspect = "<RelateQuestion: content withheld>"
    end
    RecognizeQuestion = Struct.new(:verb, :kinds, :relations, :threshold, :relation_threshold, :on, :profile, keyword_init: true) do
      def initialize(**given)
        super(**{relations: ABSENT, on: ABSENT, profile: ABSENT}.merge(given))
      end
      def inspect = "<RecognizeQuestion: content withheld>"
    end
    Failure = Struct.new(:kind, :cause, keyword_init: true) do
      def inspect = "<Failure: content withheld>"
    end
    FailedField = Struct.new(:failed, keyword_init: true) do
      def inspect = "<FailedField: content withheld>"
    end
    Entity = Struct.new(:text, :start, :end, :length, :kind, :strength, :file, :first_line, :last_line, keyword_init: true) do
      def initialize(**given)
        super(**{file: ABSENT, first_line: ABSENT, last_line: ABSENT}.merge(given))
      end
      def inspect = "<Entity: content withheld>"
    end
    Endpoint = Struct.new(:name, :kind, :record, :file, :first_line, :last_line, keyword_init: true) do
      def initialize(**given)
        super(**{record: ABSENT, file: ABSENT, first_line: ABSENT, last_line: ABSENT}.merge(given))
      end
      def inspect = "<Endpoint: content withheld>"
    end
    Edge = Struct.new(:relation, :source, :target, :probability, :either, keyword_init: true) do
      def initialize(**given)
        super(**{either: ABSENT}.merge(given))
      end
      def inspect = "<Edge: content withheld>"
    end
    EntityEdge = Struct.new(:relation, :source, :target, :probability, :either, keyword_init: true) do
      def initialize(**given)
        super(**{either: ABSENT}.merge(given))
      end
      def inspect = "<EntityEdge: content withheld>"
    end
    Recognition = Struct.new(:entities, :relations, keyword_init: true) do
      def initialize(**given)
        super(**{relations: ABSENT}.merge(given))
      end
      def inspect = "<Recognition: content withheld>"
    end
    PieceOdds = Struct.new(:start, :end, :tags, keyword_init: true) do
      def inspect = "<PieceOdds: content withheld>"
    end
    NameOdds = Struct.new(:start, :end, :kinds, :edges, keyword_init: true) do
      def inspect = "<NameOdds: content withheld>"
    end
    Span = Struct.new(:start, :end, keyword_init: true) do
      def inspect = "<Span: content withheld>"
    end
    PairOdds = Struct.new(:relation, :source, :target, :probability, keyword_init: true) do
      def inspect = "<PairOdds: content withheld>"
    end
    RecognitionAnswer = Struct.new(:pieces, :names, :pairs, keyword_init: true) do
      def inspect = "<RecognitionAnswer: content withheld>"
    end
    AnnotationSuccess = Struct.new(:answer_id, :value, :question, :answer, :threshold, :request, keyword_init: true) do
      def inspect = "<AnnotationSuccess: content withheld>"
    end
    AnnotationFailure = Struct.new(:failure_id, :question, :failure, :request, keyword_init: true) do
      def inspect = "<AnnotationFailure: content withheld>"
    end
    RelationSuccess = Struct.new(:relation, :reads, :method, :direction, :source, :target, :request, :answer_id, :probability, :accepted, keyword_init: true) do
      def inspect = "<RelationSuccess: content withheld>"
    end
    RelationFailure = Struct.new(:relation, :reads, :method, :direction, :source, :target, :request, :failure_id, :failure, keyword_init: true) do
      def inspect = "<RelationFailure: content withheld>"
    end
    RelationAnswer = Struct.new(:questions, keyword_init: true) do
      def inspect = "<RelationAnswer: content withheld>"
    end
    DecideResult = Struct.new(:schema, :answer_id, :meta, :value, :question, :answer, :threshold, :input, :position, :input_file, keyword_init: true) do
      def initialize(**given)
        super(**{input: ABSENT, position: ABSENT, input_file: ABSENT}.merge(given))
      end
      def inspect = "<DecideResult: content withheld>"
    end
    ChooseResult = Struct.new(:schema, :answer_id, :meta, :value, :question, :answer, :threshold, :input, :position, :input_file, keyword_init: true) do
      def initialize(**given)
        super(**{input: ABSENT, position: ABSENT, input_file: ABSENT}.merge(given))
      end
      def inspect = "<ChooseResult: content withheld>"
    end
    TagResult = Struct.new(:schema, :answer_id, :meta, :value, :question, :answer, :threshold, :input, :position, :input_file, keyword_init: true) do
      def initialize(**given)
        super(**{input: ABSENT, position: ABSENT, input_file: ABSENT}.merge(given))
      end
      def inspect = "<TagResult: content withheld>"
    end
    ScoreResult = Struct.new(:schema, :answer_id, :meta, :value, :question, :answer, :threshold, :input, :position, :input_file, keyword_init: true) do
      def initialize(**given)
        super(**{input: ABSENT, position: ABSENT, input_file: ABSENT}.merge(given))
      end
      def inspect = "<ScoreResult: content withheld>"
    end
    FilterResult = Struct.new(:schema, :answer_id, :meta, :value, :input, :question, :answer, :threshold, :position, keyword_init: true) do
      def initialize(**given)
        super(**{position: ABSENT}.merge(given))
      end
      def inspect = "<FilterResult: content withheld>"
    end
    RankResult = Struct.new(:schema, :answer_id, :meta, :value, :input, :question, :answer, :threshold, :question_name, :position, keyword_init: true) do
      def initialize(**given)
        super(**{question_name: ABSENT, position: ABSENT}.merge(given))
      end
      def inspect = "<RankResult: content withheld>"
    end
    FindResult = Struct.new(:schema, :answer_id, :meta, :value, :question, :answer, :threshold, :position, keyword_init: true) do
      def initialize(**given)
        super(**{position: ABSENT}.merge(given))
      end
      def inspect = "<FindResult: content withheld>"
    end
    AnnotateResult = Struct.new(:schema, :answer_id, :meta, :input, :value, :answers, :position, keyword_init: true) do
      def initialize(**given)
        super(**{position: ABSENT}.merge(given))
      end
      def inspect = "<AnnotateResult: content withheld>"
    end
    RecognizeResult = Struct.new(:schema, :answer_id, :meta, :value, :question, :answer, :input, keyword_init: true) do
      def initialize(**given)
        super(**{input: ABSENT}.merge(given))
      end
      def inspect = "<RecognizeResult: content withheld>"
    end
    RelateResult = Struct.new(:schema, :answer_id, :meta, :value, :question, :answer, keyword_init: true) do
      def inspect = "<RelateResult: content withheld>"
    end
    CallError = Struct.new(:kind, :message, :retryable, :facts, :attempts, keyword_init: true) do
      def initialize(**given)
        super(**{facts: ABSENT, attempts: ABSENT}.merge(given))
      end
      def inspect = "<CallError: content withheld>"
    end
    MODELS = JSON.parse(<<~JSON).freeze
      {
        "Position": {"file?": "str", "first?": "positive", "last?": "positive", "images?": "[str]"},
        "Usage": {"input_tokens": "uint", "output_tokens": "uint"},
        "ProfileWarning": {"tuned_for": "str", "running": "str"},
        "BatchWarning": {"tuned_for": "batch", "running": "batch"},
        "Attempt": {"ordinal": "positive", "request_sha256": "Digest", "wall_ms": "uint", "outcome": "outcome", "sdk_request_id": "SdkRequestId", "status?": "uint", "server_ms?": "uint", "request_id?": "str"},
        "Facts": {"call_id": "CallId", "records": "uint", "requests_sent": "uint", "cache_answers": "uint", "seconds": "number", "input_tokens?": "uint", "output_tokens?": "uint", "model?": "str", "estimated_cost_usd?": "cost", "command_ms?": "uint"},
        "QuestionSource": {"origin": "origin", "answered_by": "str"},
        "Observed": {"observation_id": "ObservationId"},
        "FailedObservation": {"failure_id": "FailureId"},
        "Meta": {"tool": "str", "url": "str", "model": "str", "requests_sent": "uint", "cached": "bool", "requests": "[Digest]", "failed_questions": "uint", "origin": "origin|null", "question_sources": "[QuestionSource]", "observations": "[Observation]", "question_sha256?": "Digest", "questions_sha256?": "Digest", "answered_by?": "str", "usage?": "Usage", "profile_warning?": "ProfileWarning", "batch_setting?": "batch", "batch_warning?": "BatchWarning", "context_sha256?": "Digest", "attempts?": "[Attempt]"},
        "YesNo": {"kind": "=yes_no", "probability": "probability"},
        "Choice": {"kind": "=choice", "pick": "str", "probabilities": "{probability}", "confidence?": "probability"},
        "Tags": {"kind": "=tag", "probabilities": "{probability}"},
        "Score": {"kind": "=score", "level": "str", "probabilities": "{probability}", "confidence?": "probability"},
        "FindAnswer": {"kind": "=find", "pick": "str", "probabilities": "{probability}", "confidence?": "probability"},
        "DecideQuestion": {"verb": "=decide", "text": "text", "true?": "description", "false?": "description"},
        "ChooseQuestion": {"verb": "=choose", "text": "text", "options": "[str]"},
        "TagQuestion": {"verb": "=tag", "text": "text", "labels": "[str]"},
        "ScoreQuestion": {"verb": "=score", "text": "text", "levels": "[str]"},
        "FindQuestion": {"verb": "=find", "text": "text", "none": "bool"},
        "RelationRule": {"name": "str", "source": "str", "target": "str", "reads": "str", "either": "bool", "single?": "bool"},
        "RelateFields": {"name": "str", "kind": "str"},
        "RelateQuestion": {"verb": "=relate", "fields": "RelateFields|null", "relations": "[RelationRule]", "threshold": "threshold", "profile?": "str"},
        "RecognizeQuestion": {"verb": "=recognize", "kinds": "{description}", "relations?": "[RelationRule]", "threshold": "threshold", "relation_threshold": "threshold", "on?": "str|[str]", "profile?": "str"},
        "Failure": {"kind": "=backend", "cause": "cause"},
        "FailedField": {"failed": "Failure"},
        "Entity": {"text": "text", "start": "uint", "end": "uint", "length": "uint", "kind": "str", "strength": "number", "file?": "str", "first_line?": "positive", "last_line?": "positive"},
        "Endpoint": {"name": "str", "kind": "str", "record?": "json", "file?": "str", "first_line?": "positive", "last_line?": "positive"},
        "Edge": {"relation": "str", "source": "Endpoint", "target": "Endpoint", "probability": "probability", "either?": "=true"},
        "EntityEdge": {"relation": "str", "source": "Entity", "target": "Entity", "probability": "probability", "either?": "=true"},
        "Recognition": {"entities": "[Entity]", "relations?": "[EntityEdge]"},
        "PieceOdds": {"start": "uint", "end": "uint", "tags": "{probability}"},
        "NameOdds": {"start": "uint", "end": "uint", "kinds": "{probability}|null", "edges": "{probability}|null"},
        "Span": {"start": "uint", "end": "uint"},
        "PairOdds": {"relation": "str", "source": "Span", "target": "Span", "probability": "probability"},
        "RecognitionAnswer": {"pieces": "[PieceOdds]", "names": "[NameOdds]", "pairs": "[PairOdds]"},
        "AnnotationSuccess": {"answer_id": "AnswerId", "value": "SuccessValue", "question": "AtomicQuestion", "answer": "AtomicAnswer", "threshold": "threshold", "request": "Digest"},
        "AnnotationFailure": {"failure_id": "FailureId", "question": "AtomicQuestion", "failure": "Failure", "request": "Digest"},
        "RelationSuccess": {"relation": "str", "reads": "str", "method": "str", "direction": "str", "source": "Endpoint", "target": "Endpoint|null", "request": "Digest", "answer_id": "AnswerId", "probability": "probability", "accepted": "bool"},
        "RelationFailure": {"relation": "str", "reads": "str", "method": "str", "direction": "str", "source": "Endpoint", "target": "Endpoint|null", "request": "Digest", "failure_id": "FailureId", "failure": "Failure"},
        "RelationAnswer": {"questions": "[RelationEntry]"},
        "DecideResult": {"schema": "=thinkthen.result/2", "answer_id": "AnswerId", "meta": "Meta", "value": "bool|null", "question": "DecideQuestion", "answer": "YesNo", "threshold": "threshold", "input?": "json", "position?": "Position", "input_file?": "str"},
        "ChooseResult": {"schema": "=thinkthen.result/2", "answer_id": "AnswerId", "meta": "Meta", "value": "str|null", "question": "ChooseQuestion", "answer": "Choice", "threshold": "threshold", "input?": "json", "position?": "Position", "input_file?": "str"},
        "TagResult": {"schema": "=thinkthen.result/2", "answer_id": "AnswerId", "meta": "Meta", "value": "[str]", "question": "TagQuestion", "answer": "Tags", "threshold": "threshold", "input?": "json", "position?": "Position", "input_file?": "str"},
        "ScoreResult": {"schema": "=thinkthen.result/2", "answer_id": "AnswerId", "meta": "Meta", "value": "number", "question": "ScoreQuestion", "answer": "Score", "threshold": "null", "input?": "json", "position?": "Position", "input_file?": "str"},
        "FilterResult": {"schema": "=thinkthen.result/2", "answer_id": "AnswerId", "meta": "Meta", "value": "bool", "input": "json", "question": "DecideQuestion", "answer": "YesNo", "threshold": "threshold", "position?": "Position"},
        "RankResult": {"schema": "=thinkthen.result/2", "answer_id": "AnswerId", "meta": "Meta", "value": "positive", "input": "json", "question": "AtomicQuestion", "answer": "AtomicAnswer", "threshold": "null", "question_name?": "str", "position?": "Position"},
        "FindResult": {"schema": "=thinkthen.result/2", "answer_id": "AnswerId", "meta": "Meta", "value": "json", "question": "FindQuestion", "answer": "FindAnswer", "threshold": "null", "position?": "Position"},
        "AnnotateResult": {"schema": "=thinkthen.result/2", "answer_id": "AnswerId", "meta": "Meta", "input": "json", "value": "{AnnotatedValue}", "answers": "{AnnotationEntry}", "position?": "Position"},
        "RecognizeResult": {"schema": "=thinkthen.result/2", "answer_id": "AnswerId", "meta": "Meta", "value": "Recognition", "question": "RecognizeQuestion", "answer": "RecognitionAnswer", "input?": "json"},
        "RelateResult": {"schema": "=thinkthen.result/2", "answer_id": "AnswerId", "meta": "Meta", "value": "[Edge]", "question": "RelateQuestion", "answer": "RelationAnswer"},
        "CallError": {"kind": "error_kind", "message": "str", "retryable": "bool", "facts?": "Facts", "attempts?": "[Attempt]"},
        "DecideSpec": {"decide": "text", "true?": "description", "false?": "description", "threshold?": "threshold", "model?": "str", "profile?": "str", "batch?": "batch", "on?": "str|[str]"},
        "ChooseSpec": {"choose": "text", "options": "Labels", "threshold?": "probability", "model?": "str", "profile?": "str", "batch?": "batch", "on?": "str|[str]"},
        "TagSpec": {"tag": "text", "labels": "Labels", "threshold?": "probability", "model?": "str", "profile?": "str", "batch?": "batch", "on?": "str|[str]"},
        "ScoreSpec": {"score": "text", "levels": "Labels", "model?": "str", "profile?": "str", "batch?": "batch", "on?": "str|[str]"},
        "FindSpec": {"find": "text", "none?": "bool", "model?": "str"},
        "QuestionFile": {"path": "str"},
        "QuestionSet": {"version": "one", "questions": "{AnnotationSpec}", "batch?": "batch", "threshold?": "threshold", "profile?": "str"},
        "RecognitionPlan": {"kinds?": "Labels", "relations?": "[PlanRule]"},
        "PlanRule": {"name": "str", "source": "str", "target": "str", "reads?": "str", "either?": "bool", "single?": "bool"},
        "RecognitionSpec": {"version": "one", "recognize": "RecognitionPlan", "threshold?": "probability", "relation_threshold?": "probability", "model?": "str", "profile?": "str", "on?": "str|[str]"},
        "RelationPlan": {"relations": "[PlanRule]", "fields?": "RelateFields"},
        "RelationSpec": {"version": "one", "relate": "RelationPlan", "threshold?": "probability", "model?": "str", "profile?": "str"},
        "Files": {"paths": "[str]", "unit": "unit", "window?": "positive", "media?": "media"},
        "TextInput": {"text": "json"},
        "RecordInput": {"records": "[json]", "context?": "json"},
        "CandidateInput": {"units": "[json]", "context?": "json"},
        "ImageBytes": {"data": "bytes", "name?": "str"},
        "ImageInput": {"images": "[ImageBytes]", "text?": "json"},
        "Controls": {"batch?": "batch", "context?": "json", "on?": "str|[str]", "threshold?": "threshold", "top?": "positive", "none?": "bool", "model?": "str", "attempts?": "bool", "deadline_ms?": "uint"},
        "DecideMember": {"decide": "text", "true?": "description", "false?": "description", "threshold?": "threshold", "on?": "str|[str]"},
        "ChooseMember": {"choose": "text", "options": "Labels", "threshold?": "probability", "on?": "str|[str]"},
        "TagMember": {"tag": "text", "labels": "Labels", "threshold?": "probability", "on?": "str|[str]"},
        "ScoreMember": {"score": "text", "levels": "Labels", "on?": "str|[str]"}
      }
    JSON
    ALIASES = JSON.parse(<<~JSON).freeze
      {
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
  private_constant :Complete
end

module ThinkThen
  module Complete
    DecideSpec = Struct.new(:decide, :true, :false, :threshold, :model, :profile, :batch, :on, keyword_init: true) do
      def initialize(**given)
        super(**{true: ABSENT, false: ABSENT, threshold: ABSENT, model: ABSENT, profile: ABSENT, batch: ABSENT, on: ABSENT}.merge(given))
      end
      def inspect = "<DecideSpec: content withheld>"
    end
    ChooseSpec = Struct.new(:choose, :options, :threshold, :model, :profile, :batch, :on, keyword_init: true) do
      def initialize(**given)
        super(**{threshold: ABSENT, model: ABSENT, profile: ABSENT, batch: ABSENT, on: ABSENT}.merge(given))
      end
      def inspect = "<ChooseSpec: content withheld>"
    end
    TagSpec = Struct.new(:tag, :labels, :threshold, :model, :profile, :batch, :on, keyword_init: true) do
      def initialize(**given)
        super(**{threshold: ABSENT, model: ABSENT, profile: ABSENT, batch: ABSENT, on: ABSENT}.merge(given))
      end
      def inspect = "<TagSpec: content withheld>"
    end
    ScoreSpec = Struct.new(:score, :levels, :model, :profile, :batch, :on, keyword_init: true) do
      def initialize(**given)
        super(**{model: ABSENT, profile: ABSENT, batch: ABSENT, on: ABSENT}.merge(given))
      end
      def inspect = "<ScoreSpec: content withheld>"
    end
    FindSpec = Struct.new(:find, :none, :model, keyword_init: true) do
      def initialize(**given)
        super(**{none: ABSENT, model: ABSENT}.merge(given))
      end
      def inspect = "<FindSpec: content withheld>"
    end
    QuestionFile = Struct.new(:path, keyword_init: true) do
      def inspect = "<QuestionFile: content withheld>"
    end
    QuestionSet = Struct.new(:version, :questions, :batch, :threshold, :profile, keyword_init: true) do
      def initialize(**given)
        super(**{batch: ABSENT, threshold: ABSENT, profile: ABSENT}.merge(given))
      end
      def inspect = "<QuestionSet: content withheld>"
    end
    RecognitionPlan = Struct.new(:kinds, :relations, keyword_init: true) do
      def initialize(**given)
        super(**{kinds: ABSENT, relations: ABSENT}.merge(given))
      end
      def inspect = "<RecognitionPlan: content withheld>"
    end
    PlanRule = Struct.new(:name, :source, :target, :reads, :either, :single, keyword_init: true) do
      def initialize(**given)
        super(**{reads: ABSENT, either: ABSENT, single: ABSENT}.merge(given))
      end
      def inspect = "<PlanRule: content withheld>"
    end
    RecognitionSpec = Struct.new(:version, :recognize, :threshold, :relation_threshold, :model, :profile, :on, keyword_init: true) do
      def initialize(**given)
        super(**{threshold: ABSENT, relation_threshold: ABSENT, model: ABSENT, profile: ABSENT, on: ABSENT}.merge(given))
      end
      def inspect = "<RecognitionSpec: content withheld>"
    end
    RelationPlan = Struct.new(:relations, :fields, keyword_init: true) do
      def initialize(**given)
        super(**{fields: ABSENT}.merge(given))
      end
      def inspect = "<RelationPlan: content withheld>"
    end
    RelationSpec = Struct.new(:version, :relate, :threshold, :model, :profile, keyword_init: true) do
      def initialize(**given)
        super(**{threshold: ABSENT, model: ABSENT, profile: ABSENT}.merge(given))
      end
      def inspect = "<RelationSpec: content withheld>"
    end
    Files = Struct.new(:paths, :unit, :window, :media, keyword_init: true) do
      def initialize(**given)
        super(**{window: ABSENT, media: ABSENT}.merge(given))
      end
      def inspect = "<Files: content withheld>"
    end
    TextInput = Struct.new(:text, keyword_init: true) do
      def inspect = "<TextInput: content withheld>"
    end
    RecordInput = Struct.new(:records, :context, keyword_init: true) do
      def initialize(**given)
        super(**{context: ABSENT}.merge(given))
      end
      def inspect = "<RecordInput: content withheld>"
    end
    CandidateInput = Struct.new(:units, :context, keyword_init: true) do
      def initialize(**given)
        super(**{context: ABSENT}.merge(given))
      end
      def inspect = "<CandidateInput: content withheld>"
    end
    ImageBytes = Struct.new(:data, :name, keyword_init: true) do
      def initialize(**given)
        super(**{name: ABSENT}.merge(given))
      end
      def inspect = "<ImageBytes: content withheld>"
    end
    ImageInput = Struct.new(:images, :text, keyword_init: true) do
      def initialize(**given)
        super(**{text: ABSENT}.merge(given))
      end
      def inspect = "<ImageInput: content withheld>"
    end
    Controls = Struct.new(:batch, :context, :on, :threshold, :top, :none, :model, :attempts, :deadline_ms, keyword_init: true) do
      def initialize(**given)
        super(**{batch: ABSENT, context: ABSENT, on: ABSENT, threshold: ABSENT, top: ABSENT, none: ABSENT, model: ABSENT, attempts: ABSENT, deadline_ms: ABSENT}.merge(given))
      end
      def inspect = "<Controls: content withheld>"
    end
  end
end

module ThinkThen
  module Complete
    DecideMember = Struct.new(:decide, :true, :false, :threshold, :on, keyword_init: true) do
      def initialize(**given)
        super(**{true: ABSENT, false: ABSENT, threshold: ABSENT, on: ABSENT}.merge(given))
      end
      def inspect = "<DecideMember: content withheld>"
    end
    ChooseMember = Struct.new(:choose, :options, :threshold, :on, keyword_init: true) do
      def initialize(**given)
        super(**{threshold: ABSENT, on: ABSENT}.merge(given))
      end
      def inspect = "<ChooseMember: content withheld>"
    end
    TagMember = Struct.new(:tag, :labels, :threshold, :on, keyword_init: true) do
      def initialize(**given)
        super(**{threshold: ABSENT, on: ABSENT}.merge(given))
      end
      def inspect = "<TagMember: content withheld>"
    end
    ScoreMember = Struct.new(:score, :levels, :on, keyword_init: true) do
      def initialize(**given)
        super(**{on: ABSENT}.merge(given))
      end
      def inspect = "<ScoreMember: content withheld>"
    end
  end
end
