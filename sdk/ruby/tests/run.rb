# frozen_string_literal: true

require "coverage"
require "json"
Coverage.start(lines: true, branches: true)
require_relative "cases"

isolated_ruby_environment do
  require "thinkthen"
  require_relative "surface"
  require_relative "lifecycle"
  require_relative "input_interruption"
  require_relative "traversal_lifetime"
  require_relative "allocator"
  require_relative "toolbox_json"
  require_relative "contracts"
  require_relative "abi"
  require_relative "readme"
  loaded = $LOADED_FEATURES.find { |path| path.end_with?("/lib/thinkthen.rb") }
  raise "source checkout was loaded" unless loaded && File.realpath(loaded).start_with?(File.realpath(ENV.fetch("GEM_HOME")) + "/")

  ruby_packaging
  shared_cases
  admission_cases
  ruby_saved_calls
  ruby_toolbox_json
  ruby_questions
  ruby_threshold_and_evidence
  ruby_record_identity
  ruby_filter_rows
  ruby_row_values
  ruby_abandoned_streams
  ruby_single_admission
  ruby_fork_ownership
  ruby_enumeration
  ruby_suspended_traversal
  ruby_native_allocator
  ruby_input_ownership
  ruby_input_interruptions
  ruby_held_calls
  ruby_reader_lifetime
  ruby_completion_interruption
  ruby_repeated_interruption
  ruby_scheduler_cancellation
  ruby_execution_names
  ruby_conversion_contract
  ruby_key_snapshots
  ruby_configuration
  native_abi_cases
  readme_examples
end

coverage = Coverage.result.select { |path, _| path.include?("/gems/thinkthen-") }
if ENV["THINKTHEN_RUBY_COVERAGE"]
  result = coverage.transform_keys { |path| path.split("/lib/").last }
  File.write(ENV.fetch("THINKTHEN_RUBY_COVERAGE"), JSON.pretty_generate(result))
end
missed = coverage.values.sum { |value| value.fetch(:lines).count(0) }
puts "Ruby source coverage: #{missed} executable lines not reached by installed tests"
