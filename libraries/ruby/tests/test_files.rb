# frozen_string_literal: true
require "minitest/autorun"
require_relative "backend"

class TestFiles < Minitest::Test
  def test_explicit_reader_keeps_original_sources_for_every_verb
    documents = File.expand_path("../../../specification/fixtures/files/documents", __dir__)
    questions = [{decide:"Q?"}, {choose:"Q?",options:["policy","contract"]},
      {tag:"Q?",labels:["refund","support"]}, {score:"Q?",levels:["low","high"]},
      {filter:"Q?"}, {rank:"Q?"}, {find:"Q?"},
      {annotate:{version:1,questions:{urgent:{decide:"Q?"}}}},
      {version:1,recognize:{kinds:{person:nil}}},
      {version:1,relate:{relations:[{name:"supports",source:"*",target:"*"}]}}]
    backend = TestBackend::Backend.new
    begin
      Dir.mktmpdir do |root|
        child = TestBackend::Child.new(TestBackend.env(backend.url.sub("/generic/", "/arm/full/"), root), <<~RUBY)
          engine = T::Engine.new(cache: false)
          questions = JSON.parse(#{JSON.generate(questions).inspect})
          calls = questions.map { |q| engine.files(q, #{documents.inspect}, unit: "file") }
          calls.each_with_index do |call, at|
            raise "no requests" unless call.facts.fetch(:requests_sent) > 0
            rows = at == 9 ? call.value.fetch("edges").flat_map { |e| [e.fetch("source"),e.fetch("target")] } : at == 6 ? [call.value] : call.value
            raise "no rows" if rows.empty?
            rows.each do |row|
              raise "source lost" unless row.fetch("first_line") == 1 && row.fetch("last_line") == 4 && row.fetch("record").include?("\\n")
            end
          end
          say calls.length
        RUBY
        assert_equal 10, child.hear
        status, errors = child.finish
        assert status.success?, errors
      end
    ensure
      backend.close
    end
  end
end
