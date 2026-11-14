# frozen_string_literal: true
require "thinkthen"
require "set"
require "stringio"
require_relative "cases"

# Feature: scalar failure belongs to the call, independently of the answer read.
def ruby_scalar_failures
  Backend.open("--guide-scenario", "yes") do |backend|
    ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key") do |client|
      assert(!ThinkThen.const_defined?(:RecordError, false), "removed exception has no alias")
      questions = ThinkThen::AnnotationQuestions.new(ready: ThinkThen::DecideQuestion.new("Ready?"))
      calls = {
        decide: ->(record) { client.decide("Ready?", record) },
        choose: ->(record) { client.choose("Which?", record, choices: %i[a b]) },
        tag: ->(record) { client.tag("Tags?", record, labels: %i[a b]) },
        score: ->(record) { client.score("Scale?", record, levels: %i[low high]) },
        find: ->(record) { client.find("Action?", record) },
        recognize: ->(record) { client.recognize(record, kinds: ["person"]) },
        relate: ->(record) { client.relate(record, rules: [ThinkThen::RelationRule.new(source: "person", target: "person", relation: "knows")]) },
        annotate: ->(record) { client.annotate(questions, record) }
      }
      calls.each do |function, call|
        [nil, "", "   ", {}].each do |record|
          failure = raises(ThinkThen::Error) { call.call(record) }
          assert(failure.is_a?(ThinkThen::FailedError) && failure.is_a?(ThinkThen::NoAnswerError), "#{function}: both failure families")
          equal(record.nil? ? :"record.missing" : :"record.empty", failure.code, "#{function}: scalar admission code")
          equal(1, failure.rows.length, "#{function}: own failed parent payload")
          row = failure.rows.first
          assert(row.failed? && row.function == function && row.position == 1 && row.item.equal?(record), "#{function}: own row and original survive")
          equal([row.error.message, row.error.http_status], [failure.message, failure.http_status], "#{function}: failure payload")
          equal(0, client.usage.active_calls, "#{function}: failed scalar closes its call")
        end
      end
      equal("0", backend.command("count"), "blank and missing scalar records send no requests")
      assert(client.decide("Ready?", "still usable").answered?, "client survives scalar failures")
      equal("1", backend.command("count"), "next valid call uses the same client")
      equal(0, client.usage.active_calls, "successful scalar also closes its call")
    end
  end
end

ruby_scalar_failures

ThinkThen::Client.open(backend: :stub) do |client|
  record = {id: "a1", text: "Please refund my order."}
  row = client.choose("Which?", record, choices: %i[billing shipping other])
  raise "wrong typed choice" unless row.is_a?(ThinkThen::ChooseRow) && row.answer.is_a?(Symbol)
  raise "wrong original item" unless row.item.equal?(record)
  raise "wire includes item" if JSON.parse(row.to_json).key?("item")
  raise "choice has yes?" if row.respond_to?(:yes?)
  values = Set[row]
  before = row.hash
  record[:text] = "changed"
  raise "mutable result hash" unless before == row.hash && values.include?(row)
  missing = raises(ThinkThen::FailedError) { client.decide("Ready?", nil) }.rows.first
  raise "missing state unsafe" unless missing.failed? && missing.probability.nil? && missing.item.nil?
  raise "unsafe failed printing" unless missing.to_s.start_with?("failed: record.missing:") && missing.inspect == "#<ThinkThen::DecideRow #{missing}>"
  begin
    missing.answer
    raise "quiet missing answer"
  rescue ThinkThen::NoAnswerError => failure
    raise "wrong failure family" unless failure.is_a?(ThinkThen::Error) && failure.rows.first.equal?(missing)
  end
  question = ThinkThen::AnnotationQuestions.new(refund: ThinkThen::DecideQuestion.new("Ready?"))
  form = client.annotate(question, "record")
  raise "wrong entry class" unless form.answer[:refund].is_a?(ThinkThen::DecideRow)
  raise "wrong form metadata" unless form.answer[:refund].position == form.position && form.answer[:refund].source.equal?(form.source)
  raise "safe form failed" unless form.raise_if_failed.equal?(form)
  originals = [{id: "r1", text: "first"}, nil, {id: "r2", text: "last"}]
  ranks = client.rank("Urgency?", originals, levels: %i[routine urgent]).to_a
  raise "rank loses original" unless ranks.all? { |ranked| ranked.item.equal?(originals[ranked.position - 1]) }
  raise "failed rank lost" unless ranks.last.failed? && ranks.last.item.nil?
end
puts "PASS Ruby result contract smoke"

# The full result contract also covers nested forms and all text conversions.
def ruby_ideal_forms
  %w[yes no unsure failed refund-failed topic-failed urgency-failed refund-unsure topic-unsure].each do |profile|
    Backend.open("--guide-scenario", profile) do |backend|
      ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key",
                            retry: ThinkThen::RetryPolicy.new(max_retries: 0)) do |client|
        questions = ThinkThen::AnnotationQuestions.new(
          refund: ThinkThen::DecideQuestion.new("Refund?", threshold: 0.2..0.8),
          topic: ThinkThen::ChooseQuestion.new("Topic?", choices: %i[billing shipping other]),
          urgency: ThinkThen::ScoreQuestion.new("Urgency?", levels: %i[routine urgent]))
        original = {id: +"form-1", text: +"record"}
        same = {id: "form-1", text: "record"}
        form = client.annotate(questions, original)
        copy = client.annotate(questions, same)
        assert(form.answered?, "named backend failures live in their form entries")
        entries = form.answer
        assert(entries.values.all? { |entry| entry.position == form.position && entry.id == form.id && entry.source.equal?(form.source) && entry.item.equal?(original) }, "every child retains parent metadata")
        failed = entries.values.find(&:failed?)
        if failed
          error = raises(ThinkThen::FailedError) { form.raise_if_failed }
          equal([failed], error.rows, "one affected form retains its first actual failed child")
          equal([failed.error.code, failed.error.message, failed.error.http_status], [error.code, error.message, error.http_status], "native failure survives unchanged")
        else
          assert(form.raise_if_failed.equal?(form), "uncertainty alone does not fail a form")
        end
        assert(form == copy && form.eql?(copy) && form.hash == copy.hash, "nested value equality excludes original items")
        table = {form => "kept"}
        original[:id].replace("changed-id")
        original[:text].replace("changed-record")
        equal("kept", table.fetch(form), "mutating original data preserves map lookup")
        assert(form == copy && form.id == "form-1", "own metadata and equality remain immutable")
        raises(KeyError) { entries[:absent] }
        equal(form.to_h, JSON.parse(form.to_json), "canonical form serialization")
        assert(!form.to_h.key?("item") && !form.to_h.fetch("answer").fetch("refund").key?("item"), "no original appears in saved rows")
      end
    end
  end
  Backend.open("--guide-scenario", "yes") do |backend|
    ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key") do |client|
      questions = ThinkThen::AnnotationQuestions.new(
        refund: ThinkThen::DecideQuestion.new("Refund?", field: "/absent"),
        topic: ThinkThen::ChooseQuestion.new("Topic?", choices: %i[billing shipping], field: "/absent"),
        urgency: ThinkThen::ScoreQuestion.new("Urgency?", levels: %i[routine urgent]))
      form = client.annotate(questions, {text: "record"})
      equal([:failed, :failed, :answered], form.answer.values.map(&:status), "an answered parent retains two failed children")
      raises(ThinkThen::FailedError) { form.raise_if_failed }
      equal(0, client.usage.active_calls, "returned partial form has no open call")
    end
  end
  Backend.open("--guide-scenario", "failed") do |backend|
    ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key",
                          retry: ThinkThen::RetryPolicy.new(max_retries: 0)) do |client|
      # Engine name order is UTF-8 byte order, rather than the Hash's insertion order.
      questions = ThinkThen::AnnotationQuestions.new(**{"é" => ThinkThen::ChooseQuestion.new("Topic?", choices: %i[a b]),
                                                       "z" => ThinkThen::DecideQuestion.new("Ready?")})
      original = {id: "utf8-order", text: "record"}
      form = client.annotate(questions, original)
      error = raises(ThinkThen::FailedError) { form.raise_if_failed }
      assert(error.rows.first.equal?(form.answer["z"]) && error.rows.first.item.equal?(original), "native first failed child with original metadata")
      failure = raises(ThinkThen::FailedError) { client.annotate(questions, nil) }
      outer = failure.rows.first
      assert(outer.failed?, "missing record fails the outer form")
      assert(raises(ThinkThen::FailedError) { outer.raise_if_failed }.rows.first.equal?(outer), "outer failure wins without a synthetic parent")
    end
  end
end


def ruby_ideal_formats
  failure = 'failed: backend.status: the backend answered status 503: {"error":"unavailable"}'
  %w[relations unsure failed].each do |profile|
    Backend.open("--guide-scenario", profile) do |backend|
      ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key",
                            retry: ThinkThen::RetryPolicy.new(max_retries: 0)) do |client|
        message = "Morgan works for Example\tGroup.\nPlease refund my order."
        questions = ThinkThen::AnnotationQuestions.new(
          refund: ThinkThen::DecideQuestion.new("Refund?", threshold: 0.2..0.8),
          topic: ThinkThen::ChooseQuestion.new("Topic?", choices: %i[billing shipping other]),
          urgency: ThinkThen::ScoreQuestion.new("Urgency?", levels: %i[routine urgent]))
        values = [client.decide_each("Ready?", ["record"], threshold: 0.2..0.8).first,
                  client.choose_each("Topic?", ["record"], choices: %i[billing shipping other]).first,
                  client.tag_each("Topics?", ["record"], labels: %i[billing shipping]).first,
                  client.score_each("Urgency?", ["record"], levels: %i[routine urgent]).first,
                  client.rank("Urgency?", ["record"], levels: %i[routine urgent]).first,
                  client.find_each("Action?", [message]).first, client.recognize_each([message], kinds: %w[person organization]).first,
                  client.relate_each([message], rules: [ThinkThen::RelationRule.new(source: "person", target: "organization", relation: "works_for")]).first,
                  client.annotate(questions, "record")]
        expected = if profile == "failed"
                     [*Array.new(8, failure), "refund=#{failure}\ttopic=#{failure}\turgency=#{failure}"]
                   elsif profile == "unsure"
                     ["unsure", "unsure", "billing\tshipping", "0.5", "1", "unsure", "", "", "refund=unsure\ttopic=unsure\turgency=0.5"]
                   else
                     ["true", "billing", "billing\tshipping", "1", "1", "Please refund my order.",
                      "person: Morgan (0..6)\torganization: Example Group (17..30)", "Morgan --works_for--> Example Group", "refund=true\ttopic=billing\turgency=1"]
                   end
        values.zip(expected).each do |row, word|
          equal([word, "#<#{row.class.name} #{word}>", word, word, "#<#{row.class.name} #{word}>"], [row.to_s, row.inspect, "#{row}", format("%s", row), format("%p", row)], "#{profile} #{row.function} safe text routes")
          printed = StringIO.new
          printed.puts(row)
          equal(word+"\n", printed.string, "puts uses safe result text")
          equal(row.to_h, JSON.parse(JSON.generate(row)), "saving never reads the answer")
        end
        if profile == "relations"
          assert(values[6].answer.last.text.include?("\t"), "bare name retains the original tab")
          selected = values[5].answer
          equal(selected.text, message[selected.start...selected.end], "Ruby character slicing matches find")
        end
      end
    end
  end
  Backend.open("--guide-scenario", "yes") do |backend|
    ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key") do |client|
      raw = "a\tb\r\nc"
      row = client.choose("Which?", "record", choices: [raw, "other"])
      equal("a b  c", row.to_s, "string-leaf CR LF tab folding")
      equal(raw, row.answer, "bare answer stays unchanged")
      equal(raw, JSON.parse(row.to_json).fetch("answer"), "saved JSON stays unchanged")
    end
  end
  Dir.mktmpdir("ruby-small-score-") do |folder|
    path = File.join(folder, "case.json")
    File.write(path, JSON.generate("replies" => [{"distribution" => {"routine" => 0.9999999, "urgent" => 0.0000001}}]))
    Backend.open(path) do |backend|
      ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key") do |client|
        row = client.score("Urgency?", "record", levels: %i[routine urgent])
        equal(0.0000001, row.answer, "small score stays a Float")
        equal(0.0000001, JSON.parse(row.to_json).fetch("answer"), "canonical small score stays numeric")
        equal("0.0000001", row.to_s, "small score prints a plain decimal")
        equal("#<ThinkThen::ScoreRow 0.0000001>", row.inspect, "small score inspect wraps the same command word")
      end
    end
  end
end


def ruby_ideal_refusals
  Backend.open("--guide-scenario", "yes") do |backend|
    ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key") do |client|
      [proc { ThinkThen::FilterQuestion.new("Ready?", threshold: 0.2..0.8) },
       proc { client.filter("Ready?", ["record"], threshold: 0.2..0.8) }].each do |operation|
        error = raises(ThinkThen::UsageError, &operation)
        assert(error.code == :"usage.threshold" && error.message.include?("decide_each"), "native filter band refusal names the host route")
      end
      raises(ThinkThen::UsageError) { client.choose("Which?", "record", choices: %i[billing billing]) }
      raises(ThinkThen::UsageError) { client.choose("Which?", "record", choices: [:billing, "shipping"]) }
      raises(ThinkThen::UsageError) { ThinkThen::AnnotationQuestions.new(filter: ThinkThen::FilterQuestion.new("Ready?")) }
      box = ThinkThen::Toolbox.new(questions: {topic: ThinkThen::ChooseQuestion.new("Topic?", choices: %i[billing shipping])})
      raises(ThinkThen::UsageError) { box.choose(:absent) }
      raises(ThinkThen::UsageError) { box.decide(:topic) }
      equal("0", backend.command("count"), "refused calls send nothing")
      record_class = Struct.new(:id, :text)
      original = record_class.new("own-type", "record")
      row = client.decide("Ready?", original)
      assert(row.answered? && row.id == "own-type" && row.item.equal?(original), "caller classes use to_h and retain identity")
    end
  end
end


def ruby_ideal_filter_retention
  Backend.open("--guide-scenario", "no") do |backend|
    ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key", batch: 1) do |client|
      live, peak = ObjectSpace::WeakMap.new, 0
      source = Enumerator.new do |out|
        128.times do |index|
          if index.positive? && (index % 32).zero?
            GC.start(full_mark: true, immediate_sweep: true)
            peak = [peak, live.keys.length].max
          end
          record = "original #{index}"
          live[record] = true
          out << record
        end
      end
      equal([], client.filter("Ready?", source).to_a, "all-no filter returns no records")
      assert(peak < 64, "filter retained declined originals instead of releasing by position: #{peak}")
    end
  end
end
