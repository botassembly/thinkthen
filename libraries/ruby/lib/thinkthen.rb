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

  # One name `recognize` found. `start` and `end` count Ruby characters, so
  # `text[start...end]` is the name. `strength` is the settled name for the
  # number on a name: ours, computed — the least word probability behind the
  # name times the average kind probability — with its parts under details.
  # A relation's number is `probability`, a Jev number passed through.
  Entity = Struct.new(:id, :text, :kind, :start, :end, :strength)

  # One relation between two names, by entity id. The ends are `source`
  # and `target` on every surface.
  Relation = Struct.new(:name, :source, :target, :probability)

  # What `recognize` returned: the names, and the relations when a rule
  # was given.
  Recognized = Struct.new(:entities, :relations)

  # One edge `relate` found, by record numbers counted from 1 in input
  # order. `source` is the subject and `target` the object; an `either`
  # edge prints once with the lower number in `source`.
  Edge = Struct.new(:name, :source, :target, :probability, :source_kind, :target_kind)

  # The ruled pair `rank` returns: the record's place in the input, the
  # record itself, and the probability the backend gave it. Most likely
  # yes first; ties keep input order (settled 2026-09-21). It prints as
  # its record, because the deck's sample prints ranked records directly.
  Ranked = Struct.new(:index, :record, :probability) do
    def to_s
      record.to_s
    end
  end

  # The ruled pair `find` returns: the winning unit's place, the unit
  # itself, and its probability. `index` and `unit` are nil when nothing
  # fits (settled 2026-09-21).
  Found = Struct.new(:index, :unit, :probability)

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
      ordered = placed.map { |index, probability| Ranked.new(index, list[index], probability) }
      top ? ordered.first(top) : ordered
    end

    def find(question, units, cancel: nil, deadline: nil)
      list = units.to_a
      index, probability = @engine.find(built(question), list.map(&:to_s), cancel, deadline)
      Found.new(index, index.nil? ? nil : list[index], probability)
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

    # Find every name in a text and say what kind it is.
    #
    # The deck's call, as drawn:
    #
    #   found = ThinkThen.recognize(text, kinds: %w[person organization place],
    #                               relations: { works_for: %w[person organization] })
    #   found.entities.first.kind  # "person"
    #
    # `text[start...end]` slices the name out of the original text in Ruby
    # characters. `nil` keeps the file grammar's bars. The number on a name
    # is `entity.strength`; a relation's is `probability`. A relation value
    # is a [from, to] pair, each end a kind or the one-character string
    # "*".
    def recognize(text, kinds: nil, relations: nil, threshold: nil,
                  relation_threshold: nil, cancel: nil, deadline: nil)
      spec = recognize_spec(kinds, relations, threshold, relation_threshold)
      answer = JSON.parse(@engine.recognize_json(JSON.generate(spec), text.to_s, cancel, deadline))
      Recognized.new(
        answer.fetch("entities").map do |one|
          Entity.new(one["id"], one["text"], one["kind"], one["start"], one["end"],
                     one["strength"])
        end,
        answer.fetch("relations", []).map do |one|
          Relation.new(one["name"], one["source"], one["target"], one["probability"])
        end
      )
    end

    # Say how the records relate to each other: one question per legal
    # pair.
    #
    #   edges = ThinkThen.relate(alerts, relations: %w[caused_by], either: %w[same_as])
    #   edges[0].name, edges[0].source, edges[0].target, edges[0].probability
    #
    # Every record crosses at once, and more than 255 refuses with a usage
    # error before any question is asked. `source` and `target` are record
    # numbers counted from 1 in input order. An `either` rule reads the
    # same both ways and prints once per pair.
    def relate(records, relations: nil, either: nil, threshold: nil,
               kind_field: nil, cancel: nil, deadline: nil)
      list = records.to_a
      spec = relate_spec(relations, either, threshold, kind_field)
      answer = JSON.parse(@engine.relate_json(JSON.generate(spec), list.map(&:to_s), cancel, deadline))
      answer.fetch("edges").map do |one|
        Edge.new(one["name"], one["source"], one["target"], one["probability"],
                 one["source_kind"], one["target_kind"])
      end
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

    # The recognize spec in the contract's one grammar: kinds as a list,
    # relations as named rules with from and to ends (each a kind or "*"),
    # and both bars.
    def recognize_spec(kinds, relations, threshold, relation_threshold)
      spec = {}
      spec["kinds"] = kinds.map(&:to_s) if kinds
      spec["relations"] = relation_rules(relations) if relations
      spec["threshold"] = threshold unless threshold.nil?
      spec["relation_threshold"] = relation_threshold unless relation_threshold.nil?
      spec
    end

    # The relate spec: a bare name means any kind to any kind; `either`
    # names the rules that read the same both ways.
    def relate_spec(relations, either, threshold, kind_field)
      spec = {}
      spec["relations"] = relations.flat_map { |one| one.is_a?(Hash) ? relation_rules(one) : one.to_s } if relations
      spec["either"] = either.map(&:to_s) if either
      spec["kind_field"] = kind_field.to_s unless kind_field.nil?
      spec["threshold"] = threshold unless threshold.nil?
      spec
    end

    # A relations value — a Hash of name to [source, target] or to a Hash
    # with source, target, and either — as the grammar's rule list.
    def relation_rules(relations)
      relations.map do |name, ends|
        if ends.is_a?(Hash)
          rule = { "name" => name.to_s, "source" => ends[:source].to_s, "target" => ends[:target].to_s }
          rule["either"] = true if ends[:either]
          rule
        else
          source, target = ends
          { "name" => name.to_s, "source" => source.to_s, "target" => target.to_s }
        end
      end
    end

    def choose_question(question, options)
      if question.is_a?(Question)
        raise UsageError.new("choose: a question value carries its own options; pass the options on the question, not beside it", "usage") if options

        return built(question)
      end

      body = question_body("choose", question)
      body["options"] = options if options
      _parse_question(JSON.generate(body))
    end

    def score_question(question, levels)
      if question.is_a?(Question)
        raise UsageError.new("score: a question value carries its own levels; pass the levels on the question, not beside it", "usage") if levels

        return built(question)
      end

      body = question_body("score", question)
      body["levels"] = levels if levels
      _parse_question(JSON.generate(body))
    end

    def tag_question(question, labels)
      if question.is_a?(Question)
        raise UsageError.new("tag: a question value carries its own labels; pass the labels on the question, not beside it", "usage") if labels

        return built(question)
      end

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
