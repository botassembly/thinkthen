# frozen_string_literal: true

# Every Ruby function's example, run as one test.
#
# The file `examples.json` is keyed by function: the call and the answer
# the null backend gives, plus any question files the call names. The
# site's function pages and surface pages draw their Ruby tab from this
# file (site.md's one source), so an example nobody runs cannot reach the
# site.
#
# One fresh process per example keeps every expected answer
# deterministic. Offline, no stub, no key. Run by `check.sh`.
#
# Run with: ruby -I lib tests/examples.rb  (ENGINE_NULL=1)

require "json"
require "open3"
require "tmpdir"

HERE = File.expand_path(__dir__)
FILE = File.expand_path("../examples.json", __dir__)
LIB = File.expand_path("../lib", __dir__)

def run_one(example)
  Dir.mktmpdir do |workdir|
    (example["files"] || {}).each do |name, content|
      File.write(File.join(workdir, name), content)
    end
    program = "require \"thinkthen\"\n#{example['ruby']}\n"
    out, err, status = Open3.capture3(
      { "ENGINE_NULL" => "1" }, RbConfig.ruby, "-I", LIB, "-e", program, chdir: workdir
    )
    return status.success? ? out.strip : "error: #{err.strip}"
  end
end

data = JSON.parse(File.read(FILE))
failed = 0
data["examples"].each do |name, example|
  want = example["expected"]
  got = run_one(example)
  if got.include?(want)
    puts format("ok       %s", name)
  else
    puts format("FAILED   %s: want %s, got %s", name, want.inspect, got.inspect)
    failed += 1
  end
end
puts format("%d of %d examples ok", data["examples"].length - failed, data["examples"].length)
exit 1 if failed.positive?
