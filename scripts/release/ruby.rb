# frozen_string_literal: true

require 'json'
require 'thinkthen'

JSON.parse(File.read(ARGV.fetch(0)), max_nesting: false).each do |fixture|
  request = fixture.fetch('request')
  question = request.fetch('question')
  value = case question.fetch('type')
          when 'inline' then question.fetch('value')
          when 'toolbox' then ThinkThen.toolbox(question.fetch('value'), name: question['name'])
          when 'reference' then ThinkThen.reference(question.fetch('name'))
          end
  ThinkThen::Client.open(**fixture.fetch('client').transform_keys(&:to_sym)) do |client|
    rows = client.public_send(request.fetch('function'), value, request.fetch('records'),
                             **request.fetch('options', {}))
    actual = rows.map(&:to_h)
    raise request.fetch('function') unless actual == fixture.fetch('expected')
  end
end
puts 'candidate Ruby: ten shared stub cases pass from installed gems'
