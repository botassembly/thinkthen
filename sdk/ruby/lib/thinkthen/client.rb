# frozen_string_literal: true

module ThinkThen
  class Client
    def self.open(**settings)
      client = new(**settings)
      begin
        yield client
      ensure
        client.close
      end
    end

    def initialize(**settings)
      @mutex = Mutex.new
      Thread.handle_interrupt(Exception => :never) do
        Native.bytes(Native.dump(settings)) do |bytes, length|
          output = FFI::MemoryPointer.new(:pointer)
          Native.checked { |error| Native.thinkthen_client_open(bytes, length, output, error) }
          @pointer = Native::ClientPointer.new(output.read_pointer)
        end
      end
    rescue Exception
      close
      raise
    end

    def close
      Thread.handle_interrupt(Exception => :never) do
        @mutex.synchronize do
          @pointer&.free
          @pointer = nil
        end
      end
      nil
    end

    def closed?
      @mutex.synchronize { !@pointer }
    end

    def decide(question, records, **options, &block) = call("decide", question, records, options, &block)
    def choose(question, records, **options, &block) = call("choose", question, records, options, &block)
    def tag(question, records, **options, &block) = call("tag", question, records, options, &block)
    def score(question, records, **options, &block) = call("score", question, records, options, &block)
    def filter(question, records, **options, &block) = call("filter", question, records, options, &block)
    def rank(question, records, **options, &block) = call("rank", question, records, options, &block)
    def find(question, records, **options, &block) = call("find", question, records, options, &block)
    def annotate(question, records, **options, &block) = call("annotate", question, records, options, &block)
    def recognize(question, records, **options, &block) = call("recognize", question, records, options, &block)
    def relate(question, records, **options, &block) = call("relate", question, records, options, &block)

    private

    def call(function, question, records, options)
      cancel = options.delete(:cancel)
      tagged = question.is_a?(Question) ? question.tagged : {type: "inline", value: question}
      request = Native.dump({function: function, question: tagged, options: options})
      rows = nil
      Thread.handle_interrupt(Exception => :never) do
        Native.bytes(request) do |bytes, length|
          @mutex.synchronize do
            raise IOError, "client is closed" unless @pointer
            output = FFI::MemoryPointer.new(:pointer)
            Native.checked do |error|
              Native.thinkthen_call_open(@pointer, bytes, length, 2, output, error)
            end
            rows = Rows.new(output.read_pointer, records, cancel)
          end
        end
      end
      return rows unless block_given?
      rows.each { |row| yield row }
    rescue Exception
      rows&.close
      raise
    end
  end
end
