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
    return '\n'.join(lines) + '\n'
