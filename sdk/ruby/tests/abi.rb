# frozen_string_literal: true

require "fileutils"
require "rbconfig"
require "tmpdir"

# Use the shared ABI libraries through the gem's ordinary packaged loader.
def native_abi_cases
  manifest = JSON.parse(File.read(File.join(ENV.fetch("THINKTHEN_NATIVE_FIXTURES"), "manifest.json"), encoding: Encoding::UTF_8))
  installed = File.dirname($LOADED_FEATURES.find { |path| path.end_with?("/lib/thinkthen.rb") })
  manifest.fetch("cases").each do |test|
    Dir.mktmpdir("ruby-abi-") do |temporary|
      FileUtils.cp_r(File.join(installed, "."), temporary)
      library = Dir[File.join(temporary, "thinkthen", "*thinkthen_ffi.*")].fetch(0)
      FileUtils.cp(test.fetch("library"), library)
      source = <<~'RUBY'
        begin
          require "thinkthen"
          abort "require eagerly loaded the engine" if ThinkThen.const_defined?(:Native, false)
          ThinkThen::Client.open(backend: :stub) do |client|
            row = client.decide("Ready?", ARGV.fetch(0) == "unknown-error" ? nil : "record")
            if ARGV.fetch(0) == "unknown-error"
              abort "lost raw code" unless row.error.code == :unknown && row.error.raw_code == 2147483647
              abort "lost engine message" if row.error.message.empty?
              puts "unknown"
              exit
            end
            abort "wrong row" unless row.function == :decide && row.position == 1 && row.source.backend == :stub
          end
          puts "accepted"
        rescue ThinkThen::Error => error
          abort "wrong compatibility code" unless error.code == :"usage.abi"
          puts "rejected"
        end
      RUBY
      output, status = Open3.capture2e(RbConfig.ruby, "-I", temporary, "-e", source, test.fetch("name"))
      assert(status.success?, "ABI child #{test.fetch('name')}: #{output}")
      equal(test.fetch("name") == "unknown-error" ? "unknown" : test.fetch("accept") ? "accepted" : "rejected", output.strip, "shared ABI #{test.fetch('name')}")
    end
  end
  puts "installed Ruby gem: shared ABI acceptance and refusal cases pass"
end
