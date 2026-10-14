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

# Only declarative class builders run their block with the assigned constant as self.
def builder_block?(node)
  call = node[1]
  call = call[1] if call[0] == :method_add_arg
  call[0] == :call && %w[Class Module Struct].include?(constant_path(call[1])) && call[3][1] == 'new'
end

def constant_privacy(node, owner, directives)
  if node[0] == :command && node[1][1] == 'private_constant'
    receiver, arguments = nil, node[2]
  elsif node[0] == :command_call && node[3][1] == 'private_constant'
    receiver = constant_path(node[1])
    receiver = 'self' if node[1][0] == :var_ref && node[1][1][0..1] == [:@kw, 'self']
    return unless receiver
    arguments = node[4]
  else
    return
  end
  return unless arguments[0] == :args_add_block
  arguments[1].each do |argument|
    next unless argument[0] == :symbol_literal && argument[1][0] == :symbol
    symbol = argument[1][1]
    directives << [owner, receiver, symbol[1]] if %i[@const @ident].include?(symbol[0])
  end
end

def method_privacy(node, owner, visibility)
  if node[0] == :vcall && %w[private public].include?(node[1][1])
    visibility[node[1][2][0]] = [owner, node[1][1], []]
  elsif node[0] == :command && node[1][1] == 'private_class_method'
    symbols = node[2][1].filter_map do |argument|
      argument[1][1][1] if argument[0] == :symbol_literal && argument[1][0] == :symbol
    end
    visibility[node[1][2][0]] = [owner, 'private_class_method', symbols]
  end
end

def lexical_owners(node, owner, owners, directives, visibility, executed = true)
  return unless node.is_a?(Array)
  return if %i[string_literal xstring_literal regexp_literal dyna_symbol].include?(node[0])
  constant_privacy(node, owner, directives) if executed
  method_privacy(node, owner, visibility) if executed
  if node[0].to_s.start_with?('@')
    owners[node[2][0]] = owner
  elsif node[0] == :assign && node[2].is_a?(Array) && node[2][0] == :method_add_block &&
        constant_path(node[1]) && builder_block?(node[2])
    lexical_owners(node[1], owner, owners, directives, visibility, executed)
    lexical_owners(node[2][1], owner, owners, directives, visibility, executed)
    nested = [owner, constant_path(node[1])].reject(&:empty?).join('::')
    lexical_owners(node[2][2], nested, owners, directives, visibility, executed)
  elsif %i[class module].include?(node[0])
    lexical_owners(node[1], owner, owners, directives, visibility, executed)
    name = constant_path(node[1])
    nested = name.start_with?('::') ? name.delete_prefix('::') : [owner, name].reject(&:empty?).join('::')
    lexical_owners(node[2], owner, owners, directives, visibility, executed) if node[0] == :class
    lexical_owners(node[-1], nested, owners, directives, visibility, executed)
  elsif node[0] == :sclass
    lexical_owners(node[1], owner, owners, directives, visibility, executed)
    lexical_owners(node[2], owner + '::<singleton>', owners, directives, visibility, executed)
  elsif node[0] == :method_add_block
    lexical_owners(node[1], owner, owners, directives, visibility, executed)
    lexical_owners(node[2], owner, owners, directives, visibility, false)
  else
    executed = false if %i[def defs lambda].include?(node[0])
    node.each { |child| lexical_owners(child, owner, owners, directives, visibility, executed) if child.is_a?(Array) }
  end
end
owners, directives, visibility = {}, [], {}
lexical_owners(Ripper.sexp(source), '', owners, directives, visibility)
STDOUT.write(JSON.generate(masked: output.join, owners: owners, private_constants: directives, visibility: visibility))
