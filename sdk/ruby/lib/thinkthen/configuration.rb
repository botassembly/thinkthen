# frozen_string_literal: true

module ThinkThen
  class RedactedKey
    def initialize(value)
      raise TypeError, "api_key must be a String" unless value.is_a?(String)
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
    attr_accessor :backend, :model, :url, :batch, :retry
    def initialize
      @backend = @model = @url = @batch = @retry = nil
    end
    def api_key=(value)
      @api_key = value.nil? || value.is_a?(RedactedKey) ? value : RedactedKey.new(value)
    end
    def inspect = "#<ThinkThen::Configuration>"
    alias to_s inspect
    def to_h
      {backend: backend, model: model, url: url, api_key: api_key, batch: batch, retry: self.retry}
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
      @configuration_mutex.synchronize { @shared_client&.close }
      nil
    end
    def decide(question, record, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil) = client.public_send(:decide, question, record, threshold: threshold, evidence: evidence, name: name, cancel: cancel)
    def decide_each(question, records, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block) = client.public_send(:decide_each, question, records, threshold: threshold, evidence: evidence, name: name, cancel: cancel, &block)
    def choose(question, record, choices: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil) = client.public_send(:choose, question, record, choices: choices, threshold: threshold, evidence: evidence, name: name, cancel: cancel)
    def choose_each(question, records, choices: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block) = client.public_send(:choose_each, question, records, choices: choices, threshold: threshold, evidence: evidence, name: name, cancel: cancel, &block)
    def tag(question, record, labels: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil) = client.public_send(:tag, question, record, labels: labels, threshold: threshold, evidence: evidence, name: name, cancel: cancel)
    def tag_each(question, records, labels: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block) = client.public_send(:tag_each, question, records, labels: labels, threshold: threshold, evidence: evidence, name: name, cancel: cancel, &block)
    def score(question, record, levels: UNSET, evidence: UNSET, name: nil, cancel: nil) = client.public_send(:score, question, record, levels: levels, evidence: evidence, name: name, cancel: cancel)
    def score_each(question, records, levels: UNSET, evidence: UNSET, name: nil, cancel: nil, &block) = client.public_send(:score_each, question, records, levels: levels, evidence: evidence, name: name, cancel: cancel, &block)
    def filter(question, record, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil) = client.public_send(:filter, question, record, threshold: threshold, evidence: evidence, name: name, cancel: cancel)
    def filter_each(question, records, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block) = client.public_send(:filter_each, question, records, threshold: threshold, evidence: evidence, name: name, cancel: cancel, &block)
    def rank(question, record, levels: UNSET, top: UNSET, evidence: UNSET, name: nil, cancel: nil) = client.public_send(:rank, question, record, levels: levels, top: top, evidence: evidence, name: name, cancel: cancel)
    def rank_each(question, records, levels: UNSET, top: UNSET, evidence: UNSET, name: nil, cancel: nil, &block) = client.public_send(:rank_each, question, records, levels: levels, top: top, evidence: evidence, name: name, cancel: cancel, &block)
    def find(question, record, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil) = client.public_send(:find, question, record, threshold: threshold, evidence: evidence, name: name, cancel: cancel)
    def find_each(question, records, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block) = client.public_send(:find_each, question, records, threshold: threshold, evidence: evidence, name: name, cancel: cancel, &block)
    def recognize(question_or_record, record = UNSET, kinds: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil) = client.public_send(:recognize, question_or_record, record, kinds: kinds, threshold: threshold, evidence: evidence, name: name, cancel: cancel)
    def recognize_each(question_or_records, records = UNSET, kinds: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block) = client.public_send(:recognize_each, question_or_records, records, kinds: kinds, threshold: threshold, evidence: evidence, name: name, cancel: cancel, &block)
    def relate(question_or_record, record = UNSET, rules: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil) = client.public_send(:relate, question_or_record, record, rules: rules, threshold: threshold, evidence: evidence, name: name, cancel: cancel)
    def relate_each(question_or_records, records = UNSET, rules: UNSET, threshold: UNSET, evidence: UNSET, name: nil, cancel: nil, &block) = client.public_send(:relate_each, question_or_records, records, rules: rules, threshold: threshold, evidence: evidence, name: name, cancel: cancel, &block)
    def annotate(question, record, evidence: UNSET, name: nil, cancel: nil) = client.public_send(:annotate, question, record, evidence: evidence, name: name, cancel: cancel)
    def annotate_each(question, records, evidence: UNSET, name: nil, cancel: nil, &block) = client.public_send(:annotate_each, question, records, evidence: evidence, name: name, cancel: cancel, &block)
  end
end
