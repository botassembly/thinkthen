# frozen_string_literal: true
require "minitest/autorun"
require_relative "backend"

class TestExamples < Minitest::Test
  def test_public_readme_example_runs_through_the_installed_gem
    sample = File.read(File.expand_path("../README.md", __dir__)).match(/```ruby\n(.*?)```/m)[1]
    TestBackend.with(<<~CODE) do |backend, child|
      require "stringio"
      capture = StringIO.new
      original = $stdout
      begin
        $stdout = capture
        #{sample}
      ensure
        $stdout = original
      end
      say capture.string.lines.map(&:strip)
    CODE
      assert_equal ["true", "true", "true", "billing", "billing", "billing"], child.hear
      status, errors = child.finish
      assert status.success?, errors
      assert_equal 6, backend.count
    end
  end

  def test_shipped_examples_run_through_the_installed_gem
    examples = JSON.parse(File.read(File.expand_path("../examples.json", __dir__))).fetch("examples")
    examples.each do |verb, example|
      script = <<~CODE
        Dir.chdir(ENV.fetch("HOME")) do
          #{example.fetch("files", {}).map { |path, content| "File.write(#{path.dump}, #{content.dump})" }.join("\n")}
          #{example.fetch("ruby")}
        end
      CODE
      TestBackend.with(script) do |backend, child|
        assert_equal JSON.parse(example.fetch("expected")), child.hear, verb
        status, errors = child.finish
        assert status.success?, errors
        assert_operator backend.count, :>, 0
      end
    end
  end
end
