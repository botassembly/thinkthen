# frozen_string_literal: true

require_relative "cases"

def ruby_interface
  ThinkThen::Client.open(backend: :stub) do |client|
    is_refund = client.decide("Refund?", "Please refund my order.")
    copy = is_refund.with
    assert(is_refund == copy && is_refund.eql?(copy), "Data equality remains complete value equality")
    equal(is_refund.hash, copy.hash, "equal rows retain equal hashes")
    assert(is_refund != is_refund.answer, "a Data row does not compare equal to its bare answer")
    equal(is_refund.answer, is_refund.yes?, "boolean predicate")
    equal(!is_refund.answer, is_refund.no?, "negative predicate")
    equal(is_refund.answer, client.decide?("Refund?", "Please refund my order."), "client predicate")
    equal(is_refund.to_h, JSON.parse(JSON.generate(is_refund)), "row JSON")
    equal(is_refund.answer.to_s, is_refund.to_s, "row text")
    maybe = client.decide("Refund?", "Please refund my order.", threshold: 0.01..0.99)
    assert(maybe.unsure?)
    raises(ThinkThen::UnsureError) { maybe.answer }
    raises(ThinkThen::UnsureError) { maybe.yes? }
    broken = client.decide("Refund?", nil)
    error = raises(ThinkThen::RecordError) { broken.answer }
    equal(broken.error.code, error.code, "failed answer code")
    equal(broken.error.code, raises(ThinkThen::Error) { raise broken.error }.code, "raise row error")
    symbol_record = client.decide("Refund?", {text: "Please refund my order.", id: 7})
    string_record = client.decide("Refund?", {"text" => "Please refund my order.", "id" => 7})
    equal(string_record.to_h, symbol_record.to_h, "symbol record keys")
    raises(ThinkThen::UsageError) { client.decide("Refund?", ["one", "two"]) }
    raises(ThinkThen::UsageError) { client.decide_each("Refund?", "one") }
    described = ThinkThen::DecideQuestion.new("Refund?", yes_means: "Return the money", no_means: "Keep the charge")
    serialized = JSON.parse(ThinkThen::Toolbox.new(questions: {refund: described}).to_json)
    assert(serialized.to_json.include?("Return the money"), "yes meaning reaches the saved question")
    choice = client.choose("Owner?", "Please refund my order.", choices: %w[billing shipping], descriptions: {billing: "Refunds"})
    assert(choice.answer.is_a?(String), "described choices retain string answers")
    {choose: :choices, tag: :labels, score: :levels, rank: :levels}.each do |function, field|
      native = raises(ThinkThen::UsageError) { ThinkThen::Question.new(function, "Owner?", **{field => %w[billing shipping], descriptions: {unknown: "Unmatched"}}) }
      host = raises(ThinkThen::UsageError) { client.public_send(function, "Owner?", function == :rank ? ["one"] : "one", **{field => %w[billing shipping], descriptions: {unknown: "Unmatched"}}) }
      equal([native.code, native.message], [host.code, host.message], "#{function} description refusal belongs to shared admission")
    end
    descriptions = {billing: "", shipping: "Parcels"}
    described_choice = ThinkThen::ChooseQuestion.new("Owner?", choices: %w[billing shipping], descriptions: descriptions)
    equal({billing: "", shipping: "Parcels"}, descriptions, "admission keeps the caller's descriptions")
    saved_choices = JSON.parse(ThinkThen::Toolbox.new(questions: {owner: described_choice}).to_json).fetch("entries").fetch("owner").fetch("question").fetch("choices")
    equal("", saved_choices.first.fetch("description"), "empty descriptions remain explicit")
    ThinkThen::Client.open(backend: :stub, retry: ThinkThen::Retry.new(initial_backoff: 0.25, attempt_timeout: 2.5)) { |configured| assert(!configured.closed?) }
    urgency = client.score("Urgency?", "Please refund my order.", levels: %w[low high])
    equal(urgency.answer, urgency.to_f, "score conversion")
    assert(urgency >= 0.0 && urgency <= 1.0, "score comparison")
    fitting = client.tag("Issue?", "Please refund my order.", labels: %w[billing shipping])
    equal(fitting.answer, fitting.to_a, "tag conversion")
    excerpt = client.find("Refund words?", "Please refund my order.")
    equal(excerpt.answer&.text.to_s, excerpt.to_s, "find prints matched text")
    equal(is_refund.source.to_h, JSON.parse(JSON.generate(is_refund.source)), "value JSON")
    sized = client.decide_each("Refund?", %w[one two])
    equal(2, sized.size, "Enumerable size")
    sized.close
    Dir.mktmpdir("ruby-saved-form-") do |directory|
      path = File.join(directory, "questions.json")
      saved = ThinkThen::Toolbox.new(questions: {refund: described}).to_json
      File.write(path, saved)
      equal(saved, ThinkThen::Toolbox.load_file(path).to_json, "saved toolbox file")
      loaded = ThinkThen::AnnotationQuestions.load_file(path)
      assert(client.annotate(loaded, "Please refund my order.")[:refund].answered?, "saved annotation file")
    end
    records = ["one", nil, "two", "three", nil]
    expected = client.filter_each("Refund?", records).select(&:kept?).map(&:item)
    kept = []
    error = raises(ThinkThen::RecordError) { client.filter("Refund?", records).each { |entry| kept << entry } }
    equal(expected, kept, "kept records are delivered before failures")
    equal([2, 5], error.rows.map(&:position), "all failed positions")
    mixed = client.filter("Refund?", records, failures: :row).to_a
    equal(expected, mixed.take(expected.length), "failure row mode keeps answers")
    equal([2, 5], mixed.drop(expected.length).map(&:position), "failure row mode lists failures")
    ranked = client.rank_each("Urgency?", %w[one two three], levels: %w[low high]).to_a
    by_rank = client.rank("Urgency?", %w[one two three], levels: %w[low high]).to_a
    equal(ranked.map(&:item), by_rank.map(&:item), "rank rows carry copies of the records in rank order")
    equal([1, 2, 3], by_rank.map(&:rank), "rank rows expose the engine's rank number")
    tickets = [{text: "one", id: 1}, {text: "two", id: 2}]
    copies = client.rank("Urgency?", tickets, levels: %w[low high]).map(&:item)
    equal([{"text" => "one", "id" => 1}, {"text" => "two", "id" => 2}], copies.sort_by { |ticket| ticket["id"] }, "rank copies have String keys")
    assert(copies.none? { |copy| tickets.any? { |ticket| ticket.equal?(copy) } }, "rank returns copies, not the caller's objects")
    lazy_odd = ["one", Object.new, "two"].each.lazy
    seen = []
    odd = raises(ThinkThen::RecordError) { client.rank("Urgency?", lazy_odd, levels: %w[low high]) { |row| seen << row.item } }
    equal(%w[one two].sort, seen.sort, "rank continues past a record with no JSON form")
    failed = odd.rows.first
    equal(:"record.invalid", failed.row.error.code, "unconvertible record fails as record.invalid")
    assert(failed.row.error.message.include?("Object"), "the failure names the record's class: #{failed.row.error.message}")
    raises(ThinkThen::RecordError) { failed.item }
    assert(!failed.to_h.key?("item"), "a record with no JSON form has no saved item")
    rows_mode = client.rank("Urgency?", ["one", Object.new], levels: %w[low high], failures: :row).to_a
    equal([:answered, :failed], rows_mode.map(&:status), "failure row mode yields failed rank rows last")
    collision = {text: "refund", "text" => "different evidence"}
    converted = client.decide_each("Refund?", [collision, {"body" => collision}, "one"]).to_a
    equal([:failed, :failed, :answered], converted.map(&:status), "colliding String and Symbol keys fail without losing evidence")
    equal([:"record.invalid", :"record.invalid"], converted.take(2).map { |row| row.error.code }, "lossy Hash conversion fails as a record")
    original = {"id" => 2**64 - 1, "text" => "é😀 refund", "nested" => {"source" => {"question_identity" => 7}, "error" => {"code" => "caller.value"}}}
    saved_rank = client.rank("Urgency?", [original], levels: %w[low high]).first
    equal(original, saved_rank.item, "rank copies keep Unicode, large integers and arbitrary record keys")
    kept_in_block = []
    equal(nil, client.filter("Refund?", %w[one two]) { |record| kept_in_block << record }, "filter block form returns nil")
    equal(client.filter("Refund?", %w[one two]).to_a, kept_in_block, "filter block yields each kept record")
    meant = client.filter_each("Refund?", %w[one], yes_means: "Return the money", no_means: "Keep the charge").to_a
    equal(1, meant.length, "filter takes answer meanings")
    filtered = ThinkThen::FilterQuestion.new("Refund?", yes_means: "Return the money")
    assert(ThinkThen::Toolbox.new(questions: {refund: filtered}).to_json.include?("Return the money"), "filter meaning reaches the saved question")
    form = ThinkThen::AnnotationQuestions.new(questions: {steps: ThinkThen::DecideQuestion.new("Refund?")})
    triage = client.annotate(form, "Please refund my order.")
    equal(triage.answer.to_h, client.annotate(form.questions, "Please refund my order.").answer.to_h, "plain annotation question Hash")
    equal(triage.answer.fetch("steps"), triage[:steps], "symbol annotation lookup")
    raises(KeyError) { triage[:missing] }
    raises(KeyError) { triage.answer[:missing] }
  end
  client = ThinkThen::Client.open(backend: :stub)
  closed_rows = client.decide_each("Refund?", ["Please refund my order."])
  closed_rows.close
  equal(:"usage.closed", raises(ThinkThen::UsageError) { closed_rows.next }.code, "closed call code")
  client.close
  equal(:"usage.closed", raises(ThinkThen::UsageError) { client.decide("Refund?", "Please refund my order.") }.code, "closed client code")
  ruby_backend_environment
  ThinkThen.configure { |config| config.backend = :stub }
  ThinkThen.decide("Refund?", "Please refund my order.")
  ThinkThen.close
  ThinkThen.configure { |config| config.backend = :stub }
  ThinkThen.close
  retry_policy = ThinkThen::Retry.new(initial_backoff: 0.25, attempt_timeout: 2.5)
  equal(250, retry_policy.to_h.fetch("initial_backoff_ms"), "retry seconds")
  equal(2500, retry_policy.to_h.fetch("attempt_timeout_ms"), "timeout seconds")
  %i[RowWire RowPredicates AnnotationWire WireValue NativeContract].each do |name|
    assert(!ThinkThen.constants.include?(name), "internal constant #{name}")
  end
end

# Feature: a client with no backend reads THINKTHEN_BACKEND; an explicit backend wins.
def ruby_backend_environment
  ENV["THINKTHEN_BACKEND"] = "stub"
  ThinkThen::Client.open { |client| equal(:stub, client.decide("Refund?", "Please refund my order.").source.backend, "environment backend") }
  ENV["THINKTHEN_BACKEND"] = "no-such-backend"
  equal(:"usage.backend", raises(ThinkThen::UsageError) { ThinkThen::Client.open {} }.code, "unknown environment backend")
  ThinkThen::Client.open(backend: :stub) { |client| assert(client.decide("Refund?", "Please refund my order.").answered?, "explicit backend wins") }
  ENV.delete("THINKTHEN_BACKEND")
  equal(:"usage.key-missing", raises(ThinkThen::UsageError) { ThinkThen::Client.open {} }.code, "unset means the default backend")
ensure
  ENV.delete("THINKTHEN_BACKEND")
end

# Feature: a top-1 rank over a long lazy source keeps no host copy of every record.
def ruby_bounded_rank
  count, made, peak = 3000, ObjectSpace::WeakMap.new, 0
  text = ->(index) { "record #{index} " + "x" * 256 }
  source = Enumerator.new do |output|
    count.times do |index|
      if (index % 500).zero? && index.positive?
        GC.start(full_mark: true, immediate_sweep: true)
        peak = [peak, made.keys.length].max
      end
      record = text.(index)
      made[record] = true
      output << record
    end
  end.lazy
  ThinkThen::Client.open(backend: :stub) do |client|
    rows = client.rank("Urgency?", source, levels: %w[low high], top: 1).to_a
    assert(peak < 100, "rank kept #{peak} of #{count} input records alive")
    equal(1, rows.length, "top-1 rank yields one row")
    top = rows.first
    equal(1, top.rank, "engine rank number")
    equal(text.(top.position - 1), top.item, "rank row carries an equal copy of its record")
  end
end

if $PROGRAM_NAME == __FILE__
  isolated_ruby_environment do
    require "thinkthen"
    ruby_bounded_rank
    ruby_interface
  end
  puts "Ruby interface public stub cases passed"
end
