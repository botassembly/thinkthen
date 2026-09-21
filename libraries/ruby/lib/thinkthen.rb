# frozen_string_literal: true

# The Ruby surface over the thinkthen contract.
#
# The eight verbs are module methods on ThinkThen. A question is built with
# ThinkThen.question(decide: "...", threshold: 0.2..0.8) and any verb that
# takes a question also takes the bare question text, which builds through
# the one file grammar and inherits its default cut. A Range threshold is
# the band, becoming the file grammar's "0.2:0.8" string inside the
# question. nil is "not sure".
#
# Bulk calls take any Enumerable, cross into the engine once, and run at
# the process width. Nothing here retries, sends, or holds a rule: the
# wrapper owns keywords and record handling only.

require "json"
require_relative "thinkthen/thinkthen"
require_relative "thinkthen/version"

module ThinkThen
  # The error classes carry the contract's kind and retry signal.
  [UsageError, BackendError, DeadlineError, LocalError, CancelledError, DefectError].each do |klass|
    klass.attr_reader :kind, :retryable
    klass.define_method(:initialize) do |message, kind, retryable = false|
      super(message)
      @kind = kind
      @retryable = retryable
    end
  end

  # One shared engine value: no thread is held between calls, and the
  # process keeps one width gate and one set of counters.
  @engine = Native::Engine.new
  @tick_engine = Native::Engine.new

  class << self
    # Build a question from keywords: the verb key (decide, choose, score,
    # tag), its text, and threshold, options, levels, or labels. Returns a
    # built question any verb accepts.
    def question(**keywords)
      keys = keywords.keys.map(&:to_s)
      verb = %w[decide choose score tag].find { |one| keys.include?(one) }
      raise ArgumentError, "question needs one of decide, choose, score, or tag" unless verb

      body = { verb => keywords[verb.to_sym] }
      if keywords.key?(:threshold)
        threshold = keywords[:threshold]
        body["threshold"] = threshold.is_a?(Range) ? "#{threshold.begin}:#{threshold.end}" : threshold
      end
      body["options"] = keywords[:options] if keywords.key?(:options)
      body["levels"] = keywords[:levels] if keywords.key?(:levels)
      body["labels"] = keywords[:labels] if keywords.key?(:labels)
      _parse_question(JSON.generate(body))
    end

    # A question set: a path to a question-file set, or built keywords.
    def set(path = nil, **keywords)
      return _parse_set(File.read(path)) if path

      body = { "version" => 1, "questions" => {} }
      keywords.each { |name, spec| body["questions"][name.to_s] = spec.is_a?(Question) ? JSON.parse(spec.json) : spec }
      _parse_set(JSON.generate(body))
    end

    # A built question, or a question's own text with its default cut.
    def built(value)
      return value if value.is_a?(Question)
      return _parse_question(JSON.generate({ "decide" => value.to_s })) if value.is_a?(String)

      raise ArgumentError, "a question is built text or the question's own text"
    end

    def decide(question, evidence, cancel: nil, deadline: nil)
      @engine.decide(built(question), evidence.to_s, cancel, deadline)
    end

    def decide_many(question, records, cancel: nil, deadline: nil)
      list = records.to_a
      @engine.decide_many(built(question), list.map(&:to_s), cancel, deadline, nil)
    end

    # The bulk answer with each judgment's probability beside it, for
    # callers and conformance checks that want both.
    def decide_many_with_probabilities(question, records, cancel: nil, deadline: nil)
      list = records.to_a
      pairs = @engine.decide_many(built(question), list.map(&:to_s), cancel, deadline, nil)
      # The bare answers carry the judgment; probabilities come from one
      # details pass a record, which the null backend answers for free.
      pairs.each_with_index.map do |answer, place|
        { answer: answer, probability: @engine.details(built(question), list[place].to_s, cancel, deadline)["probability"] }
      end
    end

    def filter(question, records, cancel: nil, deadline: nil)
      list = records.to_a
      kept = @engine.filter(built(question), list.map(&:to_s), cancel, deadline, nil)
      kept.map { |index| list[index] }
    end

    def rank(question, records, top: nil, cancel: nil, deadline: nil)
      list = records.to_a
      placed = @engine.rank(built(question), list.map(&:to_s), cancel, deadline, nil)
      ordered = placed.map { |index, _probability| list[index] }
      top ? ordered.first(top) : ordered
    end

    def find(question, units, cancel: nil, deadline: nil)
      list = units.to_a
      index, = @engine.find(built(question), list.map(&:to_s), cancel, deadline)
      index.nil? ? nil : list[index]
    end

    def choose(question, evidence, options: nil, cancel: nil, deadline: nil)
      question = choose_question(question, options)
      @engine.choose(question, evidence.to_s, cancel, deadline)
    end

    def score(question, evidence, levels: nil, cancel: nil, deadline: nil)
      question = score_question(question, levels)
      @engine.score(question, evidence.to_s, cancel, deadline).first
    end

    # The score with its nearest level beside it, for callers that want
    # the level's name without a second call.
    def score_with_level(question, evidence, levels: nil, cancel: nil, deadline: nil)
      question = score_question(question, levels)
      value, nearest = @engine.score(question, evidence.to_s, cancel, deadline)
      [value, nearest]
    end

    def tag(question, evidence, labels: nil, cancel: nil, deadline: nil)
      question = tag_question(question, labels)
      @engine.tag(question, evidence.to_s, cancel, deadline)
    end

    def annotate(set, records, on: nil, cancel: nil, deadline: nil)
      set = self.set(set) if set.is_a?(String)
      list = records.to_a
      evidence = on ? list.map { |record| record[on].to_s } : list.map(&:to_s)
      answers = @engine.annotate(set, evidence, cancel, deadline, nil)
      answers.each_with_index.map do |fields, place|
        if on
          record = list[place].transform_keys(&:to_sym)
          fields.each { |name, value| record[name.to_sym] = value }
          record
        else
          fields.to_h { |name, value| [name.to_sym, value] }
        end
      end
    end

    def details(question, evidence, cancel: nil, deadline: nil)
      @engine.details(built(question), evidence.to_s, cancel, deadline)
    end

    def usage
      @engine.usage
    end

    # Run one bulk call on a tick: the engine runs the block every wait
    # interval with the VM lock taken, and a raise inside it cancels the
    # call, lets sent requests finish, and re-raises. The interrupt path
    # for a host that owns its own signals.
    def with_tick(&tick)
      @tick_engine.instance_variable_set(:@tick, tick)
      @tick_engine
    end

    private

    def choose_question(question, options)
      return built(question) if options.nil? && question.is_a?(Question)

      body = question_body("choose", question)
      body["options"] = options if options
      _parse_question(JSON.generate(body))
    end

    def score_question(question, levels)
      return built(question) if levels.nil? && question.is_a?(Question)

      body = question_body("score", question)
      body["levels"] = levels if levels
      _parse_question(JSON.generate(body))
    end

    def tag_question(question, labels)
      return built(question) if labels.nil? && question.is_a?(Question)

      body = question_body("tag", question)
      body["labels"] = labels if labels
      _parse_question(JSON.generate(body))
    end

    def question_body(verb, question)
      text = question.is_a?(Question) ? JSON.parse(question.json)[verb] : question.to_s
      { verb => text }
    end
  end
end
