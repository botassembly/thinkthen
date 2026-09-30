# frozen_string_literal: true

# One error base, the deadline's bounds (R2-10, R1-11), each fault kind at
# its real boundary, and secrecy: no raised message or inspect line holds
# the key or the address's credentials, and no result value prints the
# caller's text.
require "minitest/autorun"
require_relative "backend"

class TestErrors < Minitest::Test
  def test_the_six_classes_share_one_base_and_carry_their_kind
    lines, = TestBackend.run(<<~RUBY)
      classes = %w[UsageError BackendError LocalError CancelledError DeadlineError DefectError]
      say classes.map { |name| T.const_get(name).ancestors.take(3).map(&:name) }
      error = T::UsageError.new("a refusal", "usage", false)
      say [error.message, error.kind, error.retryable]
    RUBY
    assert_equal %w[UsageError BackendError LocalError CancelledError DeadlineError DefectError].map { |name|
      ["ThinkThen::#{name}", "ThinkThen::Error", "StandardError"]
    }, lines[0]
    assert_equal ["a refusal", "usage", false], lines[1]
  end

  # deadline_ms counts whole milliseconds: nil and -1 mean none, 0 is spent,
  # and every other value refuses before any request, a number past 64 bits
  # included. A value the shim converted itself would panic into DefectError,
  # and the class check would turn red.
  def test_the_deadline_bounds_refuse_before_any_request
    lines, count = TestBackend.run(<<~RUBY)
      [nil, -1, 0, -2, 4_294_967_296_000, 2**64, 1.5, Float::INFINITY, Float::NAN, true, "soon"].each_with_index do |deadline, place|
        say [deadline.inspect, kind_of_raise { T.decide("Is it urgent?", "text \#{place}", deadline_ms: deadline) }]
      end
    RUBY
    assert_equal [%w[nil none], %w[-1 none], %w[0 DeadlineError], %w[-2 UsageError],
                  %w[4294967296000 UsageError], %w[18446744073709551616 UsageError], %w[1.5 UsageError],
                  %w[Infinity UsageError], %w[NaN UsageError], %w[true UsageError], %w["soon" UsageError]], lines
    assert_equal 2, count
  end

  # Ticket 0291: each kind carries its C code, a zero process cap refuses
  # the first live send as usage, and a spent deadline_ms stops recognize and
  # relate as deadline. The backend reads none of them.
  def test_codes_the_zero_cap_and_spent_budgets_send_nothing
    lines, count = TestBackend.run(<<~RUBY)
      say T::Error::CODES.values_at(*%w[usage backend deadline local cancelled defect])
      capped = T::Engine.new(max_requests_total: 0, cache: false)
      begin
        capped.decide("Is it urgent?", "capped")
      rescue T::UsageError => e
        say [e.kind, e.code, e.message.include?("process send budget")]
      end
      pairs = [%w[First alert], %w[Second alert]]
      [-> { T.recognize("Ada Lovelace", deadline_ms: 0) },
       -> { T.relate(pairs, relations: { caused_by: %w[alert alert] }, deadline_ms: 0) }].each do |call|
        call.call
      rescue T::DeadlineError => e
        say [e.kind, e.code]
      end
    RUBY
    assert_equal [[1, 2, 3, 4, 5, 6], ["usage", 1, true], ["deadline", 3], ["deadline", 3]], lines
    assert_equal 0, count
  end

  # The conformance runner holds the engine's fault kinds. A set is the
  # one question source it does not reach: a set file that fails to load
  # is local, and a blank inline question is usage.
  def test_a_bad_set_raises_local_from_a_file_and_usage_inline
    lines, count = TestBackend.run(<<~RUBY)
      file = File.join(ENV.fetch("HOME"), "set.json")
      File.write(file, '{"version":1,"questions":{"a":{"decide":"  "}}}')
      say kind_of_raise { T.set(file) }
      say kind_of_raise { T.set(File.join(ENV.fetch("HOME"), "missing.json")) }
      say kind_of_raise { T.set(a: { "decide" => "  " }) }
    RUBY
    assert_equal %w[LocalError LocalError UsageError], lines
    assert_equal 0, count
  end

  def test_named_single_questions_refuse_local_files_without_sends
    lines, count = TestBackend.run(<<~RUBY)
      folder = ENV.fetch("HOME")
      missing = File.join(folder, "missing-question.json")
      blank = File.join(folder, "blank-question.json")
      large = File.join(folder, "large-question.json")
      utf8 = File.join(folder, "utf8-question.json")
      unknown = File.join(folder, "unknown-question.json")
      valid = File.join(folder, "valid-é-question.json")
      wrong = File.join(folder, "choose-question.json")
      File.write(blank, '{"decide":"   "}')
      File.binwrite(large, "x" * 1_048_577)
      File.binwrite(utf8, [255].pack("C"))
      marker = "SYNTHETIC_PRIVATE_MARKER_0244"
      File.write(unknown, JSON.generate({ "decide" => "Question?", marker => 1 }))
      File.write(valid, '{"decide":"Question?"}')
      File.write(wrong, '{"choose":"Which?","options":["a","b"]}')
      seen = [missing, blank, large, utf8, unknown].map do |file|
        begin
          T.question(file: file)
          "accepted"
        rescue T::Error => error
          [error.kind, error.retryable, error.message.include?(file) || error.message.include?(marker)]
        end
      end
      [-> { T.question(file: 4) }, -> { T.question(file: [255].pack("C")) },
       -> { T.question(file: blank, decide: "x") },
       -> { T.decide(T.question(file: wrong), "text") },
       -> { T.question(decide: "   ") }].each do |call|
        begin
          call.call
          seen << "accepted"
        rescue T::Error => error
          seen << [error.kind, error.retryable]
        end
      end
      seen << (T.question(file: valid.b) ? "valid binary path" : "refused valid binary path")
      say seen
    RUBY
    assert_equal [["local", false, false]] * 5 + [["usage", false]] * 5 + ["valid binary path"], lines.fetch(0)
    assert_equal 0, count
  end

  def test_named_plans_refuse_unsafe_sources_and_mixed_inline_options_without_sends
    lines, count = TestBackend.run(<<~RUBY)
      folder = ENV.fetch("HOME")
      marker = "SYNTHETIC_PRIVATE_MARKER_0252"
      paths = %w[missing unknown wrong large utf8 malformed relation-unknown].map { |name| File.join(folder, "\#{name}.json") }
      File.write(paths[1], JSON.generate({ version: 1, recognize: { kinds: { person: "Person" } }, marker => 1 }))
      File.write(paths[2], '{"version":1,"relate":{"relations":[{"name":"knows","source":"person","target":"person","reads":"knows"}]}}')
      File.binwrite(paths[3], "x" * 1_048_577)
      File.binwrite(paths[4], [255].pack("C"))
      File.write(paths[5], '{"version":')
      File.write(paths[6], JSON.generate({ version: 1, relate: { relations: [{ name: "knows", source: "person", target: "person" }] }, marker => 1 }))
      seen = paths.map do |file|
        begin
          T.recognize("Ana", file: file)
          "accepted"
        rescue T::Error => error
          [error.kind, error.retryable, error.message.include?(file) || error.message.include?(marker)]
        end
      end
      begin
        T.relate([["Ana", "person"]], file: paths[6])
        seen << "accepted"
      rescue T::Error => error
        seen << [error.kind, error.retryable, error.message.include?(paths[6]) || error.message.include?(marker)]
      end
      [-> { T.recognize("Ana", file: paths[2], kinds: %w[person]) },
       -> { T.relate([["Ana", "person"]], file: paths[2], relations: %w[knows]) },
       -> { T.recognize("Ana", file: [255].pack("C")) },
       -> { T.relate([["Ana", "person"]], file: 4) }].each do |call|
        begin
          call.call
          seen << "accepted"
        rescue T::Error => error
          seen << [error.kind, error.retryable]
        end
      end
      say seen
    RUBY
    assert_equal [["local", false, false]] * 8 + [["usage", false]] * 4, lines.fetch(0)
    assert_equal 0, count
  end

  def test_no_message_or_inspect_line_holds_the_key_or_the_address_credentials
    lines, count, errors = TestBackend.run(<<~RUBY)
      base = ENV.fetch("THINKTHEN_BASE_URL").delete_suffix("/generic/v1")
      seen = []
      [-> { T::Engine.new(base_url: base.sub("127.0.0.1", "user:hunter2@127.0.0.1") + "/generic/v1") },
       -> { T::Engine.new(base_url: base + "/arm/refuse/v1", cache: false).decide("Is it urgent?", "text") },
       -> { T::Engine.new(base_url: base + "/arm/status/401/v1", cache: false).decide("Is it urgent?", "text") },
       -> { T.decide("Is it urgent?", "text", deadline_ms: "soon") },
       -> { T.decide("Is it urgent?", "text", cancel: T::Cancel.new.tap(&:cancel)) },
       -> { T.decide("Is it urgent?", "text", deadline_ms: 0) }].each do |call|
        call.call
      rescue T::Error => e
        seen << e.message << e.inspect << e.full_message
      end
      seen << T::Engine.new.inspect << T.details("Is it urgent?", "text").to_s
      say [seen.size, seen.grep(/\#{ENV.fetch("THINKTHEN_API_KEY")}|hunter2/)]
      warn seen.join("\n")
    RUBY
    assert_equal [[20, []]], lines
    assert_equal 3, count
    # The child printed every line it saw to stderr, so the check covers
    # what the process itself wrote too.
    refute_includes errors, TestBackend::FAKE_KEY
    refute_includes errors, "hunter2"
    assert_includes errors, "cancelled"
  end

  # Every result value prints the caller's text as a byte count, through
  # inspect, pp, to_s, and interpolation. Ranked's to_s is its record by
  # design, so it stays out. The loopback backend's recognize finds no
  # relation, so the relation is built from the names it found.
  def test_result_values_print_no_caller_text
    lines, count = TestBackend.run(<<~RUBY)
      require "pp"
      found = T.recognize("x MARK-Ana", kinds: %w[MARK-kind]).value
      first, last = found.entities
      relation = T::Relation.new("knows", first, last, 0.5, false)
      values = [relation, T::Recognized.new(found.entities, [relation]), found,
                T.relate([%w[MARK-Ana MARK-kind], %w[MARK-Bo MARK-kind]], relations: %w[knows]).value.first,
                T.rank("Is it urgent?", %w[MARK-one]).value.first, T.find("Which?", %w[MARK-one MARK-two]).value, T::Found.new]
      say values.map(&:inspect)
      say values.flat_map { |value| [value.pretty_inspect.chomp, *([value.to_s, "\#{value}"] unless value.is_a?(T::Ranked))] }.grep(/MARK/)
    RUBY
    one = "#<struct ThinkThen::RecognizedEntity text=<6 bytes withheld>, start=0, end=6, length=6, kind=<9 bytes withheld>, strength=0.6736>"
    two = "#<struct ThinkThen::RecognizedEntity text=<4 bytes withheld>, start=6, end=10, length=4, kind=<9 bytes withheld>, strength=0.6736>"
    ends = "source=#<struct ThinkThen::Entity name=<8 bytes withheld>, kind=<9 bytes withheld>>, " \
           "target=#<struct ThinkThen::Entity name=<7 bytes withheld>, kind=<9 bytes withheld>>"
    assert_equal [
      "#<struct ThinkThen::Relation relation=\"knows\", source=#{one}, target=#{two}, probability=0.5, either=false>",
      "#<struct ThinkThen::Recognized entities=2, relations=1>",
      "#<struct ThinkThen::Recognized entities=2, relations=nil>",
      "#<struct ThinkThen::Edge relation=\"knows\", #{ends}, probability=0.9, either=false>",
      "#<struct ThinkThen::Ranked index=0, record=<8 bytes withheld>, probability=0.9>",
      "#<struct ThinkThen::Found index=0, unit=<8 bytes withheld>, probability=0.9>",
      "#<struct ThinkThen::Found index=nil, unit=nil, probability=nil>"
    ], lines[0]
    assert_equal [], lines[1]
    assert_equal 5, count
  end
end
