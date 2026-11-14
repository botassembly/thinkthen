# frozen_string_literal: true

module ThinkThen
  class RedactedKey
    def initialize(value)
      raise UsageError.new("api_key must be a String", code: :"usage.settings") unless value.is_a?(String)
      ThinkThen.send(:native)
      @settings = Native.settings
      Native.bytes(value) do |bytes, length|
        Native.checked { |error| Native.thinkthen_settings_set_api_key(@settings, bytes, length, error) }
      end
      freeze
    end
    def inspect = "#<ThinkThen::RedactedKey [REDACTED]>"
    alias to_s inspect
    def copy_to(settings)
      Native.checked { |error| Native.thinkthen_settings_copy_api_key(settings, @settings, error) }
    end
    private :copy_to
  end
  class Configuration
    attr_reader :api_key
    attr_accessor :backend, :model, :url, :batch, :max_requests, :retry
    def initialize
      @backend = @model = @url = @batch = @max_requests = @retry = nil
    end
    def api_key=(value)
      @api_key = value.nil? || value.is_a?(RedactedKey) ? value : RedactedKey.new(value)
    end
    def inspect = "#<ThinkThen::Configuration>"
    alias to_s inspect
    def to_h
      {backend: backend, model: model, url: url, api_key: api_key, batch: batch, max_requests: max_requests, retry: self.retry}
    end
  end
  @configuration_mutex = Mutex.new
  @configuration = Configuration.new
  class << self
    def configure
      @configuration_mutex.synchronize do
        raise IOError, "shared client is already configured" if @shared_client
        candidate = @configuration.dup
        yield candidate
        @configuration = candidate.dup
        @configuration.model = Values.freeze_copy(candidate.model)
        @configuration.url = Values.freeze_copy(candidate.url)
      end
      nil
    end
    def client
      @configuration_mutex.synchronize { @shared_client ||= Client.new(**@configuration.to_h) }
    end
    def close
      @configuration_mutex.synchronize { @shared_client&.close; @shared_client = nil }
      nil
    end
    def decide(question, record, threshold: UNSET, field: UNSET, name: nil, cancel: nil, yes_means: UNSET, no_means: UNSET) = client.public_send(:decide, question, record, threshold: threshold, field: field, name: name, cancel: cancel, yes_means: yes_means, no_means: no_means)
    def decide_each(question, records, threshold: UNSET, field: UNSET, name: nil, cancel: nil, yes_means: UNSET, no_means: UNSET, &block) = client.public_send(:decide_each, question, records, threshold: threshold, field: field, name: name, cancel: cancel, yes_means: yes_means, no_means: no_means, &block)
    def choose(question, record, choices: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET) = client.public_send(:choose, question, record, choices: choices, threshold: threshold, field: field, name: name, cancel: cancel, descriptions: descriptions)
    def choose_each(question, records, choices: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET, &block) = client.public_send(:choose_each, question, records, choices: choices, threshold: threshold, field: field, name: name, cancel: cancel, descriptions: descriptions, &block)
    def tag(question, record, labels: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET) = client.public_send(:tag, question, record, labels: labels, threshold: threshold, field: field, name: name, cancel: cancel, descriptions: descriptions)
    def tag_each(question, records, labels: UNSET, threshold: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET, &block) = client.public_send(:tag_each, question, records, labels: labels, threshold: threshold, field: field, name: name, cancel: cancel, descriptions: descriptions, &block)
    def score(question, record, levels: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET) = client.public_send(:score, question, record, levels: levels, field: field, name: name, cancel: cancel, descriptions: descriptions)
    def score_each(question, records, levels: UNSET, field: UNSET, name: nil, cancel: nil, descriptions: UNSET, &block) = client.public_send(:score_each, question, records, levels: levels, field: field, name: name, cancel: cancel, descriptions: descriptions, &block)
    def filter(question, records, **options, &block) = client.public_send(:filter, question, records, **options, &block)
    def rank(question, records, **options, &block) = client.public_send(:rank, question, records, **options, &block)
    def find(question, record, threshold: UNSET, field: UNSET, name: nil, cancel: nil) = client.public_send(:find, question, record, threshold: threshold, field: field, name: name, cancel: cancel)
    def find_each(question, records, threshold: UNSET, field: UNSET, name: nil, cancel: nil, &block) = client.public_send(:find_each, question, records, threshold: threshold, field: field, name: name, cancel: cancel, &block)
    def recognize(record, **options) = client.public_send(:recognize, record, **options)
    def recognize_each(records, **options, &block) = client.public_send(:recognize_each, records, **options, &block)
    def relate(record, **options) = client.public_send(:relate, record, **options)
    def relate_each(records, **options, &block) = client.public_send(:relate_each, records, **options, &block)
    def annotate(question, record, field: UNSET, name: nil, cancel: nil) = client.public_send(:annotate, question, record, field: field, name: name, cancel: cancel)
    def annotate_each(question, records, field: UNSET, name: nil, cancel: nil, &block) = client.public_send(:annotate_each, question, records, field: field, name: name, cancel: cancel, &block)
  end
end
