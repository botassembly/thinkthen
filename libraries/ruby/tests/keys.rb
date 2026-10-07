# frozen_string_literal: true

require "digest"
require "json"

# Independent v2 question keys retain raw member bytes and authored reported models.
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

  def of(url, body, reported_model)
    parts = members(body).to_h
    head = ["systemone", url, parts.fetch("model"), JSON.generate(reported_model), parts.fetch("state")]
    members(parts.fetch("questions"))
      .sort_by { |name, _| Integer(name[1..]) }
      .map do |_, question|
        framed = "thinkthen.question-key/2\0".b
        [*head, question].each do |part|
          bytes = part.encode(Encoding::UTF_8).b
          framed << [bytes.bytesize].pack("Q>") << bytes
        end
        Digest::SHA256.hexdigest(framed)
      end
  end
end
