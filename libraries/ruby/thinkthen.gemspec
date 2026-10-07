# frozen_string_literal: true

# The thinkthen Ruby surface. Never published from this repository without
# Ian's word. The version is the thinkthen crate's.
Gem::Specification.new do |spec|
  spec.name = "thinkthen"
  manifest = File.read(File.expand_path("../../crates/thinkthen/Cargo.toml", __dir__))
  spec.version = manifest[/^version = "([^"]+)"/, 1]
  spec.summary = "The thinkthen surface for Ruby"
  spec.description = "The ten thinkthen verbs over the thinkthen engine."
  spec.authors = ["Ian Maurer"]
  spec.homepage = "https://thinkthen.dev"
  spec.metadata = {
    "source_code_uri" => "https://github.com/botassembly/thinkthen",
    "changelog_uri" => "https://github.com/botassembly/thinkthen/blob/main/CHANGELOG.md",
    "bug_tracker_uri" => "https://github.com/botassembly/thinkthen/issues"
  }
  spec.license = "MIT"
  if ENV["THINKTHEN_RUBY_FALLBACK"] == "1"
    spec.platform = Gem::Platform::RUBY
    spec.files = ["fallback/lib/thinkthen.rb", "lib/thinkthen/version.rb"]
    spec.require_paths = ["fallback/lib", "lib"]
    # Let unsupported Ruby versions select the current diagnostic package.
    spec.required_ruby_version = ">= 0"
    spec.post_install_message = "thinkthen #{spec.version}: no compatible native gem was selected. " \
      "Requires Ruby >= 3.4, < 4 on x86_64-linux, aarch64-linux, x86_64-darwin or arm64-darwin " \
      "(macOS 15.0 or later). This fallback cannot run ThinkThen; require \"thinkthen\" raises LoadError."
  else
    # Retain 0394: Darwin gems match every OS version; binaries keep the 15.0 floor.
    local = Gem::Platform.local
    spec.platform = local.os == "darwin" ? Gem::Platform.new([local.cpu, local.os]) : local
    extension = "lib/thinkthen/thinkthen.#{RbConfig::CONFIG["DLEXT"]}"
    raise "#{extension} is missing; run build.sh" unless File.file?(File.expand_path(extension, __dir__))

    spec.files = ["lib/thinkthen.rb", extension, "lib/thinkthen/version.rb", "lib/thinkthen/complete.rb", "lib/thinkthen/native_complete.rb"]
    spec.require_paths = ["lib"]
    # Built and tested on 3.4 only. 4.x is another ABI.
    spec.required_ruby_version = [">= 3.4", "< 4"].freeze
  end
end
