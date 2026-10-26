# frozen_string_literal: true

require "open3"
require "rbconfig"

def readme_examples
  readme = File.join(Gem::Specification.find_by_name("thinkthen").full_gem_path, "README.md")
  examples = File.read(readme, encoding: Encoding::UTF_8).scan(/```ruby\n(.*?)```/m).flatten
  raise "no Ruby README examples" if examples.empty?
  examples.each_with_index do |example, index|
    # The cancellation excerpt continues an existing client and record source.
    source = <<~RUBY
      require "thinkthen"
      ThinkThen::Client.open(backend: :stub) do |client|
        records = ["Please check my order."]
        #{example}
      end
    RUBY
    output, status = Open3.capture2e(RbConfig.ruby, "-w", "-e", source)
    raise "README Ruby example #{index + 1} failed:\n#{output}" unless status.success?
  end
  puts "installed Ruby gem: all #{examples.length} README code examples pass"
end

readme_examples if $PROGRAM_NAME == __FILE__
