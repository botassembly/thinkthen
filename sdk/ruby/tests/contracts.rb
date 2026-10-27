# frozen_string_literal: true

# Feature: execution names remain independent from toolbox keys.
def ruby_execution_names
  question = ThinkThen::DecideQuestion.new("Ready?")
  toolbox = ThinkThen::Toolbox.new(questions: {"selection" => question})
  ThinkThen::Client.open(backend: :stub) do |client|
    unnamed = client.decide(toolbox["selection"], "record")
    named = client.decide(toolbox["selection"], "record", name: "review")
    assert(unnamed.source.name.nil?, "toolbox lookup does not set a call name")
    equal("review", named.source.name, "execution name joins the source")
    equal(client.decide(question, "record", name: "review").source.to_h, named.source.to_h,
          "toolbox selection preserves independently supplied execution identity")
    raises(ThinkThen::UsageError) { client.decide(question, "record", name: "") }
    rank = ThinkThen::RankQuestion.new("Urgency?", levels: %w[low high])
    retained = ThinkThen::Toolbox.new(questions: {"ranking" => rank})
    assert(retained["ranking"].equal?(rank), "general toolbox retains rank")
    error = raises(ThinkThen::UsageError) { ThinkThen::AnnotationQuestions.new(questions: {"bad" => rank}) }
    equal(:"usage.question", error.code, "engine refuses rank in annotation")
  end
end

# Feature: JSON conversion invokes no caller serialization hooks and failed records retain positions.
def ruby_conversion_contract
  hook = Class.new do
    def to_json(*) = raise("caller serialization hook must not run")
  end
  invalid = hook.new
  ThinkThen::Client.open(backend: :stub) do |client|
    records = [invalid, "record", nil]
    finite = client.decide_each("Ready?", records)
    records.clear
    rows = finite.to_a
    equal([1, 2, 3], rows.map(&:position), "finite input snapshots original membership")
    equal(:"record.invalid", rows.first.error.code, "unsupported host value is a failed record")
    assert(rows[1].answered?, "conversion failure does not poison the next record")
    equal(:"record.missing", rows.last.error.code, "missing record is distinct")
    source = Enumerator.new { |out| out << "record"; raise TypeError, "caller enumeration error" }
    error = raises(TypeError) { client.decide_each("Ready?", source).to_a }
    equal("caller enumeration error", error.message, "caller iterator failure keeps its identity")
    [Float::NAN, Float::INFINITY, -Float::INFINITY].each do |threshold|
      error = raises(ThinkThen::UsageError) { client.decide("Ready?", "record", threshold: threshold) }
      equal(:"usage.threshold", error.code, "nonfinite numeric threshold uses engine admission")
    end
    raises(TypeError) { client.rank("Urgency?", "record", levels: %w[low high], top: 1.5) }
    token = ThinkThen::CancelToken.new
    token.cancel
    raises(ThinkThen::Cancelled) { client.decide_each("Ready?", ["record"], cancel: token).to_a }
  end
end

# Feature: redacted keys can be reused after the original settings change.
def ruby_key_snapshots
  config = ThinkThen::Configuration.new
  secret = +"synthetic-reusable-key"
  config.api_key = secret
  key = config.api_key
  secret.replace("changed-by-caller")
  config.api_key = "synthetic-other-key"
  assert(key.is_a?(ThinkThen::RedactedKey), "getter returns opaque native key")
  Backend.open("--unavailable") do |backend|
    ThinkThen::Client.open(backend: :systemone, url: backend.url, api_key: key) do |client|
      assert(!client.closed?, "opaque key satisfies independent backend credential admission")
      assert(!client.api_key.to_s.include?("synthetic-reusable-key"), "client key remains redacted")
    end
    equal("0", backend.command("count"), "credential admission does not call a backend")
  end
end

# Feature: the installed gem carries its typed API and package metadata.
def ruby_packaging
  spec = Gem::Specification.find_by_name("thinkthen")
  equal(ThinkThen::VERSION, spec.version.to_s, "gem version comes from the library version")
  equal("true", spec.metadata.fetch("rubygems_mfa_required"), "publication metadata requires MFA")
  %w[source_code_uri changelog_uri bug_tracker_uri documentation_uri].each do |field|
    assert(spec.metadata.fetch(field).start_with?("https://"), "package metadata includes #{field}")
  end
  equal(spec.homepage + "/blob/v#{ThinkThen::VERSION}/sdk/ruby/API.md", spec.metadata.fetch("documentation_uri"), "documentation matches the installed tag")
  signatures = Dir[File.join(spec.full_gem_path, "sig", "*.rbs")]
  assert(signatures.any? { |path| File.basename(path) == "error_codes.rbs" }, "generated code types ship in the gem")
  assert(signatures.any? { |path| File.basename(path) == "thinkthen.rbs" }, "public RBS ships in the gem")
end

# Feature: every saved question has matching single-record and Enumerable calls.
def ruby_saved_calls
  cases = JSON.parse(File.read(ENV.fetch("THINKTHEN_STUB_CASES"), encoding: Encoding::UTF_8), max_nesting: false)
  cases.uniq { |test| test.fetch("request").fetch("question").fetch("function") }.each do |test|
    definition = test.fetch("request").fetch("question")
    function = definition.fetch("function")
    record = test.fetch("request").fetch("records").first
    question = if function == "annotate"
                 children = definition.fetch("questions").transform_values { |child| typed_question(child) }
                 ThinkThen::AnnotateQuestion.new(ThinkThen::AnnotationQuestions.new(questions: children))
               else
                 typed_question(definition)
               end
    ThinkThen::Client.open(backend: :stub) do |client|
      single = client.public_send(function, question, record)
      stream = client.public_send("#{function}_each", question, [record]).to_a
      equal(1, stream.length, "saved #{function} returns one row")
      if function == "filter"
        assert(single.item.equal?(record), "saved filter retains original item")
        single, expected = single.row, stream.first.row
      else
        expected = stream.first
      end
      equal(expected.to_h, single.to_h, "saved #{function} maps the same question and record")
    end
  end
end
