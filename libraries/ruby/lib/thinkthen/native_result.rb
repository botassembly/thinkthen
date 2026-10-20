# frozen_string_literal: true
module ThinkThen
  module Native
    class Result
      def initialize(fields)
        @fields = fields.freeze
        freeze
      end
      def [](member) = @fields.fetch(member)
      def key?(member) = @fields.key?(member)
      def keys = @fields.keys.freeze
      def to_h = @fields.transform_values { |value| Results.plain(value) }.freeze
      def inspect = "<#{self.class}: content withheld>"
    end
  end
  module Results
    def self.plain(value)
      case value
      when Native::Result then value.to_h
      when Array then value.map { |item| plain(item) }.freeze
      when Hash then value.transform_values { |item| plain(item) }.freeze
      else value
      end
    end
    def self.convert(kind, value)
      source = SCHEMAS.fetch(kind)
      if source.key?("variants")
        child = source.fetch("variants").find { |_, tag| matches?(tag, value) }
        raise KeyError, "native result has no generated alternative" unless child
        return convert(child.first, value)
      end
      if source.key?("properties")
        fields = value.to_h { |key, item| [key, field(source.fetch("properties").fetch(key, {}), item)] }.freeze
        name = "Native" + kind.delete_prefix("complete").split("_").map { |part| part[0].upcase + part[1..] }.join
        return const_get(name).new(fields)
      end
      source = source.fetch("primitive_schemas").fetch("object") if value.is_a?(Hash) && source.key?("primitive_schemas")
      field(source, value)
    end
    def self.field(schema, value)
      return convert(schema.fetch("$ref").delete_prefix("#/$defs/"), value) if schema.key?("$ref")
      alternatives = schema["anyOf"] || schema["oneOf"]
      refs = alternatives&.select { |child| child.key?("$ref") }
      return field(refs.first, value) if !value.nil? && refs&.length == 1
      if alternatives && !value.nil?
        kind = value.is_a?(Array) ? "array" : value.is_a?(Hash) ? "object" : nil
        selected = alternatives.find { |child| child["type"] == kind } if kind
        return field(selected, value) if selected
      end
      return value.map { |item| field(schema.fetch("items", {}), item) }.freeze if value.is_a?(Array)
      source = schema["additionalProperties"]
      return value.transform_values { |item| field(source, item) }.freeze if value.is_a?(Hash) && source.is_a?(Hash)
      value
    end
    def self.matches?(tag, value)
      mode, member, wanted = tag
      case mode
      when "literal" then value[member] == wanted
      when "member" then value.key?(member)
      when "literals" then wanted.all? { |key, item| value[key] == item }
      when "structure" then wanted.fetch("required").all? { |key| value.key?(key) } && wanted.fetch("excluded").none? { |key| value.key?(key) }
      else false
      end
    end
  end
end
