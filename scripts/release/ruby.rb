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
  settings[:retry] = ThinkThen::Retry.new(**settings[:retry].transform_keys(&:to_sym)) if settings[:retry]
  ThinkThen::Client.open(**settings) do |client|
    stream = client.public_send("#{function}_each", admitted(definition), request.fetch('records'))
    begin
      actual = stream.map do |value|
        row = value.is_a?(ThinkThen::FilterRow) ? value.row : value
        row.to_h
      end
      raise "#{function}: expected #{fixture.fetch('expected').inspect}, got #{actual.inspect}" unless actual == fixture.fetch('expected')
    ensure
      stream.close
    end
  end
end
puts 'candidate Ruby: ten shared stub cases pass from installed gems'
