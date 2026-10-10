# frozen_string_literal: true

require "minitest/autorun"
require "rubygems/package"
require "zlib"
require "open3"
require "tmpdir"
require "fileutils"
require_relative "../../../conformance/children/children"

class TestPackage < Minitest::Test
  ROOT = File.expand_path("..", __dir__)
  VERSION = File.read(File.expand_path("../../crates/thinkthen/Cargo.toml", ROOT))[/^version = "([^"]+)"/, 1]
  FALLBACK = File.expand_path(ENV.fetch("THINKTHEN_ARTIFACT", "#{ROOT}/thinkthen-#{VERSION}.gem"))
  DIAGNOSTIC = "thinkthen #{VERSION}: no compatible native gem was selected. " \
    "Requires Ruby >= 3.4, < 4 on x86_64-linux, aarch64-linux, x86_64-darwin or arm64-darwin " \
    "(macOS 15.0 or later). This fallback cannot run ThinkThen."

  def child(root, code, *args)
    Open3.capture3(Children.env(keep: %w[LD_LIBRARY_PATH DYLD_LIBRARY_PATH], home: root, GEM_HOME: "#{root}/gems",
                              GEM_PATH: "#{root}/gems", LANG: "C.UTF-8"),
                  RbConfig.ruby, "-e", code, *args, chdir: root, unsetenv_others: true)
  end

  def test_fallback_install_explains_support_and_import_refuses_to_work
    spec = Gem::Package.new(FALLBACK).spec
    assert_equal VERSION, spec.version.to_s
    assert_equal "ruby", spec.platform.to_s
    assert_equal ["fallback/lib/thinkthen.rb", "lib/thinkthen/version.rb"], spec.files.sort
    assert_equal ["MIT"], spec.licenses
    %w[2.7 3.3 3.4 4.0].each { |version| assert spec.required_ruby_version.satisfied_by?(Gem::Version.new(version)) }
    assert_equal DIAGNOSTIC.delete_suffix(".") + '; require "thinkthen" raises LoadError.', spec.post_install_message
    Dir.mktmpdir("thinkthen-ruby-package-") do |root|
      out, errors, status = child(root, 'require "rubygems/gem_runner"; Gem::GemRunner.new.run(ARGV)',
                                 "install", "--local", "--no-document", FALLBACK)
      assert_equal 0, status.exitstatus, errors
      assert_includes out, spec.post_install_message
      out, errors, status = child(root, 'require "thinkthen"')
      assert_equal 1, status.exitstatus
      assert_equal "", out
      assert_includes errors, DIAGNOSTIC + " (LoadError)"
      refute_includes errors, "0.0.1"
    end
  end

  def test_local_selection_prefers_native_only_for_supported_ruby_and_platform
    native = Gem::Package.new(Dir["#{ROOT}/thinkthen-#{VERSION}-*.gem"].fetch(0))
    assert_equal ">= 3.4, < 4", native.spec.required_ruby_version.to_s
    Dir.mktmpdir("thinkthen-ruby-selection-") do |root|
      source = "#{root}/source"
      native.extract_files(source)
      FileUtils.cp(FALLBACK, root)
      Dir.chdir(source) do
        %w[x86_64-linux aarch64-linux x86_64-darwin arm64-darwin].each do |platform|
          spec = native.spec.dup
          spec.platform = Gem::Platform.new(platform)
          FileUtils.mv(Gem::Package.build(spec, true), root)
        end
        old = Gem::Specification.new do |spec|
          spec.name = "thinkthen"
          spec.version = "0.0.1"
          spec.summary = "Historical placeholder fixture"
          spec.authors = ["fixture"]
        end
        FileUtils.mv(Gem::Package.build(old, true), root)
      end
      repository = "#{root}/repository"
      FileUtils.mkdir_p("#{repository}/gems")
      FileUtils.cp(Dir["#{root}/*.gem"], "#{repository}/gems")
      FileUtils.mkdir_p("#{repository}/quick/Marshal.4.8")
      tuples = Dir["#{repository}/gems/*.gem"].map do |file|
        spec = Gem::Package.new(file).spec
        File.binwrite("#{repository}/quick/Marshal.4.8/#{spec.full_name}.gemspec.rz", Zlib.deflate(Marshal.dump(spec)))
        [spec.name, spec.version, spec.platform.to_s]
      end.sort
      %w[specs latest_specs prerelease_specs].each do |name|
        Zlib::GzipWriter.open("#{repository}/#{name}.4.8.gz") { |out| out.write(Marshal.dump(name == "prerelease_specs" ? [] : tuples)) }
      end
      cases = %w[x86_64-linux aarch64-linux x86_64-darwin-24 arm64-darwin-25 x64-mingw-ucrt riscv64-linux].product(%w[3.3 3.4 4.0])
      cases.each_with_index do |(platform, ruby), index|
        home = "#{root}/consumer-#{index}"
        FileUtils.mkdir_p(home)
        code = <<~CODE
          require "rubygems/gem_runner"
          Gem.platforms = [Gem::Platform::RUBY, Gem::Platform.new(ARGV.shift)]
          simulated_ruby = Gem::Version.new(ARGV.shift)
          Gem.define_singleton_method(:ruby_version) { simulated_ruby }
          Gem::GemRunner.new.run(["install", "thinkthen", "--clear-sources", "--source", ARGV.shift, "--no-document", "--install-dir", ARGV.shift])
        CODE
        # Search the fixture repository, installing into an isolated consumer home.
        out, errors, status = child(root, code, platform, ruby, "file://#{repository}", "#{home}/gems")
        assert_equal 0, status.exitstatus, "#{platform} Ruby #{ruby}: #{out} #{errors}"
        specs = Dir["#{home}/gems/specifications/*.gemspec"].map { |file| Gem::Specification.load(file) }
        assert_equal 1, specs.size
        selected = specs.fetch(0)
        assert_equal VERSION, selected.version.to_s
        supported = ruby == "3.4" && !%w[x64-mingw-ucrt riscv64-linux].include?(platform)
        assert_equal supported ? platform.sub(/-2[45]$/, "") : "ruby", selected.platform.to_s
        if supported
          refute_includes out, DIAGNOSTIC.delete_suffix(".")
        else
          assert_includes out, DIAGNOSTIC.delete_suffix(".")
        end
        # Only the actual host's native binary is an import claim.
        next unless supported && platform == Gem::Platform.local.to_s

        out, errors, status = child(home, 'require "thinkthen"; abort unless ThinkThen::Client && ThinkThen::VERSION == ARGV[0]; abort unless $LOADED_FEATURES.grep(%r{/lib/thinkthen(\.rb|/)}).all? { |path| path.start_with?(ENV.fetch("GEM_HOME")) }', VERSION)
        assert_equal 0, status.exitstatus, "#{out} #{errors}"
      end
    end
  end
end
