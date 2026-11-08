# frozen_string_literal: true
require "json"
require "thinkthen"
# Feature: Ruby callers retain positions in independent streams on one client.

# glibc's bytes in use for each soak checkpoint, which RSS cannot separate from the free
# storage glibc keeps: small blocks in use plus the large blocks glibc maps on their own.
# Its mapped storage, arena plus those blocks, bounds what glibc can be holding.
# glibc 2.28 has mallinfo, not mallinfo2; its int fields fit this soak.
module AllocatorInUse
  extend FFI::Library
  ffi_lib FFI::Library::LIBC
  class Info < FFI::Struct
    layout(*%i[arena ordblks smblks hblks hblkhd usmblks fsmblks uordblks fordblks keepcost].flat_map { |name| [name, :int] })
  end
  begin
    attach_function :mallinfo, [], Info.by_value
  rescue FFI::NotFoundError
    nil
  end
  def self.bytes
    return nil unless respond_to?(:mallinfo)
    info = mallinfo
    field = ->(name) { info[name] & 0xffffffff }
    [field.(:uordblks) + field.(:hblkhd), field.(:arena) + field.(:hblkhd)]
  end
end

# Ruby's live object slots, whose heap pages Ruby maps outside malloc. Each live slot
# counts at the smallest slot size, so a retained object adds at least that much.
module RuntimeHeap
  SLOT = GC::INTERNAL_CONSTANTS.fetch(:BASE_SLOT_SIZE) { GC::INTERNAL_CONSTANTS.fetch(:RVALUE_SIZE) }
  def self.bytes = GC.stat(:heap_live_slots) * SLOT
end
test = JSON.parse(File.read(ENV.fetch("THINKTHEN_STRESS_CASE")))
settings = test.fetch("client").transform_keys(&:to_sym)
settings[:backend] = settings.fetch(:backend).to_sym
policy = settings.fetch(:retry).transform_keys(&:to_sym)
policy[:initial_backoff] = policy.delete(:initial_backoff_ms) / 1000.0 if policy.key?(:initial_backoff_ms)
policy[:attempt_timeout] = policy.delete(:attempt_timeout_ms) / 1000.0 if policy.key?(:attempt_timeout_ms)
settings[:retry] = ThinkThen::Retry.new(**policy)
count = test.fetch("records")
widths = {}
failed = 0
pulled = 0
mutex = Mutex.new
ThinkThen::Client.open(**settings) do |client|
  records = ->(offset) { count.times.lazy.map { |i| {"id" => "record-#{offset + i}", "body" => "record #{offset + i}"} } }
  if test.fetch("cancel")
    held_records = count.times.lazy.map do |i|
      pulled += 1
      raise "input read ahead exceeded the engine bound" if test["read_ahead_limit"] && pulled > test["read_ahead_limit"]
      {"id" => "record-#{i}", "body" => "record #{i}"}
    end
    rows = client.decide_each("Ready?", held_records, field: "/body")
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
          client.decide_each("Ready?", records.call(offset), field: "/body") do |row|
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
        # A full collection first, so both series count what is still reachable, not garbage
        # waiting for the collector.
        GC.start
        in_use, mapped = AllocatorInUse.bytes
        ENV["THINKTHEN_STRESS_ALLOCATOR_IN_USE"] = in_use.to_s if in_use
        ENV["THINKTHEN_STRESS_ALLOCATOR_MAPPED"] = mapped.to_s if mapped
        ENV["THINKTHEN_STRESS_RUNTIME_HEAP"] = RuntimeHeap.bytes.to_s
        raise "resource checkpoint failed" unless system(hook)
      end
    end
    raise "wrong failed count" unless test.fetch("scenario").fetch("status") == "failed" ? failed.positive? : failed.zero?
  end
end
puts JSON.generate(read_ahead_records: pulled, identifier_width_mask: widths.keys.sum { |width| 1 << width }, calls: test.fetch("cancel") ? 1 : test.fetch("concurrency") * test.fetch("rounds"), rows: test.fetch("cancel") ? 0 : count * test.fetch("concurrency") * test.fetch("rounds"), failed: failed, cancelled: test.fetch("cancel"))
