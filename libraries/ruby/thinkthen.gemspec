# frozen_string_literal: true

# The thinkthen Ruby surface. Never published from this repository without
# Ian's word; the version follows the repository's one VERSION file.
Gem::Specification.new do |spec|
  spec.name = "thinkthen"
  spec.version = File.read(File.expand_path("../../VERSION", __dir__)).lines.find { |line| line =~ /\A\d/ }.strip
  spec.summary = "The thinkthen surface for Ruby"
  spec.description = "The eight thinkthen verbs over one engine. Not the first release."
  spec.authors = ["thinkthen"]
  spec.homepage = "https://example.invalid/thinkthen"
  spec.license = "MIT"
  # The gem carries a compiled extension built on the host, so it is a
  # platform gem: the build host's platform is stamped at build time.
  spec.platform = Gem::Platform::CURRENT
  spec.files = ["lib/thinkthen.rb", "lib/thinkthen/version.rb", "lib/thinkthen/thinkthen.so"]
  spec.require_paths = ["lib"]
  spec.required_ruby_version = ">= 3.1"
end
