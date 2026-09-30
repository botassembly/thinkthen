# frozen_string_literal: true
# The shared settings corpus through Ruby's keyword surface.
require "minitest/autorun"
require_relative "backend"

class TestSettingsCases < Minitest::Test
  CASES = JSON.parse(File.read(File.expand_path("../../../conformance/settings.json", __dir__)))

  def test_shared_settings
    assert_equal "thinkthen.settings-cases/1", CASES.fetch("schema")
    CASES.fetch("cases").each do |entry|
      steps = entry.fetch("steps")
      script = <<~RUBY
        folder = File.join(ENV.fetch("HOME"), "recording")
        Dir.mkdir(folder)
        profile = File.join(ENV.fetch("HOME"), "profile.json")
        File.write(profile, #{JSON.generate(entry["profile"]).inspect}) if #{entry.key?("profile")}
        #{steps.map { |step| step_script(step, entry) }.join("\n")}
      RUBY
      TestBackend.with(script, arm: entry.fetch("arm").delete_suffix("/v1")) do |backend, child, root|
        steps.each do |step|
          got = child.hear(8)
          if step.key?("error")
            assert_equal step.fetch("error"), got.fetch("error"), entry.fetch("id")
          elsif step["verb"] == "relate"
            assert_equal step.fetch("edges"), got.fetch("value"), entry.fetch("id")
          else
            assert_equal step.fetch("value"), got.fetch("value"), entry.fetch("id")
            assert_equal step.fetch("model"), got.fetch("model"), entry.fetch("id") if step.key?("model")
          end
          assert_equal step.fetch("count"), backend.count, entry.fetch("id")
          child.tell
        end
        status, errors = child.finish
        assert status.success?, errors
        if entry.key?("entries")
          # A recording keeps its answers in one question store by ADR 0111.
          # Ruby reads no SQLite without an extra gem, so this checks the
          # store and that no old per-request entry remains.
          recording = File.join(root, "home", "recording")
          assert File.file?(File.join(recording, "thinkthen.sqlite")), entry.fetch("id")
          assert_empty Dir.glob("*.json", base: recording).grep(/\A\h{64}\.json\z/), entry.fetch("id")
        end
      end
    end
  end

  def step_script(step, entry)
    <<~RUBY
      settings = JSON.parse(#{JSON.generate(step.fetch("settings")).inspect}).transform_keys(&:to_sym)
      settings.transform_values! { |value| value == "$FOLDER" ? folder : value == "$PROFILE" ? profile : value }
      begin
        engine = T::Engine.new(**settings)
        value = if #{step["verb"] == "relate"}
                  engine.relate(#{entry.fetch("entities", []).map { |one| [one.fetch("name"), one.fetch("kind")] }.inspect},
                    relations: { linked: %w[item item] }).value.length
                elsif #{step["verb"] == "decide_many"}
                  engine.decide_many(#{CASES.fetch("question").inspect}, #{step.fetch("records", []).inspect}, batch: 1).value
                elsif #{step.key?("model")}
                  details = engine.details(#{CASES.fetch("question").inspect}, #{step.fetch("text", "").inspect}).value
                  { "value" => details["value"], "model" => details["meta"]["model"] }
                else
                  engine.decide(#{CASES.fetch("question").inspect}, #{step.fetch("text", "").inspect}).value
                end
        say(value.is_a?(Hash) ? value : { "value" => value })
      rescue T::Error => error
        say("error" => error.kind)
      end
      hear
    RUBY
  end
end
