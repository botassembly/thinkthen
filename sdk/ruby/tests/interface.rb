# frozen_string_literal: true

require_relative "cases"

def ruby_interface
  ThinkThen::Client.open(backend: :stub) do |client|
    is_refund = client.decide("Refund?", "Please refund my order.")
    copy = client.decide("Refund?", "Please refund my order.".dup)
    assert(is_refund == copy && is_refund.eql?(copy), "result value equality excludes original identity")
    equal(is_refund.hash, copy.hash, "equal results retain equal hashes")
    error = raises(TypeError) { is_refund == is_refund.answer }
    assert(error.message.include?(".answer"), "bare equality names the answer read")
    equal(is_refund.answer, is_refund.yes?, "decide predicate")
    %i[no? found? to_f to_i to_sym to_a empty? include? each <=> []].each do |name|
      assert(!is_refund.respond_to?(name), "removed result shortcut #{name}")
    end
    assert(!client.respond_to?(:decide?) && !ThinkThen.respond_to?(:decide?), "one decide call")
    equal(is_refund.to_h, JSON.parse(JSON.generate(is_refund)), "canonical row JSON")
    equal(is_refund.answer.to_s, is_refund.to_s, "safe result text")
    maybe = client.decide("Refund?", "Please refund my order.", threshold: 0.01..0.99)
    assert(maybe.unsure?)
    uncertain = raises(ThinkThen::NoAnswerError) { maybe.yes? }
    assert(uncertain.row.equal?(maybe) && !uncertain.is_a?(ThinkThen::Error), "uncertainty has one result and is not failure")
    failed = raises(ThinkThen::NoAnswerError) { client.decide("Refund?", nil) }
    broken = failed.rows.first
    assert(failed.is_a?(ThinkThen::Error) && failed.rows.first.equal?(broken), "record failure belongs to both catches")
    equal(client.decide("Refund?", {"text" => "refund", "id" => 7}).to_h,
          client.decide("Refund?", {text: "refund", id: 7}).to_h, "symbol record keys")
    %i[decide choose tag score find annotate].each do |name|
      next if name == :annotate
      options = {choose: {choices: %i[a b]}, tag: {labels: %i[a b]}, score: {levels: %i[a b]}}.fetch(name,{})
      error = raises(ThinkThen::UsageError) { client.public_send(name,"Ready?",["one"],**options) }
      assert(error.message.include?("#{name}_each"), "collection refusal names the exact method")
    end
    raises(ThinkThen::UsageError) { client.decide_each("Refund?", "one") }
    described = ThinkThen::DecideQuestion.new("Refund?", yes_means: "Return the money", no_means: "Keep the charge")
    serialized = ThinkThen::Toolbox.new(questions: {refund: described}).to_json
    assert(serialized.include?("Return the money"), "answer meaning reaches saved question")
    descriptions = {billing: "", shipping: "Parcels"}
    choice = ThinkThen::ChooseQuestion.new("Owner?", choices: %i[billing shipping], descriptions: descriptions)
    equal({billing: "", shipping: "Parcels"}, descriptions, "descriptions stay caller-owned")
    saved = JSON.parse(ThinkThen::Toolbox.new(questions: {owner: choice}).to_json)
    equal("", saved.fetch("entries").fetch("owner").fetch("question").fetch("choices").first.fetch("description"), "empty descriptions remain explicit")
    ThinkThen::Client.open(backend: :stub, retry: ThinkThen::RetryPolicy.new(initial_backoff: 0.25, attempt_timeout: 2.5)) { |configured| assert(!configured.closed?) }
    %i[choose tag score rank].each do |function|
      field = {choose: :choices, tag: :labels, score: :levels, rank: :levels}.fetch(function)
      native = raises(ThinkThen::UsageError) { ThinkThen::Question.new(function,"Owner?",**{field => %w[a b], descriptions: {unknown: "Unmatched"}}) }
      host = raises(ThinkThen::UsageError) { client.public_send(function,"Owner?",function == :rank ? ["one"] : "one",**{field => %w[a b], descriptions: {unknown: "Unmatched"}}) }
      equal([native.code,native.message],[host.code,host.message],"#{function} uses shared admission")
    end
    Dir.mktmpdir("ruby-saved-form-") do |directory|
      path = File.join(directory,"questions.json")
      File.write(path,serialized)
      equal(serialized,ThinkThen::Toolbox.load_file(path).to_json,"saved toolbox file")
      loaded = ThinkThen::AnnotationQuestions.load_file(path)
      assert(client.annotate(loaded,"record").answer[:refund].answered?, "saved annotation file")
    end
    records = ["one",nil,"two","three",nil]
    kept = []
    failure = raises(ThinkThen::FailedError) { client.filter("Refund?",records) { |record| kept << record } }
    equal([2,5],failure.rows.map(&:position),"failed filter positions")
    assert(failure.rows.all? { |row| row.failed? && row.item.equal?(records[row.position-1]) }, "failed filter originals")
    again = []
    raises(ThinkThen::FailedError) { client.filter("Refund?",records) { |record| again << record } }
    equal(kept,again,"filter block keeps results before one failure")
    tickets = [{text:"one",id:1},{text:"two",id:2}]
    ranked = client.rank("Urgency?",tickets,levels:%i[low high]).to_a
    equal([1,2],ranked.map(&:answer),"rank answer is its rank")
    assert(ranked.all? { |row| row.item.equal?(tickets[row.position-1]) }, "rank keeps caller objects")
    invalid = Object.new
    ranked = client.rank("Urgency?",["one",invalid,"two"].each.lazy,levels:%i[low high]).to_a
    equal([:answered,:answered,:failed],ranked.map(&:status),"failed ranks are returned last")
    assert(ranked.last.item.equal?(invalid),"unsupported original is readable")
    equal(:"record.invalid",ranked.last.error.code,"unconvertible record failure")
    raises(ThinkThen::FailedError) { ranked.last.answer }
    collision = {text:"refund","text"=>"other"}
    converted = client.decide_each("Refund?",[collision,{"body"=>collision},"one"]).to_a
    equal([:failed,:failed,:answered],converted.map(&:status),"lossy key conversion is refused")
    form = ThinkThen::AnnotationQuestions.new(steps: described)
    triage = client.annotate(form,"record")
    equal(triage.answer[:steps],client.annotate(form.questions,"record").answer[:steps],"named form answers")
    raises(KeyError) { triage.answer[:missing] }
    assert(!triage.respond_to?(:[]),"form has one loud entry read")
  end
  client = ThinkThen::Client.open(backend: :stub)
  rows = client.decide_each("Ready?",["record"])
  rows.close
  equal(:"usage.closed",raises(ThinkThen::UsageError) { rows.next }.code,"closed call")
  client.close
  equal(:"usage.closed",raises(ThinkThen::UsageError) { client.decide("Ready?","record") }.code,"closed client")
  ruby_backend_environment
  ThinkThen.configure { |config| config.backend = :stub }
  ThinkThen.decide("Ready?","record")
  ThinkThen.close
  ThinkThen.configure { |config| config.backend = :stub }
  ThinkThen.close
  retry_policy = ThinkThen::RetryPolicy.new(initial_backoff:0.25,attempt_timeout:2.5)
  equal(250,retry_policy.to_h.fetch("initial_backoff_ms"),"retry seconds")
  equal(2500,retry_policy.to_h.fetch("attempt_timeout_ms"),"timeout seconds")
  %i[WireValue NativeContract Row FilterRow Answered Unsure Failed].each do |name|
    assert(!ThinkThen.constants.include?(name),"removed/private constant #{name}")
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
  equal(:"usage.key-missing", raises(ThinkThen::SetupError) { ThinkThen::Client.open {} }.code, "unset means the default backend")
ensure
  ENV.delete("THINKTHEN_BACKEND")
end

# Feature: top rank retains the original object of a generated input, as required.
def ruby_rank_originals
  records = []
  source = Enumerator.new do |output|
    3000.times do |index|
      record = "record #{index} " + "x" * 256
      records << record
      output << record
    end
  end
  ThinkThen::Client.open(backend: :stub) do |client|
    rows = client.rank("Urgency?",source,levels:%i[low high],top:1).to_a
    equal(1,rows.length,"top-1 result")
    equal(1,rows.first.answer,"rank answer")
    assert(rows.first.item.equal?(records[rows.first.position-1]),"generated record identity survives ranking")
  end
end

if $PROGRAM_NAME == __FILE__
  isolated_ruby_environment do
    require "thinkthen"
    ruby_rank_originals
    ruby_interface
  end
  puts "Ruby interface public stub cases passed"
end

# Feature: usage is an immutable independent value and remains readable after close.
def ruby_usage
  raises(ThinkThen::UsageError) { ThinkThen::Client.new(backend: :stub, max_requests: -1) }
  client = ThinkThen::Client.new(backend: :stub, max_requests: 0)
  before = client.usage
  equal(0, before.attempts, "stub has no remote attempts")
  equal(0, before.active_calls, "new client has no active calls")
  equal(0, before.input_tokens.reported, "stub input usage is known zero")
  assert(before.frozen? && before.input_tokens.frozen?, "snapshot values are immutable")
  client.decide("Ready?", "record")
  client.close
  Backend.open("--stress", "healthy") do |backend|
    remote = ThinkThen::Client.new(backend: :systemone, url: backend.url, api_key: "synthetic-loopback-key")
    remote.decide("Ready?", "record")
    used = remote.usage
    equal(1, used.attempts, "snapshot translates a remote attempt")
    equal(1, used.responses, "snapshot translates a received response")
    assert(used.request_body_bytes > 0, "snapshot retains encoded byte count")
    remote.close
    equal(used.attempts, remote.usage.attempts, "closed copy preserves paid attempts")
  ensure
    remote&.close
  end
  equal(0, client.usage.active_calls, "owned calls settle before close snapshot")
  equal(0, client.usage.attempts, "closed client retains usage without native access")
  equal(0, before.responses, "old copy remains independent")
  client.close
ensure
  client&.close
end
