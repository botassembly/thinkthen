# frozen_string_literal: true

require "open3"
require "rbconfig"

def readme_examples
  readme = File.join(Gem::Specification.find_by_name("thinkthen").full_gem_path, "README.md")
  examples = File.read(readme, encoding: Encoding::UTF_8).scan(/```ruby\n(.*?)```/m).flatten
  raise "no Ruby README examples" if examples.empty?
  # The full literal runner owns guide continuations, scenarios and promised refusals.
  # This separate installed-package smoke check executes the first self-contained quickstart.
  examples.first(1).each_with_index do |example, index|
    # The quickstart opens its own default client, as a reader's code does.
    output, status = Open3.capture2e({"THINKTHEN_BACKEND" => "stub"}, RbConfig.ruby, "-w", "-e", example)
    raise "README Ruby example #{index + 1} failed:\n#{output}" unless status.success?
  end
  puts "installed Ruby gem: README quickstart passes"
end

readme_examples if $PROGRAM_NAME == __FILE__
