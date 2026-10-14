# Lex source only. Literal and interpolation tokens cannot declare privacy.
require 'ripper'
require 'json'

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
# Ripper supplies lexical nesting independently of indentation or repeated names.
def constant_path(node)
  return nil unless node.is_a?(Array)
  case node[0]
  when :@const then node[1]
  when :const_ref, :var_ref, :var_field then constant_path(node[1])
  when :top_const_ref then '::' + constant_path(node[1]).to_s
  when :const_path_ref, :const_path_field
    [constant_path(node[1]), constant_path(node[2])].join('::')
  end
end

def lexical_owners(node, owner, owners)
  return unless node.is_a?(Array)
  return if %i[string_literal xstring_literal regexp_literal dyna_symbol].include?(node[0])
  if node[0].to_s.start_with?('@')
    owners[node[2][0]] = owner
  elsif node[0] == :assign && node[2].is_a?(Array) && node[2][0] == :method_add_block && constant_path(node[1])
    lexical_owners(node[1], owner, owners)
    lexical_owners(node[2][1], owner, owners)
    nested = [owner, constant_path(node[1])].reject(&:empty?).join('::')
    lexical_owners(node[2][2], nested, owners)
  elsif %i[class module].include?(node[0])
    lexical_owners(node[1], owner, owners)
    name = constant_path(node[1])
    nested = name.start_with?('::') ? name.delete_prefix('::') : [owner, name].reject(&:empty?).join('::')
    lexical_owners(node[2], owner, owners) if node[0] == :class
    lexical_owners(node[-1], nested, owners)
  else
    node.each { |child| lexical_owners(child, owner, owners) if child.is_a?(Array) }
  end
end
owners = {}
lexical_owners(Ripper.sexp(source), '', owners)
STDOUT.write(JSON.generate(masked: output.join, owners: owners))
