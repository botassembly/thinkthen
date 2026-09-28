# frozen_string_literal: true

# Every Ruby function's example, run as one test. `examples.json` holds
# each function's call and the answer 0092's generic arm gives, plus any
# files the call names. The site's Ruby tab reads this file, so an example
# nobody runs cannot reach the site. Each example runs in its own scrubbed
# child against its own loopback backend, and its printed line must equal
# the expected text.
require "json"
require "open3"
require "tmpdir"
require_relative "backend"

data = JSON.parse(File.read(File.expand_path("../examples.json", __dir__)))
failed = 0
data["examples"].each do |name, example|
  backend = TestBackend::Backend.new
  got = Dir.mktmpdir do |root|
    (example["files"] || {}).each { |file, content| File.write(File.join(root, file), content) }
    out, errors, status = Open3.capture3(TestBackend.env(backend.url, root), RbConfig.ruby, "-I", TestBackend::LIB,
                                         "-rthinkthen", "-e", example["ruby"], chdir: root, unsetenv_others: true)
    status.success? ? out.strip : "error: #{errors.strip}"
  end
  backend.close
  if got == example["expected"]
    puts "ok #{name}"
  else
    failed += 1
    puts "FAIL #{name}: expected #{example['expected'].inspect}, got #{got.inspect}"
  end
end
puts "#{data['examples'].size - failed} of #{data['examples'].size} examples ok"
exit(failed.zero? ? 0 : 1)
