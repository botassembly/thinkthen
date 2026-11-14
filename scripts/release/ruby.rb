# frozen_string_literal: true

require 'json'
require 'thinkthen'


def options_for(definition)
  options = definition.reject { |key, _| %w[function question questions].include?(key) }.transform_keys(&:to_sym)
  if options[:threshold].is_a?(Hash)
    band = options[:threshold]
    options[:threshold] = band.fetch('low')..band.fetch('high')
  end
  if options[:rules]
    options[:rules] = options[:rules].map do |rule|
      fields = rule.transform_keys(&:to_sym)
      ThinkThen::RelationRule.new(**fields)
    end
  end
  options
end

def admitted(definition)
  function = definition.fetch('function')
  options = options_for(definition)
  klass = ThinkThen.const_get("#{function.capitalize}Question")
  return klass.new(**options) if %w[recognize relate].include?(function)
  if function == 'annotate'
    children = definition.fetch('questions').transform_values { |child| admitted(child) }
    return klass.new(ThinkThen::AnnotationQuestions.new(**children), **options)
  end
  klass.new(definition.fetch('question'), **options)
end

fixtures = JSON.parse(File.read(ARGV.fetch(0), encoding: Encoding::UTF_8), max_nesting: false)
raise 'no Ruby release cases' if fixtures.empty?
fixtures.each do |fixture|
  request = fixture.fetch('request')
  definition = request.fetch('question')
  function = definition.fetch('function')
  settings = fixture.fetch('client').transform_keys(&:to_sym)
  settings[:backend] = settings.fetch(:backend).to_sym
  if settings[:retry]
    policy = settings[:retry].transform_keys(&:to_sym)
    policy[:initial_backoff] = policy.delete(:initial_backoff_ms) / 1000.0 if policy.key?(:initial_backoff_ms)
    policy[:attempt_timeout] = policy.delete(:attempt_timeout_ms) / 1000.0 if policy.key?(:attempt_timeout_ms)
    settings[:retry] = ThinkThen::RetryPolicy.new(**policy)
  end
  ThinkThen::Client.open(**settings) do |client|
    records = request.fetch('records')
    expected = fixture.fetch('expected')
    if function == 'filter'
      kept, failed = [], []
      begin
        client.filter(admitted(definition), records) { |item| kept << item }
      rescue ThinkThen::FailedError => error
        failed = error.rows
      end
      expected_kept = expected.select { |row| row['status'] == 'answered' && row['answer'] }.map { |row| records.fetch(row.fetch('position') - 1) }
      unless kept.length == expected_kept.length && kept.zip(expected_kept).all? { |actual, original| actual.equal?(original) }
        raise "filter: expected original kept records #{expected_kept.inspect}, got #{kept.inspect}"
      end
      actual = failed.map(&:to_h)
      expected = expected.select { |row| row['status'] == 'failed' }
      raise "filter: expected failed rows #{expected.inspect}, got #{actual.inspect}" unless actual == expected
      raise 'filter: failed rows lost original records' unless failed.all? { |row| row.item.equal?(records.fetch(row.position - 1)) }
      next
    end
    stream = if %w[recognize relate].include?(function)
               client.public_send("#{function}_each", records, **options_for(definition))
             elsif function == 'rank'
               client.rank(admitted(definition), records)
             else
               client.public_send("#{function}_each", admitted(definition), records)
             end
    begin
      rows = stream.to_a
      actual = rows.map(&:to_h)
      raise "#{function}: expected #{expected.inspect}, got #{actual.inspect}" unless actual == expected
      raise "#{function}: lost original records" unless rows.all? { |row| row.item.equal?(records.fetch(row.position - 1)) }
    ensure
      stream.close
    end
  end
end
puts "candidate Ruby: #{fixtures.length} shared stub cases pass from installed gems"
