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
  # platform gem: the build host's platform is stamped at build time, and
  # the extension's file is whichever name this platform builds - .so on
  # Linux, .bundle on the Mac - found at build, never hard-coded.
  spec.platform = Gem::Platform::CURRENT
  extension = Dir.glob("lib/thinkthen/thinkthen.{so,bundle}").first
  spec.files = ["lib/thinkthen.rb", "lib/thinkthen/version.rb", extension].compact
  spec.require_paths = ["lib"]
  # What is actually built and tested: the builder image carries 3.4, and
  # no earlier minor has been proven. Widen only when one is built and
  # run (the third review caught the untested ">= 3.1" claim).
  spec.required_ruby_version = ">= 3.4"
end
