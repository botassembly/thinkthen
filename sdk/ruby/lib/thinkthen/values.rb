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
      when Data then value.to_h
      when Symbol then value.to_s
      when Array then value.map { |entry| wire(entry) }
      when Hash then value.to_h { |key, entry| [key.to_s, wire(entry)] }
      else value
      end
    end
  end
  module WireValue
    def to_h
      # @type var klass: untyped
      klass = self.class
      klass.members.to_h { |key| [key.to_s, Values.wire(public_send(key))] }.reject { |_, value| value.nil? }
    end
  end
  Source = Data.define(:question_identity, :backend, :model, :name)
  class Source
    include WireValue
    def initialize(question_identity:, backend:, model:, name: nil)
      super
    end
  end
  RowError = Data.define(:code, :message, :http_status, :raw_code, :raw_name)
  class RowError
    include WireValue
    def initialize(code:, message:, http_status: nil, raw_code: nil, raw_name: nil)
      super(code: code.to_sym, message: Values.freeze_copy(message), http_status: http_status, raw_code: raw_code, raw_name: Values.freeze_copy(raw_name))
    end
  end
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
  Retry = Data.define(:max_retries, :initial_backoff_ms, :attempt_timeout_ms)
  class Retry
    include WireValue
    def initialize(max_retries: nil, initial_backoff_ms: nil, attempt_timeout_ms: nil)
      super
    end
  end
  LabelDetail = Data.define(:labels) { include WireValue }
  LevelDetail = Data.define(:levels) { include WireValue }
  Found = Data.define(:text, :start, :end) { include WireValue }
  Name = Data.define(:text, :start, :end, :kind, :probability) { include WireValue }
  RelationName = Data.define(:text, :start, :end, :kind) { include WireValue }
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
  RelationDetail = Data.define(:names, :pairs) { include WireValue }
  PieceDetail = Data.define(:text, :start, :end, :tags) { include WireValue }
  KindDetail = Data.define(:start, :end, :kinds) { include WireValue }
  RecognizeDetail = Data.define(:pieces, :names) { include WireValue }
end
