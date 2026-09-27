# frozen_string_literal: true

# The Ruby surface over the thinkthen public Rust API.
#
# The ten verbs are module methods on ThinkThen, beside decide_many,
# details, question, set, usage, and with_tick. The same methods live on a
# ThinkThen::Engine value, which carries the engine settings. The module
# methods call one lazy engine built from the environment.
#
# A question is built with ThinkThen.question(decide: "...", threshold:
# 0.2..0.8), and a verb that takes a question also takes the bare question
# text. A Range threshold is the band. nil is unsure.
#
# Every failure raises a ThinkThen::Error. The six kind classes inherit it,
# and the binding's own refusals raise UsageError. A record that is not a
# String crosses as its JSON text, never as Ruby's to_s form.

require "json"
require_relative "thinkthen/version"

module ThinkThen
  # The one base class. `kind` is the kind's word and `retryable` the
  # engine's retry signal.
  class Error < StandardError
    attr_reader :kind, :retryable

    def initialize(message = nil, kind = nil, retryable = false)
      super(message)
      @kind = kind
      @retryable = retryable
    end
  end

  class UsageError < Error; end
  class BackendError < Error; end
  class LocalError < Error; end
  class CancelledError < Error; end
  class DeadlineError < Error; end
  class DefectError < Error; end
end

require_relative "thinkthen/thinkthen"

module ThinkThen
  # The print form of every result value. A field holding the caller's text
  # prints as its byte count, as the Rust Debug and Python repr forms do. A
  # list prints as its length. Places, probabilities, and rule names print
  # in clear.
  module Withheld
    TEXT = %i[name kind text record unit].freeze

    def inspect
      fields = each_pair.map do |name, value|
        shown = if value.nil? then "nil"
                elsif TEXT.include?(name) then "<#{value.to_s.bytesize} bytes withheld>"
                elsif value.is_a?(Array) then value.size.to_s
                else value.inspect
                end
        "#{name}=#{shown}"
      end
      "#<struct #{self.class.name} #{fields.join(", ")}>"
    end

    alias to_s inspect

    def pretty_print(printer)
      printer.text(inspect)
    end
  end

  # A name and its kind, as relate reads and returns them.
  Entity = Struct.new(:name, :kind) { include Withheld }

  # One name recognize found. `start`, `end`, and `length` count characters,
  # so `text[start...end]` is the name. `kind` is "ENTITY" when the call
  # named no kind.
  RecognizedEntity = Struct.new(:text, :start, :end, :length, :kind, :strength) { include Withheld }

  # One relation between two recognized names.
  Relation = Struct.new(:relation, :source, :target, :probability) { include Withheld }

  # What recognize returned. `relations` is nil when no rule was given.
  Recognized = Struct.new(:entities, :relations) { include Withheld }

  # One edge relate found, with Entity ends.
  Edge = Struct.new(:relation, :source, :target, :probability) { include Withheld }

  # The record's place in the input, the record, and its probability.
  # Most likely yes first. Ties keep input order. `to_s` is the record.
  Ranked = Struct.new(:index, :record, :probability) do
    include Withheld

    def to_s
      record.to_s
    end
  end

  # The selected unit's place, the unit, and its probability. Every field
  # is nil when nothing is selected.
  Found = Struct.new(:index, :unit, :probability) { include Withheld }

  # The watchdog's cadence and one in-flight call's row.
  WATCHDOG_INTERVAL = 0.1
  Row = Struct.new(:tick, :token, :error)
  private_constant :WATCHDOG_INTERVAL, :Row, :Native, :Withheld

  @rows = {}
  @rows_mutex = Mutex.new
  @default_mutex = Mutex.new

  # An engine and its settings. Omitted settings come from the environment,
  # as the module methods' engine does. The throttle is the most requests
  # in flight at once, per loaded copy of the library.
  class Engine
    def initialize(base_url: nil, model: nil, throttle: nil, max_requests: nil, cache: nil,
                   timeout: nil, max_retries: nil, record: nil, replay: nil, profile: nil)
      ThinkThen.__send__(:text_setting, :base_url, base_url)
      ThinkThen.__send__(:text_setting, :model, model)
      ThinkThen.__send__(:whole_setting, :throttle, throttle)
      ThinkThen.__send__(:whole_setting, :max_requests, max_requests)
      ThinkThen.__send__(:whole_setting, :timeout, timeout)
      ThinkThen.__send__(:whole_setting, :max_retries, max_retries)
      { record: record, replay: replay, profile: profile }.each do |name, value|
        ThinkThen.__send__(:text_setting, name, value)
      end
      unless cache.nil? || cache == false || cache.is_a?(String)
        raise UsageError.new("cache is a folder path, false for none, or nil for the default", "usage")
      end
      @native = Native.engine({ base_url: base_url, model: model, throttle: throttle,
        max_requests: max_requests, cache_at: cache || nil, no_cache: cache == false,
        timeout: timeout, max_retries: max_retries, record: record, replay: replay, profile: profile })
    end

    def self.from_native(native)
      engine = allocate
      engine.instance_variable_set(:@native, native)
      engine
    end
    private_class_method :from_native

    def inspect
      "#<ThinkThen::Engine>"
    end

    def decide(question, evidence, cancel: nil, deadline: nil)
      crossing("decide", ThinkThen.__send__(:built, question), ThinkThen.__send__(:text_of, evidence), cancel, deadline)
    end

    def decide_many(question, records, cancel: nil, deadline: nil)
      decide_many_with_probabilities(question, records, cancel: cancel, deadline: deadline).map { |row| row[:answer] }
    end

    # Each answer with its probability, from the same requests.
    def decide_many_with_probabilities(question, records, cancel: nil, deadline: nil)
      rows = crossing("decide_many", ThinkThen.__send__(:built, question), ThinkThen.__send__(:texts, records.to_a), cancel, deadline)
      rows.map { |answer, probability| { answer: answer, probability: probability } }
    end

    def filter(question, records, cancel: nil, deadline: nil)
      list = records.to_a
      places = crossing("filter", ThinkThen.__send__(:built, question), ThinkThen.__send__(:texts, list), cancel, deadline)
      places.map { |place| list[place] }
    end

    def rank(question, records, top: nil, cancel: nil, deadline: nil)
      list = records.to_a
      placed = crossing("rank", ThinkThen.__send__(:question_text, question), ThinkThen.__send__(:texts, list), cancel, deadline)
      ranked = placed.map { |place, probability| Ranked.new(place, list[place], probability) }
      top ? ranked.first(top) : ranked
    end

    # none: true offers a none candidate, as find --none does, so nothing may fit.
    def find(question, units, none: false, cancel: nil, deadline: nil)
      raise UsageError.new("none is true or false", "usage") unless [true, false].include?(none)

      list = units.to_a
      place, probability = crossing(none ? "find_none" : "find", ThinkThen.__send__(:question_text, question), ThinkThen.__send__(:texts, list), cancel, deadline)
      Found.new(place, place.nil? ? nil : list[place], probability)
    end

    def choose(question, evidence, options: nil, cancel: nil, deadline: nil)
      question = ThinkThen.__send__(:keyed, "choose", question, :options, options)
      crossing("details", question, ThinkThen.__send__(:text_of, evidence), cancel, deadline)[1]
    end

    def score(question, evidence, levels: nil, cancel: nil, deadline: nil)
      question = ThinkThen.__send__(:keyed, "score", question, :levels, levels)
      crossing("score", question, ThinkThen.__send__(:text_of, evidence), cancel, deadline)
    end

    # The position and its nearest level, from one call.
    def score_with_level(question, evidence, levels: nil, cancel: nil, deadline: nil)
      question = ThinkThen.__send__(:keyed, "score", question, :levels, levels)
      _, position, nearest = crossing("details", question, ThinkThen.__send__(:text_of, evidence), cancel, deadline)
      [position, nearest]
    end

    def tag(question, evidence, labels: nil, cancel: nil, deadline: nil)
      question = ThinkThen.__send__(:keyed, "tag", question, :labels, labels)
      crossing("details", question, ThinkThen.__send__(:text_of, evidence), cancel, deadline)[1]
    end

    # The command's --details document for one text.
    def details(question, evidence, cancel: nil, deadline: nil)
      json, = crossing("details", ThinkThen.__send__(:built, question), ThinkThen.__send__(:text_of, evidence), cancel, deadline)
      JSON.parse(json)
    end

    # One Hash per record, named by the set's questions. A set member whose
    # `on` names a part reads it from each record as JSON text. With `on:`, each
    # record is a Hash, its `on` value is the evidence, and the answers
    # join its own keys. A question landing on any record's key refuses
    # before any request.
    def annotate(set, records, on: nil, cancel: nil, deadline: nil)
      set = ThinkThen.set(set) if set.is_a?(String)
      list = records.to_a
      if on
        ThinkThen.__send__(:refuse_clashes, set, list)
        evidence = list.each_with_index.map { |record, place| ThinkThen.__send__(:text_of, record[on], "record #{place}") }
      else
        evidence = ThinkThen.__send__(:texts, list)
      end
      rows = crossing("annotate", set, evidence, cancel, deadline)
      rows.each_with_index.map do |json, place|
        answers = JSON.parse(json).transform_keys(&:to_sym)
        on ? list[place].transform_keys(&:to_sym).merge(answers) : answers
      end
    end

    # Find every name in a text and say what kind it is.
    #
    #   found = ThinkThen.recognize(text, kinds: %w[person organization place],
    #                               relations: { works_for: %w[person organization] })
    #   found.entities.first.kind  # "person"
    def recognize(text, kinds: nil, relations: nil, threshold: nil, relation_threshold: nil, cancel: nil, deadline: nil)
      spec = ThinkThen.__send__(:recognize_spec, kinds, relations, threshold, relation_threshold)
      json = crossing("recognize", JSON.generate(spec), ThinkThen.__send__(:text_of, text), cancel, deadline)
      ThinkThen.__send__(:recognized, JSON.parse(json))
    end

    # Say how named entities relate. An entity is a [name, kind] pair, a
    # Hash with name and kind, an Entity, or a RecognizedEntity. A found name,
    # or a Hash with text and no name, is named by its text.
    #
    #   edges = ThinkThen.relate([["Ana", "person"], ["Acme", "organization"]],
    #                            relations: { works_for: %w[person organization] })
    def relate(entities, relations:, either: nil, threshold: nil, cancel: nil, deadline: nil)
      spec = ThinkThen.__send__(:relate_spec, relations, either, threshold)
      pairs = entities.to_a.each_with_index.map { |entity, place| ThinkThen.__send__(:pair_of, entity, place) }
      rows = crossing("relate", JSON.generate(spec), pairs, cancel, deadline)
      rows.map do |json|
        edge = JSON.parse(json)
        Edge.new(edge["relation"], ThinkThen.__send__(:entity, edge["source"]), ThinkThen.__send__(:entity, edge["target"]), edge["probability"])
      end
    end

    # The engine's counters since the process started.
    def usage
      @native.usage
    end

    # Set this thread's tick. The watchdog runs it about ten times a second
    # while a call from this thread is in flight. A raise inside it stops the
    # call at once and surfaces from the call. A positional tick with a block
    # scopes the tick to the block's calls.
    def with_tick(tick = nil, &block)
      held = tick || block
      Thread.current[:thinkthen_tick] = held
      return self unless block && tick

      begin
        yield
      ensure
        Thread.current[:thinkthen_tick] = nil
      end
    end

    private

    def crossing(verb, subject, input, cancel, deadline)
      unless cancel.nil? || cancel.is_a?(Cancel)
        raise UsageError.new("cancel is a ThinkThen::Cancel or nil", "usage")
      end

      seconds = ThinkThen.__send__(:deadline_of, deadline)
      own = Cancel.new
      tick = Thread.current[:thinkthen_tick]
      row = tick && ThinkThen.__send__(:watch, Row.new(tick, own, nil))
      begin
        @native.call(verb, subject, input, own, cancel, seconds)
      rescue Interrupt
        raise CancelledError.new("the call was cancelled", "cancelled")
      ensure
        if row
          ThinkThen.__send__(:unwatch, row)
          raise row.error if row.error
        end
      end
    end
  end

  class << self
    # Build a question from the question file's keys: one verb key
    # (decide, choose, score, or tag) and its text, and threshold, options,
    # levels, labels, or meanings. A Range threshold is the band.
    def question(**keywords)
      body = keywords.transform_keys(&:to_s)
      threshold = body["threshold"]
      body["threshold"] = "#{threshold.begin}:#{threshold.end}" if threshold.is_a?(Range)
      Native.question(JSON.generate(body))
    end

    # A question set: a path to a set file, or questions by name.
    def set(path = nil, **questions)
      return Native.set_file(path.to_s) if path

      body = { "version" => 1, "questions" => {} }
      questions.each do |name, spec|
        body["questions"][name.to_s] = spec.is_a?(Question) ? JSON.parse(spec.json) : spec
      end
      Native.set_json(JSON.generate(body))
    end

    %i[decide decide_many decide_many_with_probabilities filter rank find choose score score_with_level
       tag details annotate recognize relate usage].each do |name|
      define_method(name) { |*args, **keywords, &block| default_engine.public_send(name, *args, **keywords, &block) }
    end

    def with_tick(tick = nil, &block)
      default_engine.with_tick(tick, &block)
    end

    private

    def default_engine
      @default_mutex.synchronize do
        @default ||= Engine.__send__(:from_native, Native.default_engine)
      end
    end

    def refuse(message)
      raise UsageError.new(message, "usage")
    end

    def text_setting(name, value)
      refuse("#{name} is text or nil") unless value.nil? || value.is_a?(String)
    end

    def whole_setting(name, value)
      refuse("#{name} is a whole number or nil") unless value.nil? || value.is_a?(Integer)
    end

    # A deadline is seconds: nil or -1 for none, 0 for spent.
    def deadline_of(value)
      return nil if value.nil?
      refuse("the deadline is seconds, a number, or nil for no deadline") unless value.is_a?(Numeric) && value.real?

      value.to_f
    end

    # The text one record crosses as: a String unchanged, anything else its
    # JSON text. nil, invalid UTF-8, and a NUL byte refuse.
    def text_of(record, where = "the evidence")
      refuse("#{where} is nil; a record is text or a JSON value") if record.nil?
      text = record.is_a?(String) ? record : JSON.generate(record)
      text = text.dup.force_encoding(Encoding::UTF_8) unless text.encoding == Encoding::UTF_8
      refuse("#{where} is not valid UTF-8") unless text.valid_encoding?
      refuse("#{where} holds a NUL byte") if text.include?("\0")
      text
    end

    def texts(list)
      list.each_with_index.map { |record, place| text_of(record, "record #{place}") }
    end

    # A built question, or a question's own text with its default cut.
    def built(value)
      return value if value.is_a?(Question)
      return Native.question(JSON.generate({ "decide" => value })) if value.is_a?(String)

      refuse("a question is a built question or its text")
    end

    # rank and find read one question text and no rule.
    def question_text(value)
      return value if value.is_a?(String)
      return JSON.parse(value.json).fetch("decide") { refuse("rank and find take a decide question") } if value.is_a?(Question)

      refuse("a question is a built question or its text")
    end

    # A choose, score, or tag question from its text and members, or a
    # built question that carries its own.
    def keyed(verb, question, key, members)
      if question.is_a?(Question)
        refuse("#{verb}: a built question carries its own #{key}; pass them on the question") if members

        return question
      end
      body = { verb => question }
      body[key.to_s] = members if members
      Native.question(JSON.generate(body))
    end

    def refuse_clashes(set, list)
      existing = list.flat_map { |record| record.keys.map(&:to_sym) }.uniq
      clash = (set.names.map(&:to_sym) & existing).first
      return unless clash

      refuse("annotate cannot add a question named '#{clash}': the input already has a key by that name; rename one")
    end

    def recognize_spec(kinds, relations, threshold, relation_threshold)
      kinds = [] if kinds.nil?
      kinds = kinds.to_h { |name| [name.to_s, nil] } if kinds.is_a?(Array)
      body = { "kinds" => kinds.to_h { |name, description| [name.to_s, description] } }
      body["relations"] = relation_rules(relations) if relations
      spec = { "version" => 1, "recognize" => body }
      spec["threshold"] = threshold unless threshold.nil?
      spec["relation_threshold"] = relation_threshold unless relation_threshold.nil?
      spec
    end

    # relations: names (any kind to any kind), a Hash of name to [source,
    # target], or the file's rule Hashes. either: the names that read the
    # same both ways.
    def relate_spec(relations, either, threshold)
      rules = relation_rules(relations)
      both = Array(either).map(&:to_s)
      rules.each { |rule| rule["either"] = true if both.include?(rule["name"]) }
      spec = { "version" => 1, "relate" => { "relations" => rules } }
      spec["threshold"] = threshold unless threshold.nil?
      spec
    end

    def relation_rules(relations)
      relations.map do |name, ends|
        next name.transform_keys(&:to_s) if name.is_a?(Hash)
        next { "name" => name.to_s, "source" => "*", "target" => "*" } if ends.nil?

        source, target = ends
        { "name" => name.to_s, "source" => source.to_s, "target" => target.to_s }
      end
    end

    def pair_of(entity, place)
      name, kind = case entity
                   when Entity then [entity.name, entity.kind]
                   when RecognizedEntity then [entity.text, entity.kind]
                   when Hash then [first_of(entity, :name) || first_of(entity, :text), first_of(entity, :kind)]
                   when Array then entity
                   else refuse("entity #{place} is a [name, kind] pair, a Hash, or an Entity")
                   end
      refuse("entity #{place} needs a name and a kind as text") unless name.is_a?(String) && kind.is_a?(String)

      [text_of(name, "entity #{place}"), text_of(kind, "entity #{place}")]
    end

    def first_of(hash, key)
      hash.key?(key) ? hash[key] : hash[key.to_s]
    end

    def entity(held)
      Entity.new(held["name"], held["kind"])
    end

    def recognized_entity(held)
      RecognizedEntity.new(*held.values_at("text", "start", "end", "length", "kind", "strength"))
    end

    def recognized(held)
      relations = held["relations"]&.map do |one|
        Relation.new(one["relation"], recognized_entity(one["source"]), recognized_entity(one["target"]), one["probability"])
      end
      Recognized.new(held.fetch("entities").map { |one| recognized_entity(one) }, relations)
    end

    def watch(row)
      @rows_mutex.synchronize { @rows[row.object_id] = row }
      watchdog
      row
    end

    def unwatch(row)
      @rows_mutex.synchronize { @rows.delete(row.object_id) }
    end

    # One thread runs every in-flight call's tick. A tick that raises keeps
    # its error in the row and fires the call's own token, and the call's
    # next wait slice stops it. The thread starts again after a fork.
    def watchdog
      return if @watchdog&.alive?

      @watchdog = Thread.new do
        Thread.current.name = "thinkthen-watchdog"
        loop do
          sleep WATCHDOG_INTERVAL
          @rows_mutex.synchronize { @rows.values }.each do |row|
            row.tick.call
          rescue Exception => e # rubocop:disable Lint/RescueException
            row.error ||= e
            row.token.cancel
          end
        end
      end
    end
  end
end
