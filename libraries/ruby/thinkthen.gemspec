# frozen_string_literal: true

# The thinkthen Ruby surface. Never published from this repository without
# Ian's word. The version is the thinkthen crate's.
Gem::Specification.new do |spec|
  spec.name = "thinkthen"
  manifest = File.read(File.expand_path("../../crates/thinkthen/Cargo.toml", __dir__))
  spec.version = manifest[/^version = "([^"]+)"/, 1]
  spec.summary = "The thinkthen surface for Ruby"
  spec.description = "The ten thinkthen verbs over the thinkthen engine."
  spec.authors = ["thinkthen"]
  spec.homepage = "https://example.invalid/thinkthen"
  spec.license = "MIT"
  # A platform gem: it carries the extension built on this host, under the
  # name this host's Ruby loads.
  spec.platform = Gem::Platform::CURRENT
  extension = "lib/thinkthen/thinkthen.#{RbConfig::CONFIG["DLEXT"]}"
  raise "#{extension} is missing; run build.sh" unless File.file?(File.expand_path(extension, __dir__))

  spec.files = ["lib/thinkthen.rb", extension, "lib/thinkthen/version.rb"]
  spec.require_paths = ["lib"]
  # Built and tested on 3.4 only. 4.x is another ABI.
  spec.required_ruby_version = [">= 3.4", "< 4"].freeze
end
