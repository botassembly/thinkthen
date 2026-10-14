# Lex source only. Literal and interpolation tokens cannot declare privacy.
require 'ripper'

source = STDIN.read
source.force_encoding(Encoding::UTF_8)
characters = source.each_char.to_a
output = characters.map { |character| character == "\n" ? "\n" : ' ' }
byte_positions = {}
byte_offset = 0
characters.each_with_index do |character, index|
  byte_positions[byte_offset] = index
  byte_offset += character.bytesize
end
offsets = [0]
source.each_line { |line| offsets << offsets.last + line.bytesize }
masked = %i[on_comment on_embdoc_beg on_embdoc on_embdoc_end on_tstring_beg
            on_tstring_content on_tstring_end on_regexp_beg on_regexp_end
            on_words_beg on_qwords_beg on_symbols_beg on_qsymbols_beg
            on_words_sep on_backtick on_heredoc_beg on_heredoc_end
            on_label_end on___end__]
interpolation = 0
variable = false
Ripper.lex(source, raise_errors: true).each do |position, event, token, _state|
  interpolation += 1 if event == :on_embexpr_beg
  hidden = interpolation.positive? || variable || masked.include?(event)
  hidden ||= event == :on_symbeg && token != ':'
  hidden ||= event == :on_embvar
  unless hidden
    start = offsets.fetch(position[0] - 1) + position[1]
    output[byte_positions.fetch(start), token.length] = token.each_char.to_a
  end
  interpolation -= 1 if event == :on_embexpr_end
  variable = event == :on_embvar
end
STDOUT.write(output.join)
