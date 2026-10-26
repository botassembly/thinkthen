# frozen_string_literal: true

module ThinkThen
  class Question
    attr_reader :value
    def initialize(function, text = nil, **options)
      raise TypeError, "question must be a String" unless text.nil? || text.is_a?(String)
      ThinkThen.send(:native)
      Native.validate_threshold(function, text, options[:threshold])
      @value = Values.freeze_copy({"function" => function.to_s, **(text.nil? ? {} : {"question" => text}), **options.transform_keys(&:to_s)})
      Native.validate_question(@value)
      freeze
    end
    def self.evidence(value)
      return value if value.nil? || value.is_a?(String)
      raise TypeError, "evidence must be a pointer or path" unless value.is_a?(Array)
      value.map do |part|
        case part
        when String, Symbol then "/" + part.to_s.gsub("~", "~0").gsub("/", "~1")
        when Integer
          raise ArgumentError, "path indices must be nonnegative" if part.negative?
          "/#{part}"
        else raise TypeError, "path components must be strings, symbols or integers"
        end
      end.join
    end
    def self.threshold(value, band:)
      return value if value.nil? || value.is_a?(Float)
      if band && value.is_a?(Range)
        raise ArgumentError, "threshold range must have inclusive endpoints" if value.exclude_end? || value.begin.nil? || value.end.nil?
        raise TypeError, "threshold endpoints must be Float" unless value.begin.is_a?(Float) && value.end.is_a?(Float)
        return {"low" => value.begin, "high" => value.end}
      end
      raise TypeError, "threshold must be a Float#{band ? ' or inclusive Range' : ''}"
    end
    def self.list(values)
      raise TypeError, "expected an Array of String" unless values.is_a?(Array) && values.all? { |value| value.is_a?(String) }
      values
    end
  end
  class DecideQuestion < Question
    def initialize(text, threshold: nil, evidence: nil)
      super(:decide, text, **{threshold: Question.threshold(threshold, band: true), evidence: Question.evidence(evidence)}.reject { |_, value| value.nil? })
    end
  end
  class ChooseQuestion < Question
    def initialize(text, choices:, threshold: nil, evidence: nil)
      super(:choose, text, **{choices: Question.list(choices), threshold: Question.threshold(threshold, band: true), evidence: Question.evidence(evidence)}.reject { |_, value| value.nil? })
    end
  end
  class TagQuestion < Question
    def initialize(text, labels:, threshold: nil, evidence: nil)
      super(:tag, text, **{labels: Question.list(labels), threshold: Question.threshold(threshold, band: false), evidence: Question.evidence(evidence)}.reject { |_, value| value.nil? })
    end
  end
  class ScoreQuestion < Question
    def initialize(text, levels:, evidence: nil)
      super(:score, text, **{levels: Question.list(levels), evidence: Question.evidence(evidence)}.reject { |_, value| value.nil? })
    end
  end
  class FilterQuestion < Question
    def initialize(text, threshold: nil, evidence: nil)
      super(:filter, text, **{threshold: Question.threshold(threshold, band: true), evidence: Question.evidence(evidence)}.reject { |_, value| value.nil? })
    end
  end
  class RankQuestion < Question
    def initialize(text, levels:, top: nil, evidence: nil)
      raise TypeError, "top must be an Integer" unless top.nil? || top.is_a?(Integer)
      super(:rank, text, **{levels: Question.list(levels), top: top, evidence: Question.evidence(evidence)}.reject { |_, value| value.nil? })
    end
  end
  class FindQuestion < Question
    def initialize(text, threshold: nil, evidence: nil)
      super(:find, text, **{threshold: Question.threshold(threshold, band: false), evidence: Question.evidence(evidence)}.reject { |_, value| value.nil? })
    end
  end
  class RecognizeQuestion < Question
    def initialize(kinds:, threshold: nil, evidence: nil)
      super(:recognize, nil, **{kinds: Question.list(kinds), threshold: Question.threshold(threshold, band: false), evidence: Question.evidence(evidence)}.reject { |_, value| value.nil? })
    end
  end
  class RelateQuestion < Question
    def initialize(rules:, threshold: nil, evidence: nil)
      raise TypeError, "rules must be an Array" unless rules.is_a?(Array)
      super(:relate, nil, **{rules: rules.map { |rule| raise TypeError, "expected RelationRule" unless rule.is_a?(RelationRule); rule.send(:definition) }, threshold: Question.threshold(threshold, band: false), evidence: Question.evidence(evidence)}.reject { |_, value| value.nil? })
    end
  end
  class Toolbox
    def initialize(questions:)
      raise TypeError, "questions must be a Hash" unless questions.is_a?(Hash)
      @questions = questions.to_h do |name, question|
        raise TypeError, "expected named non-annotate questions" unless name.is_a?(String) && question.is_a?(Question) && !question.is_a?(AnnotateQuestion)
        [String.new(name).freeze, question]
      end.freeze
      freeze
    end
    def [](name) = @questions.fetch(name)
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
  class AnnotationQuestions < Toolbox
    attr_reader :questions
    def initialize(questions:)
      super
      Native.validate_question({"function" => "annotate", "questions" => value})
    end
    def value = @questions.transform_values(&:value)
  end
  class AnnotateQuestion < Question
    def initialize(questions, evidence: nil)
      raise TypeError, "expected AnnotationQuestions" unless questions.is_a?(AnnotationQuestions)
      super(:annotate, nil, questions: questions.value, **{evidence: Question.evidence(evidence)}.reject { |_, value| value.nil? })
    end
  end
end
