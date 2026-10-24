# frozen_string_literal: true

require "coverage"
require "json"
Coverage.start(lines: true, branches: true)
require "thinkthen"
require_relative "cases"
require_relative "surface"

loaded = $LOADED_FEATURES.find { |path| path.end_with?("/lib/thinkthen.rb") }
raise "source checkout was loaded" unless loaded && File.realpath(loaded).start_with?(File.realpath(ENV.fetch("GEM_HOME")) + "/")
ENV["THINKTHEN_API_KEY"] = "synthetic-loopback-key"

shared_cases
ruby_surface
held_interrupt
held_cancellation
held_close

coverage = Coverage.result.select { |path, _| path.include?("/gems/thinkthen-") }
if ENV["THINKTHEN_RUBY_COVERAGE"]
  result = coverage.transform_keys { |path| path.split("/lib/").last }
  File.write(ENV.fetch("THINKTHEN_RUBY_COVERAGE"), JSON.pretty_generate(result))
end
missed = coverage.values.sum { |value| value.fetch(:lines).count(0) }
puts "Ruby source coverage: #{missed} executable lines not reached by installed tests"
