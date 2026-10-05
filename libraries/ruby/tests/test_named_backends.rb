# frozen_string_literal: true
require "minitest/autorun"
require_relative "backend"

class TestNamedBackends < Minitest::Test
  ROWS = JSON.parse(File.read(File.expand_path("../../../conformance/binding-backends.json", __dir__))).fetch("backends")
  SLOTS = %w[generic_systemone generic_decisions generic_custom capture_systemone capture_decisions capture_custom other non_post].freeze

  def test_provider_selection_keeps_key_model_path_and_question_form
    ROWS.each do |row|
      marker = "fake-named-ruby-#{row.fetch('name')}"
      script = <<~RUBY
        engine = T::Engine.new(backend: #{row.fetch('name').inspect}, base_url: "http://127.0.0.1:PORT/arm/full/capture/v1", cache: false)
        ENV[#{row.fetch('key').inspect}] = 'fake-later'
        question = T.question(decide: 'Does it need attention?', true: {what: 'yes', examples: ['refund']})
        say engine.decide(question, 'refund').value
      RUBY
      # The port is fixed after the backend starts, before the child builds its engine.
      backend = TestBackend::Backend.new("captured" => marker, "later" => "fake-later", "unnamed" => TestBackend::FAKE_KEY)
      begin
        Dir.mktmpdir do |root|
          child = TestBackend::Child.new(TestBackend.env(backend.url, root, row.fetch("key") => marker), script.gsub("PORT", backend.port.to_s))
          assert_equal true, child.hear
          status, errors = child.finish
          assert status.success?, errors
          refute_includes errors, marker
          assert_equal 1, backend.count
          assert_equal SLOTS.to_h { |slot| [slot, slot == row.fetch("path") ? 1 : 0] }.merge("overflow" => false), backend.snapshot("paths")
          assert_equal({"markers" => {"captured" => 1, "later" => 0, "unnamed" => 0}, "absent" => 0, "unknown" => 0, "overflow" => false}, backend.snapshot("bearers"))
          body = JSON.parse(backend.capture.fetch(0))
          assert_equal row.fetch("model"), body.fetch("model")
          criteria = body.fetch("questions").fetch("q1").fetch("criteria")
          assert_equal(row.fetch("form") == "text" ? "yes" : {"what" => "yes", "examples" => ["refund"]}, criteria.fetch("true"))
          if row.fetch("form") == "both"
            assert_equal({}, criteria.fetch("false"))
          else
            refute criteria.key?("false")
          end
        end
      ensure
        backend.close
      end
    end
  end

  def test_invalid_backend_values_send_nothing
    values, count = TestBackend.run(<<~RUBY)
      [1, true, [], {}, '', 'nowhere'].each do |value|
        say kind_of_raise { T::Engine.new(backend: value, cache: false) }
      end
    RUBY
    assert_equal ["UsageError"] * 6, values
    assert_equal 0, count
  end
end
