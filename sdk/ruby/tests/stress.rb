# frozen_string_literal: true
require "json"
require "thinkthen"
# Feature: Ruby callers retain positions in independent streams on one client.
test = JSON.parse(File.read(ENV.fetch("THINKTHEN_STRESS_CASE")))
settings = test.fetch("client").transform_keys(&:to_sym)
settings[:backend] = settings.fetch(:backend).to_sym
settings[:retry] = ThinkThen::Retry.new(**settings.fetch(:retry).transform_keys(&:to_sym))
count = test.fetch("records")
widths = {}
failed = 0
mutex = Mutex.new
ThinkThen::Client.open(**settings) do |client|
  records = ->(offset) { count.times.lazy.map { |i| {"id" => "record-#{offset + i}", "body" => "record #{offset + i}"} } }
  if test.fetch("cancel")
    rows = client.decide_each("Ready?", records.call(0), evidence: "/body")
    worker = Thread.new do
      begin
        rows.each { raise "held stream published a row" }
        raise "held stream completed"
      rescue ThinkThen::Error => error
        raise unless error.code.to_s == "local.cancelled"
      ensure
        rows.close
      end
    end
    raise "missing cancellation event" unless STDIN.gets&.strip == "cancel"
    rows.cancel
    worker.value
  else
    test.fetch("rounds").times do |round|
      workers = test.fetch("concurrency").times.map do |index|
        Thread.new do
          offset = test.fetch("identifier_base") + (round * test.fetch("concurrency") + index) * count
          received = failures = 0
          client.decide_each("Ready?", records.call(offset), evidence: "/body") do |row|
            widths[row.id.length] = true
            received += 1
            raise "position or ID changed" unless row.position == received && row.id == "record-#{offset + received - 1}"
            value = row.to_h
            if value.fetch("status") == "failed"
              failures += 1
              raise "wrong row failure" unless value.fetch("error").fetch("code") == test.fetch("scenario").fetch("code")
            else
              raise "wrong answer" unless value.fetch("status") == "answered" && value.fetch("answer") == true && value.fetch("probability") == test.fetch("probability")
            end
          end
          raise "missing rows" unless received == count
          mutex.synchronize { failed += failures }
        end
      end
      workers.each(&:value)
      if (hook = ENV["THINKTHEN_STRESS_CHECKPOINT_COMMAND"])
        raise "resource checkpoint failed" unless system(hook)
      end
    end
    raise "wrong failed count" unless test.fetch("scenario").fetch("status") == "failed" ? failed.positive? : failed.zero?
  end
end
puts JSON.generate(identifier_width_mask: widths.keys.sum { |width| 1 << width }, calls: test.fetch("cancel") ? 1 : test.fetch("concurrency") * test.fetch("rounds"), rows: test.fetch("cancel") ? 0 : count * test.fetch("concurrency") * test.fetch("rounds"), failed: failed, cancelled: test.fetch("cancel"))
