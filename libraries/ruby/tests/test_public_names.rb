# frozen_string_literal: true

# The loaded module's public names equal one pinned list (R5-14, R6-12). A
# name the native half defines counts too, and the private Native module,
# the watchdog's row, and the helpers stay out of it.
require "json"
require "minitest/autorun"
require "open3"
require "rbconfig"
require "tmpdir"
require_relative "../../../conformance/children/children"

class TestPublicNames < Minitest::Test
  CONSTANTS = %i[BackendError Call Cancel CancelledError Complete Completion DeadlineError DefectError Edge Engine Entity Error Found
                 LocalError NO Question QuestionSet Ranked Recognized RecognizedEntity Relation UNSURE UsageError VERSION YES].freeze
  VERBS = %i[annotate choose choose_many decide decide_many decide_many_with_probabilities details files filter find plan rank
             recognize relate score score_many score_with_level tag tag_many usage with_tick].freeze

  def test_the_loaded_module_shows_only_the_pinned_names
    out, errors, status = Dir.mktmpdir("thinkthen-ruby-names-") do |root|
      Open3.capture3(Children.env(keep: %w[LD_LIBRARY_PATH], home: root), RbConfig.ruby, "-I", File.expand_path("../lib", __dir__),
                                         "-rjson", "-rthinkthen", "-e", <<~RUBY, unsetenv_others: true)
      T = ThinkThen
      puts JSON.generate(
        constants: T.constants.sort,
        module: T.singleton_methods.sort,
        engine: T::Engine.public_instance_methods(false).sort,
        engine_class: T::Engine.singleton_methods.sort,
        cancel: T::Cancel.public_instance_methods(false).sort,
        question: T::Question.public_instance_methods(false).sort,
        set: T::QuestionSet.public_instance_methods(false).sort,
        ranked: T::Ranked.public_instance_methods(false).sort
      )
    RUBY
    end
    assert status.success?, errors
    names = JSON.parse(out, symbolize_names: true).transform_values { |list| list.map(&:to_sym) }
    assert_equal({
                   constants: CONSTANTS,
                   module: (VERBS + %i[failed outcome question set]).sort,
                   engine: (VERBS + %i[complete inspect]).sort,
                   engine_class: [],
                   cancel: %i[cancel cancelled?],
                   question: %i[json],
                   set: %i[names],
                   ranked: %i[index index= probability probability= record record= to_s]
                 }, names)
  end
end
