# frozen_string_literal: true

require "digest"
require "json"

# The question keys of ADR 0111 section 2, recomputed from one request body:
# the SHA-256 of the adapter, the URL, the model, the state and one question
# as the body carries them, joined by line feeds. The raw member bytes are
# read from the compact body, so no re-encoding can move a byte.
module QuestionKeys
  module_function

  # Each member of the compact JSON object at `at` as its key and raw text.
  def members(text, at = 0)
    found = []
    i = at + 1
    while text[i] != "}"
      key_end = value_end(text, i)
      close = value_end(text, key_end + 1)
      found << [JSON.parse(text[i...key_end]), text[(key_end + 1)...close]]
      i = text[close] == "," ? close + 1 : close
    end
    found
  end

  # The end of the JSON value that starts at `at` in compact `text`.
  def value_end(text, at)
    depth = 0
    i = at
    while i < text.length
      char = text[i]
      if char == '"'
        i += 1
        i += text[i] == "\\" ? 2 : 1 while text[i] != '"'
        return i + 1 if depth.zero?
      elsif "{[".include?(char)
        depth += 1
      elsif "}]".include?(char)
        depth -= 1
        return i + 1 if depth.zero?
      elsif depth.zero? && char == ","
        return i
      end
      i += 1
    end
    text.length
  end

  def of(url, body)
    parts = members(body).to_h
    head = ["systemone", url, parts.fetch("model"), parts.fetch("state")].join("\n")
    members(parts.fetch("questions"))
      .sort_by { |name, _| Integer(name[1..]) }
      .map { |_, question| Digest::SHA256.hexdigest("#{head}\n#{question}") }
  end
end
