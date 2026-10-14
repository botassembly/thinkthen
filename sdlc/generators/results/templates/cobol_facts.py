"""Project every native diagnostic field into owned COBOL records."""
import re


def render(facts, roots=None, prefix="Answer"):
    records = facts['records']
    roots = roots or ('thinkthen_complete_facts_v1', 'thinkthen_complete_immediate_facts_v1', 'thinkthen_complete_error_v1')
    selected, pending = {}, list(roots)
    special = {'thinkthen_complete_utf8_v1': 'thinkthen_cobol_AnswerText', 'thinkthen_complete_json_v1': 'tt_value'}
    def name(native):
        if native in special:
            return special[native]
        words = native.removeprefix('thinkthen_complete_').removesuffix('_v1').split('_')
        return 'thinkthen_cobol_' + prefix + ''.join(word.title() for word in words)
    def base(kind):
        return re.sub(r'\b(const|struct|union)\b|\*', '', kind).strip()
    while pending:
        key = pending.pop()
        if key in selected or key in special:
            continue
        fields = {k: v for k, v in records[key]['fields'].items() if '.' not in k and not (key == 'thinkthen_complete_extension_v1' and k == 'json')}
        for member, field in fields.items():
            if field['type'] == 'union':
                union = next(name for name, owners in facts['union_owners'].items() if (key, member) in [tuple(owner) for owner in owners])
                fields[member] = {**field, 'type': 'union ' + union}
        selected[key] = fields
        for field in fields.values():
            nested = base(field['type'])
            if nested in records:
                pending.append(nested)
    ordered, pending = [], list(selected)
    while pending:
        key = pending.pop(0)
        dependencies = [base(field['type']) for field in selected[key].values() if '*' not in field['type'] and base(field['type']) in selected]
        if all(dependency in ordered for dependency in dependencies):
            ordered.append(key)
        else:
            pending.append(key)
    declarations = ['typedef struct ' + name(key) + ' ' + name(key) + ';' for key in selected]
    bodies = []
    for key in ordered:
        fields = selected[key]
        output = ['struct ' + name(key) + ' {']
        union = key in facts['union_owners']
        extra = ',uint32_t kind' if union else ''
        clone = [f'static int copy_{name(key)}(arena *a,const {key} *input,{name(key)} *out{extra}) {{', 'int code=0;', '(void)a;']
        for member, field in fields.items():
            kind = field['type']; nested = base(kind)
            if union:
                constant = key.removesuffix('_data_v1').upper() + '_' + member.upper() + '_V1'
                if constant not in facts['constants']:
                    raise ValueError('unknown diagnostic union arm: ' + constant)
                clone.append('if(kind==' + constant + ') {')
            if nested in records:
                typ = name(nested)
                output.append(('const ' + typ + ' *' if '*' in kind else typ + ' ') + member + ';')
                if member == 'data' and 'len' in fields:
                    output[-1] = 'const ' + typ + ' *data;'
                    clone += [f'{typ} *items=allocate(a,input->len,sizeof(*items));', 'out->data=items; out->len=input->len;', 'if(!items) return a->code;', 'for(size_t i=0;i<input->len && !code;i++) {']
                    source = 'input->data[i]' if kind.count('*') == 2 else '&input->data[i]'
                    if nested == 'thinkthen_complete_utf8_v1':
                        clone.append(f'code=answer_text(*({source}),items+i);')
                    elif nested == 'thinkthen_complete_json_v1':
                        clone.append(f'items[i].value=copy_value(a,{source}); code=a->code;')
                    else:
                        clone.append(f'code=copy_{typ}(a,{source},items+i);')
                    clone.append('}')
                elif nested == 'thinkthen_complete_utf8_v1':
                    clone.append(f'if(!code) code=answer_text(input->{member},&out->{member});')
                elif nested == 'thinkthen_complete_json_v1':
                    clone.append(f'if(input->{member}) out->{member}=answer_original(a,input->{member});')
                elif '*' in kind:
                    clone += [f'if(input->{member}) {{', f'{typ} *value=allocate(a,1,sizeof(*value));', 'if(!value) return a->code;', f'if(!code) code=copy_{typ}(a,input->{member},value); out->{member}=value;', '}']
                else:
                    suffix = ',input->kind' if nested in facts['union_owners'] else ''
                    clone.append(f'if(!code) code=copy_{typ}(a,&input->{member},&out->{member}{suffix});')
            else:
                output.append(kind + ' ' + member + ';')
                clone.append(f'out->{member}=input->{member};')
            if union:
                clone.append('}')
        output.append('};'); declarations.extend(output)
        clone += ['return code ? code : a->code;', '}']; bodies.extend(clone)
    prototypes = [f'static int copy_{name(key)}(arena *,const {key} *,{name(key)} *' + (',uint32_t);' if key in facts['union_owners'] else ');') for key in selected]
    return '\n'.join(declarations), '\n'.join(prototypes + bodies)
