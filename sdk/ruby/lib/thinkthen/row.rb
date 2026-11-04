# frozen_string_literal: true

module ThinkThen
  module RowPredicates
    def answered? = status == :answered
    def unsure? = status == :unsure
    def failed? = status == :failed
  end
  # Read an answer through Ruby's predicates and conversion protocols.
  module AnswerValue
    include Comparable
    # Comparable's answer ordering must not replace Data's complete value equality.
    def ==(other) = Data.instance_method(:==).bind_call(self, other)
    def yes? = answer == true
    def no? = answer == false
    def found? = !answer.nil?
    def to_s
      value = answer
      value.is_a?(Found) ? value.text : value.to_s
    end
    def to_f = answer.to_f
    def to_i = answer.to_i
    def to_sym = answer.to_sym
    def to_a = answer.to_a
    def empty? = answer.empty?
    def include?(value) = answer.include?(value)
    def each(&block) = answer.each(&block)
    def <=>(other) = answer <=> other
    def [](name) = answer[name]
    def to_json(*_args) = JSON.generate(to_h)
    def inspect
      value = answered? ? answer.inspect : status.to_s
      "#<ThinkThen::#{self.class.name.to_s.split('::').last} #{value}>"
    end
  end
  # Annotation names accept symbols and unknown names always raise KeyError.
  class AnnotationAnswers < Hash
    def [](name) = fetch(name.to_s)
    def fetch(name) = super(name.to_s)
  end
  module RowWire
    include RowPredicates
    include AnswerValue
    def id_present? = id_present
    def to_h
      # @type var klass: untyped
      klass = self.class
      result = klass.members.to_h { |key| [key.to_s, Values.wire(public_send(key))] }
      result.delete("id_present")
      result.delete("id") unless id_present?
      result.delete("probability") if result["probability"].nil?
      result.delete("detail") if result["detail"].nil?
      result
    end
  end
  # A definite answer with input position, identity, provenance and optional typed detail.
  Answered = Data.define(:function, :position, :id, :id_present, :source, :status, :answer, :probability, :detail) { include RowWire }
  # An uncertain outcome. Reading its answer or conversions raises UnsureError with its probability.
  Unsure = Data.define(:function, :position, :id, :id_present, :source, :status, :probability, :detail) { include RowWire }
  # A failed record with its position and error. Reading its answer raises RecordError.
  Failed = Data.define(:function, :position, :id, :id_present, :source, :status, :error) { include RowWire }
  module AnnotationWire
    include WireValue
    include AnswerValue
    def to_h
      result = super
      result["answer"] = Values.wire(answer) if answered?
      result
    end
  end
  # One definite named annotation answer; the containing row holds input metadata.
  AnnotationAnswered = Data.define(:function, :status, :answer, :probability, :detail) do
    include AnnotationWire
    include RowPredicates
  end
  # One uncertain named annotation answer. Reading its answer raises UnsureError.
  AnnotationUnsure = Data.define(:function, :status, :probability, :detail) do
    include AnnotationWire
    include RowPredicates
  end
  # One failed named annotation answer. Reading its answer raises RecordError.
  AnnotationFailed = Data.define(:function, :status, :error) do
    include AnnotationWire
    include RowPredicates
  end
  module UnsureAnswer
    def answer = raise UnsureError.new(probability)
  end
  module FailedAnswer
    def answer
      raise RecordError.new(error.message, code: error.code, http_status: error.http_status, rows: [self])
    end
  end
  class Unsure; include UnsureAnswer; end
  class AnnotationUnsure; include UnsureAnswer; end
  class Failed; include FailedAnswer; end
  class AnnotationFailed; include FailedAnswer; end
  module InputRow
    include WireValue
    def answer = row.answer
    def probability = row.probability
    def detail = row.detail
    def kept?
      value = row
      value.is_a?(Answered) && value.answer == true
    end
    def answered? = row.answered?
    def unsure? = row.unsure?
    def failed? = row.failed?
    def function = row.function
    def position = row.position
    def id = row.id
    def id_present? = row.id_present?
    def source = row.source
    def status = row.status
  end
  # A filter decision with its original input record.
  FilterRow = Data.define(:item, :row) { include InputRow }
  module RankCopy
    # An equal copy of the record. A record with no JSON form failed, and reading it raises that RecordError.
    def item
      value = super
      value.equal?(Native::NO_COPY) ? row.answer : value
    end
  end
  # A rank decision with an equal copy of its input record, read back from the engine.
  RankRow = Data.define(:item, :row) { include InputRow }
  class RankRow
    prepend RankCopy
    # The engine's rank number, starting at 1; raises like +answer+ on not sure or failure.
    def rank = row.answer
    # Contract data; a record with no JSON form has no item.
    def to_h
      stored = Data.instance_method(:to_h).bind_call(self).fetch(:item)
      {"item" => stored.equal?(Native::NO_COPY) ? nil : Values.wire(stored), "row" => row.to_h}.reject { |_, value| value.nil? }
    end
  end
  module Row
    def self.decode(native)
      function = Native.discriminator(NativeContract::FUNCTION_NAMES, native[:function])
      status = Native.discriminator(NativeContract::STATUS_NAMES, native[:status])
      # @type var fields: Hash[String, untyped]
      fields = {"function" => function, "status" => status, "position" => native[:position]}
      %i[id answer source error detail].each do |key|
        fields[key.to_s] = Native.parse(Native.copy(native[key])) unless native[key][:data].null?
      end
      fields["probability"] = native[:probability] unless native[:probability].nan?
      if status == "failed"
        code = Native.thinkthen_code_name(native[:error_code])
        if code && NativeContract::CODE_NAMES.key?(native[:error_code])
          raise UsageError.new(NativeContract::ABI_MESSAGE, code: NativeContract::ABI_CODE) unless fields.fetch("error").fetch("code") == code
        else
          fields.fetch("error")["raw_code"] = native[:error_code]
          fields.fetch("error")["raw_name"] = code || fields.fetch("error").fetch("code")
          fields.fetch("error")["code"] = "unknown"
        end
      end
      source = fields.fetch("source")
      metadata = Source.new(question_identity: Values.freeze_copy(source.fetch("question_identity")),
                            backend: source.fetch("backend").to_sym, model: Values.freeze_copy(source.fetch("model")),
                            name: Values.freeze_copy(source["name"]))
      shared = {position: fields.fetch("position"), id: Values.freeze_copy(fields["id"]), id_present: fields.key?("id"), source: metadata}
      variant(fields, shared)
    end
    def self.variant(fields, shared = nil)
      function, status = fields.fetch("function"), fields.fetch("status")
      common = {function: function.to_sym, status: status.to_sym, **(shared || {})}
      # @type var classes: untyped
      classes = shared ? [Answered, Unsure, Failed] : [AnnotationAnswered, AnnotationUnsure, AnnotationFailed]
      return classes[2].new(**common, error: row_error(fields.fetch("error"))) if status == "failed"
      result = {**common, probability: fields["probability"], detail: detail(function, fields["detail"])}
      return classes[1].new(**result) if status == "unsure"
      classes[0].new(**result, answer: answer(function, fields["answer"]))
    end
    def self.value(klass, fields)
      # @type var values: Hash[Symbol, untyped]
      values = klass.members.to_h { |key| [key, Values.freeze_copy(fields[key.to_s])] }
      klass.new(**values)
    end
    def self.row_error(fields)
      name = fields.fetch("code")
      known = NativeContract::CODE_NAMES.value?(name)
      RowError.new(code: known ? Native.error_symbol(name) : :unknown, message: fields.fetch("message"),
                   http_status: fields["http_status"], raw_code: fields["raw_code"],
                   raw_name: fields["raw_name"] || (known ? nil : name))
    end
    def self.relation(fields)
      value(Relation, fields.merge("symmetric" => fields["either"], "source" => value(RelationName, fields.fetch("source")),
                                   "target" => value(RelationName, fields.fetch("target"))))
    end
    def self.answer(function, fields)
      case function
      when "find" then fields && value(Found, fields)
      when "recognize" then fields.map { |name| value(Name, name) }.freeze
      when "relate" then fields.map { |entry| relation(entry) }.freeze
      when "annotate" then AnnotationAnswers[fields.transform_values { |entry| variant(entry) }].freeze
      when "score" then fields.to_f
      else Values.freeze_copy(fields)
      end
    end
    def self.detail(function, fields)
      return nil unless fields
      case function
      when "choose", "tag" then value(LabelDetail, fields)
      when "score", "rank" then value(LevelDetail, fields)
      when "relate" then RelationDetail.new(names: fields.fetch("names").map { |name| value(Name, name) }.freeze, pairs: fields.fetch("pairs").map { |entry| relation(entry) }.freeze)
      when "recognize"
        pieces = fields.fetch("pieces").map { |entry| value(PieceDetail, entry) }.freeze
        names = fields.fetch("names").map { |entry| value(KindDetail, entry) }.freeze
        RecognizeDetail.new(pieces: pieces, names: names)
      end
    end
  end
  private_constant :Row, :Values, :RowWire, :RowPredicates, :AnnotationWire, :WireValue, :AnswerValue, :AnnotationAnswers, :NativeContract, :UnsureAnswer, :FailedAnswer, :InputRow, :RankCopy
end
