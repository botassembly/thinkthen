# frozen_string_literal: true

module ThinkThen
  module Values
    def self.freeze_copy(value)
      case value
      when String then String.new(value).freeze
      when Array then Array.new(value).map { |entry| freeze_copy(entry) }.freeze
      when Hash then value.to_h { |key, entry| [freeze_copy(key), freeze_copy(entry)] }.freeze
      else value
      end
    end
    def self.wire(value)
      case value
      when Data, RowError then value.to_h
      when Symbol then value.to_s
      when Array then value.map { |entry| wire(entry) }
      when Hash then value.to_h { |key, entry| [key.to_s, wire(entry)] }
      else value
      end
    end
  end
  module WireValue
    # Serialize the contract value, including when JSON.generate calls this method.
    def to_json(*_args) = JSON.generate(to_h)
    def to_h
      # @type var klass: untyped
      klass = self.class
      klass.members.to_h { |key| [key.to_s, Values.wire(public_send(key))] }.reject { |_, value| value.nil? }
    end
  end
  # Provider-reported token sum (nil on overflow), with finished unknown attempts.
  TokenUsage = Data.define(:reported, :unknown_attempts)
  # An immutable copied cumulative client snapshot, independent of native ownership.
  Usage = Data.define(:attempts, :responses, :status_retries, :connection_retries,
                      :request_body_bytes, :active_attempts, :active_calls, :input_tokens, :output_tokens)
  # The question identity, backend, model and optional execution name.
  Source = Data.define(:question_identity, :backend, :model, :name)
  class Source
    include WireValue
    def initialize(question_identity:, backend:, model:, name: nil)
      super
    end
  end
  # A record failure can be inspected as a value or raised as an exception.
  class RowError < RecordError
    attr_reader :raw_code, :raw_name
    def initialize(code:, message:, http_status: nil, raw_code: nil, raw_name: nil)
      @raw_code, @raw_name = raw_code, Values.freeze_copy(raw_name)
      super(message, code: code, http_status: http_status)
    end
    def to_h
      {"code" => code.to_s, "message" => message, "http_status" => http_status,
       "raw_code" => raw_code, "raw_name" => raw_name}.reject { |_, value| value.nil? }
    end
    def to_json(*_args) = JSON.generate(to_h)
  end
  # A named source/target relationship and its optional phrase.
  RelationRule = Data.define(:source, :target, :relation, :phrase, :symmetric)
  class RelationRule
    include WireValue
    def initialize(source:, target:, relation:, phrase: nil, symmetric: false)
      super(source: Values.freeze_copy(source), target: Values.freeze_copy(target), relation: Values.freeze_copy(relation),
            phrase: Values.freeze_copy(phrase), symmetric: symmetric)
    end
    private
    def definition
      {"source" => source, "target" => target, "relation" => relation, "reads" => phrase, "either" => symmetric}.reject { |_, value| value.nil? }
    end
  end
  # Retry delays and attempt timeouts are expressed in seconds.
  Retry = Data.define(:max_retries, :initial_backoff, :attempt_timeout)
  class Retry
    include WireValue
    def initialize(max_retries: nil, initial_backoff: nil, attempt_timeout: nil)
      super
    end
    def to_h
      {"max_retries" => max_retries, "initial_backoff_ms" => initial_backoff && (initial_backoff * 1000).round,
       "attempt_timeout_ms" => attempt_timeout && (attempt_timeout * 1000).round}.reject { |_, value| value.nil? }
    end
  end
  # Per-label probabilities for choose and tag answers.
  LabelDetail = Data.define(:labels) { include WireValue }
  # Per-level probabilities for score and rank answers.
  LevelDetail = Data.define(:levels) { include WireValue }
  # Matched text and zero-based character offsets.
  Found = Data.define(:text, :start, :end) { include WireValue }
  # Recognized text, character offsets, kind and probability.
  Name = Data.define(:text, :start, :end, :kind, :probability) { include WireValue }
  # An endpoint name, its character offsets and its kind.
  RelationName = Data.define(:text, :start, :end, :kind) { include WireValue }
  # The source and target names, relation probability and symmetry.
  Relation = Data.define(:source, :target, :relation, :probability, :symmetric)
  class Relation
    include WireValue
    def to_h
      result = super
      result.delete("symmetric")
      result["either"] = true if symmetric
      result
    end
  end
  # The recognized names and evaluated relation pairs.
  RelationDetail = Data.define(:names, :pairs) { include WireValue }
  # A text piece, character offsets and tag probabilities.
  PieceDetail = Data.define(:text, :start, :end, :tags) { include WireValue }
  # Name text, character offsets and kind probabilities.
  KindDetail = Data.define(:text, :start, :end, :kinds) { include WireValue }
  # The text pieces and recognized name details.
  RecognizeDetail = Data.define(:pieces, :names) { include WireValue }
end
