# frozen_string_literal: true

require_relative "lib/thinkthen/version"

Gem::Specification.new do |spec|
  spec.name = "thinkthen"
  spec.version = ThinkThen::VERSION
  spec.summary = "Ask typed questions about text"
  spec.authors = ["Ian Maurer"]
  spec.license = "MIT"
  spec.homepage = "https://github.com/botassembly/thinkthen"
  spec.required_ruby_version = ">= 3.3"
  spec.platform = Gem::Platform.new(File.read("PLATFORM").strip)
  spec.files = ["README.md", "API.md", "LICENSE", *Dir["{lib,sig}/**/*"].select { |path| File.file?(path) }]
  spec.metadata = {
    "source_code_uri" => spec.homepage,
    "changelog_uri" => spec.homepage + "/releases",
    "bug_tracker_uri" => spec.homepage + "/issues",
    "documentation_uri" => spec.homepage + "/blob/main/sdk/ruby/API.md",
    "rubygems_mfa_required" => "true"
  }
  spec.require_paths = ["lib"]
  spec.add_runtime_dependency "ffi", "~> 1.17"
end
