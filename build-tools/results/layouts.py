"""Primitive ABI layouts shared by request carriers and result projections.

Transport shapes and canonical alternative tags belong here. JSON projection
follows these alternatives; native decoding and backing lifetimes remain handwritten; source mode also derives the public C header from these
same generated records.
"""
from dataclasses import dataclass


@dataclass(frozen=True)
class Layout:
    name: str
    fields: tuple[tuple[str, str], ...]
    union: bool = False
    default: bool = False
    debug_label: str | None = None


def counted(name, element):
    return Layout(name, (('data', '*const ' + element), ('len', 'usize')))


def optional(name, value):
    return Layout(name, (('present', 'std::ffi::c_int'), ('value', value)), default=True)


PRIMITIVES = (
    Layout('thinkthen_error_details', (('code', 'std::ffi::c_int'), ('retryable', 'std::ffi::c_int'),
        ('message', 'thinkthen_complete_utf8'), ('facts', '*const crate::session::views_generated::thinkthen_complete_immediate_facts'))),
    counted('thinkthen_complete_utf8', 'std::ffi::c_char'),
    Layout('thinkthen_complete_extension', (('name', 'thinkthen_complete_utf8'), ('json', 'thinkthen_complete_utf8'), ('value', '*const thinkthen_complete_json'))),
    counted('thinkthen_complete_extensions', 'thinkthen_complete_extension'),
    Layout('thinkthen_complete_json_entry', (('name', 'thinkthen_complete_utf8'), ('value', '*const thinkthen_complete_json'))),
    counted('thinkthen_complete_json_array', '*const thinkthen_complete_json'),
    counted('thinkthen_complete_json_object', 'thinkthen_complete_json_entry'),
    Layout('thinkthen_complete_json_data', (
        ('boolean', 'u32'), ('number', 'thinkthen_complete_utf8'), ('string', 'thinkthen_complete_utf8'),
        ('array', 'thinkthen_complete_json_array'), ('object', 'thinkthen_complete_json_object')),
        union=True, debug_label='JSON payload'),
    Layout('thinkthen_complete_json', (('kind', 'u32'), ('data', 'thinkthen_complete_json_data'))),
)
# Numbers are fixed ABI assignments; JSON kinds follow the native schema grammar.
JSON_KINDS = ('null', 'boolean', 'number', 'string', 'array', 'object')
AUTHOR = (
    Layout('CountedString', counted('CountedString', 'std::ffi::c_char').fields, default=True),
    Layout('Strings', counted('Strings', 'CountedString').fields, default=True),
    optional('OptionalString', 'CountedString'),
    optional('OptionalU64', 'u64'),
    Layout('InputPropertyView', (('name', 'CountedString'), ('kind', 'u32')), default=True),
    Layout('InputProperties', counted('InputProperties', 'InputPropertyView').fields, default=True),
    Layout('InputDeclarationView', (('kind', 'u32'), ('properties', 'InputProperties'), ('required', 'Strings')), default=True),
    Layout('QuestionAuthorInfo', (('name', 'OptionalString'), ('wording_version', 'OptionalU64'),
        ('item_schema', 'InputDeclarationView'), ('context_schema', 'InputDeclarationView')), default=True),
)
