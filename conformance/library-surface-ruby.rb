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

# Method identity includes its Ruby lookup domain. Defaults belong to a body,
# while named visibility changes follow the owner across reopened bodies.
def method_symbols(arguments)
  return [] unless arguments.is_a?(Array) && arguments[0] == :args_add_block
  arguments[1].filter_map do |argument|
    argument[1][1][1] if argument[0] == :symbol_literal && argument[1][0] == :symbol
  end
end

def keyword_names(node)
  return [] unless node.is_a?(Array)
  return [node[1].delete_suffix(':')] if node[0] == :@label
  node.flat_map { |child| child.is_a?(Array) ? keyword_names(child) : [] }
end

def source_origin(node)
  return nil unless node.is_a?(Array)
  return node[2] if node[0].to_s.start_with?('@')
  node.each do |child|
    origin = source_origin(child)
    return origin if origin
  end
  nil
end

def singleton_domain(domain)
  domain == 'instance' ? 'singleton' : domain + '_singleton'
end

def method_inventory(node, owner, domain, body, methods, owners, directives, executed = true, lexical = owner)
  return unless node.is_a?(Array)
  return if %i[string_literal xstring_literal regexp_literal dyna_symbol].include?(node[0])
  constant_privacy(node, lexical, directives) if executed
  if node[0].to_s.start_with?('@')
    owners[node[2][0]] = lexical
  elsif node[0].is_a?(Array) || node[0] == :program
    node.each do |child|
      method_inventory(child, owner, domain, body, methods, owners, directives, executed, lexical) if child.is_a?(Array)
    end
  elsif node[0] == :bodystmt
    method_inventory(node[1], owner, domain, body, methods, owners, directives, executed, lexical)
    node.drop(2).each { |child| method_inventory(child, owner, domain, body, methods, owners, directives, false, lexical) }
  elsif %i[class module].include?(node[0])
    name = constant_path(node[1])
    method_inventory(node[1], owner, domain, body, methods, owners, directives, false, lexical)
    nested = name.start_with?('::') ? name.delete_prefix('::') : [lexical, name].reject(&:empty?).join('::')
    method_inventory(node[-1], nested, 'instance', { visibility: 'public', origin: source_origin(node[1]) }, methods, owners, directives, executed)
  elsif node[0] == :sclass
    self_receiver = node[1][0] == :var_ref && node[1][1][0..1] == [:@kw, 'self']
    nested_domain = singleton_domain(domain)
    method_inventory(node[2], owner, nested_domain, { visibility: 'public', origin: source_origin(node[1]) }, methods, owners, directives,
                     executed && self_receiver, lexical + '::<singleton>')
  elsif node[0] == :assign && node[2].is_a?(Array) && node[2][0] == :method_add_block &&
        constant_path(node[1]) && builder_block?(node[2])
    method_inventory(node[1], owner, domain, body, methods, owners, directives, false, lexical)
    nested = [lexical, constant_path(node[1])].reject(&:empty?).join('::')
    method_inventory(node[2][2], nested, 'instance', { visibility: 'public', origin: source_origin(node[1]) }, methods, owners, directives, executed)
  elsif %i[def defs].include?(node[0])
    singleton = node[0] == :defs
    token, params = singleton ? [node[3], node[4]] : [node[1], node[2]]
    target_domain = singleton ? (singleton_domain(domain)) : domain
    mode = executed && !singleton ? body[:visibility] : 'public'
    owners[token[2][0]] = lexical
    identity = [owner, target_domain, token[1]]
    methods[identity] = { owner: owner, domain: target_domain, body: body[:origin],
                          name: token[1], kind: 'call', visibility: mode, keywords: keyword_names(params) }
    method_inventory(node[-1], owner, domain, body, methods, owners, directives, false, lexical)
  elsif node[0] == :vcall && %w[private protected public].include?(node[1][1])
    owners[node[1][2][0]] = lexical
    body[:visibility] = node[1][1] if executed
  elsif node[0] == :command && %w[private protected public private_class_method public_class_method].include?(node[1][1])
    owners[node[1][2][0]] = lexical
    mode = node[1][1]
    arguments = node[2][0] == :args_add_block ? node[2][1] : node[2]
    method_inventory(arguments, owner, domain, body, methods, owners, directives, executed, lexical)
    if executed
      target_domain = mode.end_with?('_class_method') ? singleton_domain(domain) : domain
      names = method_symbols(node[2])
      arguments.each do |argument|
        names << argument[argument[0] == :defs ? 3 : 1][1] if %i[def defs].include?(argument[0])
        names.concat(method_symbols(argument[2])) if argument[0] == :command && argument[1][1] == 'attr_reader'
      end
      names.each do |name|
        entry = methods[[owner, target_domain, name]]
        entry[:visibility] = mode.start_with?('public') ? 'public' : 'private' if entry
      end
    end
  elsif node[0] == :command && node[1][1] == 'attr_reader'
    owners[node[1][2][0]] = lexical
    method_symbols(node[2]).each do |name|
      methods[[owner, domain, name]] = { owner: owner, domain: domain, body: body[:origin],
                                       name: name, kind: 'field', visibility: executed ? body[:visibility] : 'public',
                                       keywords: [] }
    end
  elsif %i[do_block brace_block].include?(node[0])
    method_inventory(node[2], owner, domain, body, methods, owners, directives, executed, lexical)
  else
    node.each do |child|
      method_inventory(child, owner, domain, body, methods, owners, directives, false, lexical) if child.is_a?(Array)
    end
  end
end

owners, directives, methods = {}, [], {}
method_inventory(Ripper.sexp(source), '', 'instance', { visibility: 'public', origin: [1, 0] }, methods, owners, directives)
STDOUT.write(JSON.generate(masked: output.join, owners: owners, private_constants: directives, methods: methods.values))
