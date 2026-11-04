# frozen_string_literal: true

module ThinkThen
  # An immutable question validated once by the native engine.
  class Question
    attr_reader :value
    def initialize(function, text = nil, **options)
      raise UsageError.new("question must be a String", code: :"usage.question") unless text.nil? || text.is_a?(String)
      ThinkThen.send(:native)
      Native.validate_threshold(function, text, options[:threshold])
      @value = Values.freeze_copy({"function" => function.to_s, **(text.nil? ? {} : {"question" => text}), **options.transform_keys(&:to_s)})
      Native.validate_question(@value)
      freeze
    end
    def self.field(value)
      return value if value.nil? || value.is_a?(String)
      raise UsageError.new("field must be a pointer or path", code: :"usage.question") unless value.is_a?(Array)
      value.map do |part|
        case part
        when String, Symbol then "/" + part.to_s.gsub("~", "~0").gsub("/", "~1")
        when Integer
          raise UsageError.new("path indices must be nonnegative", code: :"usage.question") if part.negative?
          "/#{part}"
        else raise UsageError.new("path components must be strings, symbols or integers", code: :"usage.question")
        end
      end.join
    end
    def self.threshold(value, band:)
      return value if value.nil? || value.is_a?(Float)
      if band && value.is_a?(Range)
        raise UsageError.new("threshold range must have inclusive endpoints", code: :"usage.question") if value.exclude_end? || value.begin.nil? || value.end.nil?
        raise UsageError.new("threshold endpoints must be Float", code: :"usage.question") unless value.begin.is_a?(Float) && value.end.is_a?(Float)
        return {"low" => value.begin, "high" => value.end}
      end
      raise UsageError.new("threshold must be a Float#{band ? ' or inclusive Range' : ''}", code: :"usage.question")
    end
    def self.list(field, values, descriptions = nil)
      unless values.is_a?(Array) && values.all? { |value| value.is_a?(String) }
        raise UsageError.new("expected an Array of String", code: :"usage.question")
      end
      unless descriptions.nil? || descriptions.is_a?(Hash)
        raise UsageError.new("descriptions must be a Hash", code: :"usage.question")
      end
      descriptions = descriptions&.transform_keys(&:to_s)
      options = values.map do |value|
        description = descriptions && descriptions.delete(value.to_s)
        description.nil? ? value.to_s : {"name" => value.to_s, "description" => description}
      end
      # @type var result: Hash[Symbol, untyped]
      result = {field => options}
      # Unmatched keys go to shared admission, which owns their refusal.
      result[:descriptions] = descriptions if descriptions && !descriptions.empty?
      result
    end
  end
  # A yes-or-no question with an optional cut or uncertainty band.
  class DecideQuestion < Question
    def initialize(text, threshold: nil, field: nil, yes_means: nil, no_means: nil)
      super(:decide, text, **{true: yes_means, false: no_means, threshold: Question.threshold(threshold, band: true), field: Question.field(field)}.reject { |_, value| value.nil? })
    end
  end
  # A question selecting one named choice.
  class ChooseQuestion < Question
    def initialize(text, choices:, threshold: nil, field: nil, descriptions: nil)
      super(:choose, text, **{**Question.list(:choices, choices, descriptions), threshold: Question.threshold(threshold, band: true), field: Question.field(field)}.reject { |_, value| value.nil? })
    end
  end
  # A question selecting every label above the threshold.
  class TagQuestion < Question
    def initialize(text, labels:, threshold: nil, field: nil, descriptions: nil)
      super(:tag, text, **{**Question.list(:labels, labels, descriptions), threshold: Question.threshold(threshold, band: false), field: Question.field(field)}.reject { |_, value| value.nil? })
    end
  end
  # A question scoring text on ordered levels.
  class ScoreQuestion < Question
    def initialize(text, levels:, field: nil, descriptions: nil)
      super(:score, text, **{**Question.list(:levels, levels, descriptions), field: Question.field(field)}.reject { |_, value| value.nil? })
    end
  end
  # A question deciding which input records to keep.
  class FilterQuestion < Question
    def initialize(text, threshold: nil, field: nil, yes_means: nil, no_means: nil)
      super(:filter, text, **{true: yes_means, false: no_means, threshold: Question.threshold(threshold, band: true), field: Question.field(field)}.reject { |_, value| value.nil? })
    end
  end
  # A question ordering records against levels, with an optional top count.
  class RankQuestion < Question
    def initialize(text, levels:, top: nil, field: nil, descriptions: nil)
      raise UsageError.new("top must be an Integer", code: :"usage.question") unless top.nil? || top.is_a?(Integer)
      super(:rank, text, **{**Question.list(:levels, levels, descriptions), top: top, field: Question.field(field)}.reject { |_, value| value.nil? })
    end
  end
  # A question finding a text span.
  class FindQuestion < Question
    def initialize(text, threshold: nil, field: nil)
      super(:find, text, **{threshold: Question.threshold(threshold, band: false), field: Question.field(field)}.reject { |_, value| value.nil? })
    end
  end
  # A set of kinds to recognize in text.
  class RecognizeQuestion < Question
    def initialize(kinds:, threshold: nil, field: nil)
      super(:recognize, nil, **{**Question.list(:kinds, kinds), threshold: Question.threshold(threshold, band: false), field: Question.field(field)}.reject { |_, value| value.nil? })
    end
  end
  # Relationship rules to evaluate between recognized names.
  class RelateQuestion < Question
    def initialize(rules:, threshold: nil, field: nil)
      raise UsageError.new("rules must be an Array", code: :"usage.question") unless rules.is_a?(Array)
      super(:relate, nil, **{rules: rules.map { |rule| raise UsageError.new("expected RelationRule", code: :"usage.question") unless rule.is_a?(RelationRule); rule.send(:definition) }, threshold: Question.threshold(threshold, band: false), field: Question.field(field)}.reject { |_, value| value.nil? })
    end
  end
  # An immutable collection of saved, named questions.
  class Toolbox
    # The immutable named question definitions.
    attr_reader :questions
    def initialize(questions:)
      raise UsageError.new("questions must be a Hash", code: :"usage.question") unless questions.is_a?(Hash)
      @questions = questions.to_h do |name, question|
        raise UsageError.new("expected named non-annotate questions", code: :"usage.question") unless (name.is_a?(String) || name.is_a?(Symbol)) && question.is_a?(Question) && !question.is_a?(AnnotateQuestion)
        [name.to_s.freeze, question]
      end.freeze
      freeze
    end
    # Fetch a named question; an unknown name raises KeyError.
    def [](name) = @questions.fetch(name.to_s)
    # Read a saved toolbox from a UTF-8 JSON file.
    def self.load_file(path) = from_json(File.read(path))
    # Read the canonical saved toolbox JSON format.
    def self.from_json(json)
      definitions = ThinkThen.send(:native).toolbox_json(json, saved: true, definitions: true)
      questions = Native.parse(definitions).transform_values do |value|
        klass = {"decide" => DecideQuestion, "choose" => ChooseQuestion, "tag" => TagQuestion,
                 "score" => ScoreQuestion, "filter" => FilterQuestion, "rank" => RankQuestion,
                 "find" => FindQuestion, "recognize" => RecognizeQuestion, "relate" => RelateQuestion}.fetch(value.fetch("function"))
        question = klass.allocate
        question.instance_variable_set(:@value, Values.freeze_copy(value))
        question.freeze
      end
      new(questions: questions)
    end
    def to_json(*_args)
      native = ThinkThen.send(:native)
      native.toolbox_json(native.context_dump(@questions.transform_values(&:value), :question))
    end
  end
  # Named questions evaluated together for one record.
  class AnnotationQuestions < Toolbox
    attr_reader :questions
    def initialize(questions:)
      super
      Native.validate_question({"function" => "annotate", "questions" => value})
    end
    def value = @questions.transform_values(&:value)
  end
  # A reusable annotation form with optional field selection.
  class AnnotateQuestion < Question
    def initialize(questions, field: nil)
      questions = AnnotationQuestions.new(questions: questions) if questions.is_a?(Hash)
      if questions.is_a?(Toolbox) && !questions.is_a?(AnnotationQuestions)
        # A Toolbox may hold rank questions; AnnotationQuestions refuses them when it is built.
        # @type var held: untyped
        held = questions.questions
        questions = AnnotationQuestions.new(questions: held)
      end
      raise UsageError.new("expected named annotation questions", code: :"usage.question") unless questions.is_a?(AnnotationQuestions)
      super(:annotate, nil, questions: questions.value, **{field: Question.field(field)}.reject { |_, value| value.nil? })
    end
  end
end
