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
#
# Every failure this surface raises is a ThinkThen::Error: the six kind
# classes below inherit it, and the wrapper's own refusals raise
# UsageError, so one `rescue ThinkThen::Error` hears them all. A record or
# an evidence text that is not a String crosses as its JSON text, never as
# Ruby's `to_s` form.

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

  # The text one record or evidence crosses as: a String unchanged,
  # anything else its JSON text, so a Hash keeps its fields and a record
  # object can speak its own `to_json`. Ruby's `to_s` form is never sent.
  # `nil` refuses: it has no honest text, and the third review caught it
  # silently crossing as the literal string "null".
  def self.text_of(record)
    return record if record.is_a?(String)
    raise UsageError.new("a record is text or a JSON-able value, not nil", "usage") if record.nil?
    JSON.generate(record)
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

  # The watchdog: one Ruby thread that runs each in-flight call's tick
  # and relays the caller's token. Each call registers a row - its own
  # cancel token, the caller's token, and the thread's tick - and the
  # watchdog polls the rows about ten times a second: the caller's fired
  # token fires the call's token, and the tick runs. It never fires a
  # token the caller shares, so one call's stop cannot cancel a sibling.
  # The host's own interrupts - Ctrl-C, Thread#raise, a raising trap - are
  # heard by the native crossing itself between its wait slices, which
  # fires the call's token and raises once sent requests finish
  # (src/lib.rs, `cross`).
  WATCHDOG_INTERVAL = 0.1
  Row = Struct.new(:thread, :token, :tick, :caller, :error)
  # Crossing internals, not the ruled surface (surfaces-review-4 and -5:
  # the check reads the loaded module and must not find them as API).
  private_constant :WATCHDOG_INTERVAL, :Row, :Native
  private_class_method :_parse_question, :_parse_set, :text_of

  @rows = {}
  @rows_mutex = Mutex.new

  class << self
    # Build a question from keywords: the verb key (decide, choose, score,
    # tag), its text, and threshold, options, levels, or labels. Returns a
    # built question any verb accepts.
    def question(**keywords)
      keys = keywords.keys.map(&:to_s)
      verb = %w[decide choose score tag].find { |one| keys.include?(one) }
      unless verb
        raise UsageError.new("question needs one of decide, choose, score, or tag", "usage")
      end

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

      raise UsageError.new("a question is built text or the question's own text", "usage")
    end

    def decide(question, evidence, cancel: nil, deadline: nil)
      crossing(cancel: cancel) { |own| @engine.decide(built(question), text_of(evidence), own, deadline) }
    end

    def decide_many(question, records, cancel: nil, deadline: nil)
      list = records.to_a
      crossing(cancel: cancel, tick: Thread.current[:thinkthen_tick]) do |own|
        @engine.decide_many(built(question), list.map { |one| text_of(one) }, own, deadline)
      end
    end

    # The bulk answer with each judgment's probability beside it, for
    # callers and conformance checks that want both. One crossing: the
    # native call carries the probabilities from the same judgments the
    # answers came from, so exposing them costs no extra request. The
    # map below only re-keys the native hashes into this surface's
    # symbol-keyed shape.
    def decide_many_with_probabilities(question, records, cancel: nil, deadline: nil)
      list = records.to_a
      crossing(cancel: cancel, tick: Thread.current[:thinkthen_tick]) do |own|
        pairs = @engine.decide_many_with_probabilities(built(question), list.map { |one| text_of(one) }, own, deadline)
        pairs.map { |pair| { answer: pair["answer"], probability: pair["probability"] } }
      end
    end

    def filter(question, records, cancel: nil, deadline: nil)
      list = records.to_a
      crossing(cancel: cancel, tick: Thread.current[:thinkthen_tick]) do |own|
        kept = @engine.filter(built(question), list.map { |one| text_of(one) }, own, deadline)
        kept.map { |index| list[index] }
      end
    end

    def rank(question, records, top: nil, cancel: nil, deadline: nil)
      list = records.to_a
      crossing(cancel: cancel, tick: Thread.current[:thinkthen_tick]) do |own|
        placed = @engine.rank(built(question), list.map { |one| text_of(one) }, own, deadline)
        ordered = placed.map { |index, probability| Ranked.new(index, list[index], probability) }
        top ? ordered.first(top) : ordered
      end
    end

    def find(question, units, cancel: nil, deadline: nil)
      list = units.to_a
      crossing(cancel: cancel) do |own|
        index, probability = @engine.find(built(question), list.map { |one| text_of(one) }, own, deadline)
        Found.new(index, index.nil? ? nil : list[index], probability)
      end
    end

    def choose(question, evidence, options: nil, cancel: nil, deadline: nil)
      question = choose_question(question, options)
      crossing(cancel: cancel) { |own| @engine.choose(question, text_of(evidence), own, deadline) }
    end

    def score(question, evidence, levels: nil, cancel: nil, deadline: nil)
      question = score_question(question, levels)
      crossing(cancel: cancel) { |own| @engine.score(question, text_of(evidence), own, deadline).first }
    end

    # The score with its nearest level beside it, for callers that want
    # the level's name without a second call.
    def score_with_level(question, evidence, levels: nil, cancel: nil, deadline: nil)
      question = score_question(question, levels)
      crossing(cancel: cancel) do |own|
        value, nearest = @engine.score(question, text_of(evidence), own, deadline)
        [value, nearest]
      end
    end

    def tag(question, evidence, labels: nil, cancel: nil, deadline: nil)
      question = tag_question(question, labels)
      crossing(cancel: cancel) { |own| @engine.tag(question, text_of(evidence), own, deadline) }
    end

    def annotate(set, records, on: nil, cancel: nil, deadline: nil)
      set = self.set(set) if set.is_a?(String)
      list = records.to_a
      evidence = if on
        # The on column's values are records: nil refuses like any other
        # record, and a non-String crosses as its JSON text.
        list.map { |record| value = record[on]; text_of(value.is_a?(String) ? value : value) }
      else
        list.map { |one| text_of(one) }
      end
      # The input's keys are preserved: a question landing on any
      # record's key refuses before any request is paid. The check unions
      # every record's keys — the fourth review's probe caught a question
      # landing on the second record's column overwriting it silently,
      # because only the first record was checked.
      if on
        existing = list.flat_map { |record| record.keys.to_a }.map(&:to_sym).uniq
        clashes = set.names.map(&:to_sym) & existing
        unless clashes.empty?
          raise UsageError.new(
            "annotate cannot add a question named '#{clashes.first}': the " \
            "input already has a key by that name; rename one", "usage")
        end
      end
      answers = crossing(cancel: cancel, tick: Thread.current[:thinkthen_tick]) do |own|
        @engine.annotate(set, evidence, own, deadline)
      end
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
      crossing(cancel: cancel) { |own| @engine.details(built(question), text_of(evidence), own, deadline) }
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
      answer = crossing(cancel: cancel) { |own| @engine.recognize(JSON.generate(spec), text_of(text), own, deadline) }
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

    # Say how the records relate to each other: one yes/no question per
    # legal pair per relation.
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
      answer = crossing(cancel: cancel) { |own| @engine.relate(JSON.generate(spec), list.map { |one| text_of(one) }, own, deadline) }
      answer.map do |one|
        Edge.new(one["name"], one["source"], one["target"], one["probability"],
                 one["source_kind"], one["target_kind"])
      end
    end

    def usage
      @engine.usage
    end

    # Run bulk calls with a tick: the watchdog runs the block about ten
    # times a second while a call from this thread is in flight, and a
    # raise inside it cancels the call, lets sent requests finish, and
    # re-raises after the call returns. A tick is for progress reporting
    # or a host's own gesture.
    #
    # The tick is this thread's own: it rides Thread.current, so two
    # threads sharing one engine each hear their own block and never each
    # other's. A block alone sets the tick for this thread's later calls;
    # a positional tick with a block scopes it to the block's calls.
    #
    # Honesty about the cadence: the tick runs on the watchdog's clock,
    # roughly every tenth of a second while a call is in flight, not once
    # per request; a call that finishes between ticks runs its tick zero
    # times.
    def with_tick(tick = nil, &block)
      # A block alone is the tick, set for this thread's later calls (the
      # historical shape). A positional tick with a block scopes the tick
      # to the block's calls and clears it on the way out.
      held = tick || block
      Thread.current[:thinkthen_tick] = held
      return @engine unless block && tick
      begin
        yield
      ensure
        Thread.current[:thinkthen_tick] = nil
      end
    end

    private
    # Start (or find) the one watchdog thread. It is created lazily and
    # recreated after a fork, which does not carry Ruby threads into the
    # child.
    def watchdog
      current = @watchdog
      return current if current&.alive?
      @watchdog = Thread.new do
        Thread.current.name = "tt-watchdog" if Thread.current.respond_to?(:name=)
        loop do
          sleep WATCHDOG_INTERVAL
          rows = begin
            @rows_mutex.synchronize { @rows.values }
          rescue StandardError
            next
          end
          rows.each do |row|
            begin
              row.token.cancel if row.caller&.cancelled?
              row.tick&.call
            rescue Exception => e # rubocop:disable Lint/RescueException
              # A tick that raises cancels its own call and rides out.
              row.error ||= e
              row.token.cancel
            end
          end
        end
      end
    end

    # Run one crossing under the watchdog: register the row, hand the
    # call's own token to the block, unregister, and surface a tick's
    # raise after the call is fully finished. `cancel` is the caller's
    # own token, watched here and never fired; `tick` is this thread's
    # progress block, run by the watchdog while the call is in flight.
    def crossing(cancel: nil, tick: nil)
      watchdog
      row = Row.new(Thread.current, nil, tick, cancel, nil)
      token = Cancel.new
      # A token fired before the call stops it before any send; the
      # watchdog's poll would come a round late (surfaces-review-5).
      token.cancel if cancel&.cancelled?
      row.token = token
      @rows_mutex.synchronize { @rows[row.object_id] = row }
      begin
        yield token
      ensure
        @rows_mutex.synchronize { @rows.delete(row.object_id) }
        raise row.error if row.error
      end
    end

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
