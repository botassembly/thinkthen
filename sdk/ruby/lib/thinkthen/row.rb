# frozen_string_literal: true

module ThinkThen
  # A result owns its immutable value; item remains the caller's own object.
  class Result
    undef_method :<=>
    attr_reader :function, :status, :probability, :error, :position, :id, :source, :detail, :item

    def initialize(fields, question: nil, item: nil, parent: nil, raw_code: nil)
      if parent
        fields = fields.merge("position" => parent.position, "source" => parent.source.to_h)
        fields = fields.merge("id" => parent.id) if parent.id_present?
      end
      @wire = Values.freeze_copy(fields)
      @function, @status = fields.fetch("function").to_sym, fields.fetch("status").to_sym
      @position, @id, @item = parent ? [parent.position, parent.id, parent.item] : [fields["position"], Values.freeze_copy(fields["id"]), item]
      @source = parent ? parent.source : Row.value(Source, fields.fetch("source"))
      @probability = fields["probability"]
      @error = fields["error"] && Row.row_error(fields["error"], raw_code: raw_code)
      @detail = Row.detail(fields.fetch("function"), fields["detail"])
      @answer = answered? ? Row.answer(fields.fetch("function"), fields["answer"], question, self) : nil
      @value = [function, status, @answer, probability, Values.freeze_copy(@error&.to_h), position, id, source, detail].freeze
      freeze
    end
    private_class_method :new

    def answered? = status == :answered
    def unsure? = status == :unsure
    def failed? = status == :failed
    def id_present? = @wire.key?("id")

    def answer
      raise UnsureError.new(self) if unsure?
      raise_failure(self) if failed?
      @answer
    end

    def raise_if_failed
      failed = failed? ? self : function == :annotate && @answer&.each_value&.find(&:failed?)
      raise_failure(failed) if failed
      self
    end

    def ==(other)
      raise TypeError, "compare the result's .answer" unless other.is_a?(Result)
      eql?(other)
    end
    def eql?(other) = other.is_a?(Result) && @value.eql?(other.instance_variable_get(:@value))
    def hash = @value.hash
    def to_h = Values.wire(@wire)
    def to_json(*_args) = JSON.generate(@wire)
    def to_s = Row.text(self, @answer)
    def inspect = "#<#{self.class.name} #{to_s}>"

    private

    def raise_failure(row)
      # Native failed outcomes carry an error; the public getter remains optional.
      # @type var failure: untyped
      failure = row.error
      raise FailedError.new(failure.message, code: failure.code, http_status: failure.http_status, rows: [row])
    end
  end

  class DecideRow < Result
    def yes? = answer
  end
  class ChooseRow < Result; end
  class TagRow < Result; end
  class ScoreRow < Result; end
  class RankRow < Result; end
  class FindRow < Result; end
  class RecognizeRow < Result; end
  class RelateRow < Result; end
  class AnnotateRow < Result; end

  # Symbols and strings address the same canonical question name, loudly.
  class AnnotationAnswers < Hash
    def [](name) = fetch(name)
    def fetch(name) = super(name.to_s)
  end

  module Row
    def self.decode(native, question: nil, item: nil)
      variant(Native.parse(Native.row_json(native)), question: question, item: item, raw_code: native[:error_code])
    end

    def self.variant(fields, question: nil, item: nil, parent: nil, raw_code: nil)
      function = fields.fetch("function")
      klass = function == "filter" ? Result : ThinkThen.const_get("#{function.capitalize}Row")
      klass.send(:new, fields, question: question, item: item, parent: parent, raw_code: raw_code)
    end

    def self.value(klass, fields)
      # @type var values: Hash[Symbol, untyped]
      values = klass.members.to_h { |key| [key, Values.freeze_copy(fields[key.to_s])] }
      klass.new(**values)
    end

    def self.row_error(fields, raw_code: nil)
      name = fields.fetch("code")
      known = NativeContract::CODE_NAMES.value?(name)
      RowError.new(code: known ? Native.error_symbol(name) : :unknown, message: Values.freeze_copy(fields.fetch("message")),
                   http_status: fields["http_status"], raw_code: known ? fields["raw_code"] : raw_code || fields["raw_code"],
                   raw_name: fields["raw_name"] || (known ? nil : name)).freeze
    end

    def self.relation(fields)
      value(Relation, fields.merge("either" => fields.fetch("either", false), "source" => value(RelationName, fields.fetch("source")),
                                   "target" => value(RelationName, fields.fetch("target"))))
    end

    def self.answer(function, fields, question, parent)
      case function
      when "find" then fields && value(Span, fields)
      when "recognize" then fields.map { |entry| value(Name, entry) }.freeze
      when "relate" then fields.map { |entry| relation(entry) }.freeze
      when "annotate"
        # @type var questions: Hash[String, untyped]
        questions = question.is_a?(AnnotateQuestion) ? question.questions : {}
        AnnotationAnswers[fields.to_h { |name, entry| [name, variant(entry, question: questions[name], parent: parent)] }].freeze
      when "choose" then question&.symbols? ? fields.to_sym : Values.freeze_copy(fields)
      when "tag" then fields.map { |entry| question&.symbols? ? entry.to_sym : Values.freeze_copy(entry) }.freeze
      when "score" then fields.to_f
      else Values.freeze_copy(fields)
      end
    end

    def self.detail(function, fields)
      return nil unless fields
      case function
      when "choose", "tag" then value(LabelDetail, fields)
      when "score", "rank" then value(LevelDetail, fields)
      when "relate"
        RelationDetail.new(names: fields.fetch("names").map { |entry| value(Name, entry) }.freeze,
                           pairs: fields.fetch("pairs").map { |entry| relation(entry) }.freeze)
      when "recognize"
        RecognizeDetail.new(pieces: fields.fetch("pieces").map { |entry| value(PieceDetail, entry) }.freeze,
                            names: fields.fetch("names").map { |entry| value(KindDetail, entry) }.freeze)
      end
    end

    def self.number(value)
      parts = value.to_s.split("e")
      mantissa, exponent = parts.fetch(0), parts[1]
      return mantissa.sub(/\.0\z/, "") unless exponent
      "0." + "0" * (-exponent.to_i - 1) + mantissa.delete(".").sub(/0+\z/, "")
    end

    # Fold only string leaves. The tabs joining labels/entries remain separators.
    def self.leaf(value) = value.to_s.tr("\r\n\t", "   ")
    def self.text(row, answer)
      return "unsure" if row.unsure?
      if row.failed?
        # @type var failure: untyped
        failure = row.error
        return "failed: #{leaf(failure.raw_name || failure.code)}: #{leaf(failure.message)}"
      end
      case row.function
      when :decide, :filter then answer ? "true" : "false"
      when :choose then leaf(answer)
      when :tag then answer.map { |label| leaf(label) }.join("\t")
      when :score then number(answer)
      when :rank then answer.to_s
      when :find then answer ? leaf(answer.text) : ""
      when :recognize then answer.map { |name| "#{leaf(name.kind)}: #{leaf(name.text)} (#{name.start}..#{name.end})" }.join("\t")
      when :relate then answer.map { |entry| "#{leaf(entry.source.text)} --#{leaf(entry.relation)}--> #{leaf(entry.target.text)}" }.join("\t")
      when :annotate then answer.map { |name, entry| "#{leaf(name)}=#{entry}" }.join("\t")
      end
    end
  end
  private_constant :Row, :Values, :WireValue, :AnnotationAnswers, :NativeContract
end
