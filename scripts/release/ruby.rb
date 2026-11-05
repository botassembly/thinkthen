# frozen_string_literal: true

require 'json'
require 'thinkthen'


def admitted(definition)
  function = definition.fetch('function')
  options = definition.reject { |key, _| %w[function question questions].include?(key) }.transform_keys(&:to_sym)
  if options[:threshold].is_a?(Hash)
    band = options[:threshold]
    options[:threshold] = band.fetch('low')..band.fetch('high')
  end
  if options[:rules]
    options[:rules] = options[:rules].map do |rule|
      fields = rule.to_h { |key, value| [{'reads' => :phrase, 'either' => :symmetric}.fetch(key, key.to_sym), value] }
      ThinkThen::RelationRule.new(**fields)
    end
  end
  klass = ThinkThen.const_get("#{function.capitalize}Question")
  return klass.new(**options) if %w[recognize relate].include?(function)
  if function == 'annotate'
    children = definition.fetch('questions').transform_values { |child| admitted(child) }
    return klass.new(ThinkThen::AnnotationQuestions.new(questions: children), **options)
  end
  klass.new(definition.fetch('question'), **options)
end

JSON.parse(File.read(ARGV.fetch(0)), max_nesting: false).each do |fixture|
  request = fixture.fetch('request')
  definition = request.fetch('question')
  function = definition.fetch('function')
  settings = fixture.fetch('client').transform_keys(&:to_sym)
  settings[:backend] = settings.fetch(:backend).to_sym
  if settings[:retry]
    policy = settings[:retry].transform_keys(&:to_sym)
    policy[:initial_backoff] = policy.delete(:initial_backoff_ms) / 1000.0 if policy.key?(:initial_backoff_ms)
    policy[:attempt_timeout] = policy.delete(:attempt_timeout_ms) / 1000.0 if policy.key?(:attempt_timeout_ms)
    settings[:retry] = ThinkThen::Retry.new(**policy)
  end
  ThinkThen::Client.open(**settings) do |client|
    stream = client.public_send("#{function}_each", admitted(definition), request.fetch('records'))
    begin
      actual = stream.map do |value|
        row = value.is_a?(ThinkThen::FilterRow) || value.is_a?(ThinkThen::RankRow) ? value.row : value
        row.to_h
      end
      raise "#{function}: expected #{fixture.fetch('expected').inspect}, got #{actual.inspect}" unless actual == fixture.fetch('expected')
    ensure
      stream.close
    end
  end
end
puts 'candidate Ruby: ten shared stub cases pass from installed gems'
