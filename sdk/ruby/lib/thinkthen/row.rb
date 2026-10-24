# frozen_string_literal: true

module ThinkThen
  class Row
    attr_reader :position

    def initialize(native)
      @fields = Native.parse(Native.copy(native[:fields])).freeze
      @detail = Native.copy(native[:detail]).freeze unless native[:detail][:data].null?
      @position = native[:position]
      @kept = native[:kept] != 0
      freeze
    end

    def id = @fields["id"]
    def status = @fields["status"]
    def answer = @fields["answer"]
    def p = @fields["p"]
    def error = @fields["error"]
    def kept? = @kept
    def detail = @detail && Native.parse(@detail)

    def to_h
      result = @fields.dup
      result["detail"] = detail if @detail
      result
    end
  end
end
