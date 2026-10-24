# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "thinkthen"
  spec.version = File.read("VERSION").strip
  spec.summary = "Ask typed questions about text"
  spec.authors = ["Ian Maurer"]
  spec.license = "MIT"
  spec.homepage = "https://github.com/botassembly/thinkthen"
  spec.required_ruby_version = ">= 3.3"
  spec.platform = Gem::Platform.new(File.read("PLATFORM").strip)
  spec.files = ["README.md", "LICENSE", *Dir["lib/**/*"].select { |path| File.file?(path) }]
  spec.require_paths = ["lib"]
  spec.add_runtime_dependency "ffi", "~> 1.17"
end
