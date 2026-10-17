"""Render reusable counted, optional, record and union ABI layouts."""
import json


def render(layouts, kinds=()):
    lines = ['//! Generated primitive ABI transport declarations. Do not edit.',
        '#![allow(non_camel_case_types, missing_docs, reason = "native immutable ABI transport names")]',
        '#![forbid(unsafe_code)]']
    for layout in layouts:
        derive = ['Clone', 'Copy'] + ([] if layout.union else ['Debug']) + (['Default'] if layout.default else [])
        lines.extend(['/// C descriptor or borrowed view `' + layout.name + '`.', '#[repr(C)]',
            '#[derive(' + ', '.join(derive) + ')]',
            'pub ' + ('union' if layout.union else 'struct') + ' ' + layout.name + ' {'])
        for name, kind in layout.fields:
            lines.extend(['    /// C field `' + name + '`.', '    pub ' + name + ': ' + kind + ','])
        lines.append('}')
        if layout.debug_label:
            lines.extend(['impl std::fmt::Debug for ' + layout.name + ' {',
                "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {",
                '        f.write_str(' + json.dumps(layout.debug_label) + ')', '    }', '}'])
    for value, kind in enumerate(kinds, 1):
        lines.append('pub const THINKTHEN_COMPLETE_JSON_' + kind.upper() + '_V1: u32 = ' + str(value) + ';')
    return '\n'.join(lines) + '\n' + (json_projection(kinds) if kinds else '')


def json_projection(kinds):
    """Copy the canonical JSON alternatives into one immutable native backing pool."""
    lines = ['pub(crate) fn read_json(node: &super::Node, store: &mut super::Storage) -> Result<*const thinkthen_complete_json_v1, thinkthen::ErrorKind> {',
             'let (kind, data) = match node.kind() {']
    for kind in kinds:
        symbol = 'THINKTHEN_COMPLETE_JSON_' + kind.upper() + '_V1'
        if kind == 'null':
            payload = 'boolean: 0'
            body = []
        elif kind == 'boolean':
            payload = 'boolean: node.boolean()?'
            body = []
        elif kind in ('number', 'string'):
            payload = kind + ': store.text(node.' + ('raw_text()' if kind == 'number' else 'text()?') + ')'
            body = []
        elif kind in ('array', 'object'):
            body = ['let mut copied = Vec::new();', 'for ' + ('value' if kind == 'array' else '(name, value)') + ' in node.' + kind + '()? {']
            item = 'read_json(value, store)?' if kind == 'array' else 'thinkthen_complete_json_entry_v1 { name: store.text(name), value: read_json(value, store)? }'
            body.extend(['copied.push(' + item + ');', '}', 'let (data, len) = store.slice(copied);'])
            payload = kind + ': thinkthen_complete_json_' + kind + '_v1 { data, len }'
        else:
            raise ValueError('unsupported canonical JSON kind: ' + kind)
        lines.extend([json.dumps(kind) + ' => {', *body,
                      '(' + symbol + ', thinkthen_complete_json_data_v1 { ' + payload + ' })', '},'])
    lines.extend(['_ => return Err(thinkthen::ErrorKind::Defect),', '};',
                  'Ok(store.hold(thinkthen_complete_json_v1 { kind, data }))', '}'])
    return '\n'.join(lines) + '\n'
