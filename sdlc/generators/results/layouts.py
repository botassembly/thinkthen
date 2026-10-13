"""Primitive ABI layouts shared by request carriers and result projections.

Only transport shapes belong here. Native JSON copying and backing lifetimes
remain handwritten; source mode also derives the public C header from these
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
    counted('thinkthen_complete_utf8_v1', 'std::ffi::c_char'),
    Layout('thinkthen_complete_extension_v1', (('name', 'thinkthen_complete_utf8_v1'), ('json', 'thinkthen_complete_utf8_v1'))),
    counted('thinkthen_complete_extensions_v1', 'thinkthen_complete_extension_v1'),
    Layout('thinkthen_complete_json_entry_v1', (('name', 'thinkthen_complete_utf8_v1'), ('value', '*const thinkthen_complete_json_v1'))),
    counted('thinkthen_complete_json_array_v1', '*const thinkthen_complete_json_v1'),
    counted('thinkthen_complete_json_object_v1', 'thinkthen_complete_json_entry_v1'),
    Layout('thinkthen_complete_json_data_v1', (
        ('boolean', 'u32'), ('number', 'thinkthen_complete_utf8_v1'), ('string', 'thinkthen_complete_utf8_v1'),
        ('array', 'thinkthen_complete_json_array_v1'), ('object', 'thinkthen_complete_json_object_v1')),
        union=True, debug_label='JSON payload'),
    Layout('thinkthen_complete_json_v1', (('kind', 'u32'), ('data', 'thinkthen_complete_json_data_v1'))),
)
# Numbers are fixed ABI assignments; JSON kinds follow the native schema grammar.
JSON_KINDS = ('null', 'boolean', 'number', 'string', 'array', 'object')
AUTHOR = (
    Layout('StringV1', counted('StringV1', 'std::ffi::c_char').fields, default=True),
    Layout('StringsV1', counted('StringsV1', 'StringV1').fields, default=True),
    optional('OptionalStringV1', 'StringV1'),
    optional('OptionalU64V1', 'u64'),
    Layout('InputPropertyV1', (('name', 'StringV1'), ('kind', 'u32')), default=True),
    Layout('InputPropertiesV1', counted('InputPropertiesV1', 'InputPropertyV1').fields, default=True),
    Layout('InputDeclarationV1', (('kind', 'u32'), ('properties', 'InputPropertiesV1'), ('required', 'StringsV1')), default=True),
    Layout('QuestionAuthorV1', (('name', 'OptionalStringV1'), ('wording_version', 'OptionalU64V1'),
        ('item_schema', 'InputDeclarationV1'), ('context_schema', 'InputDeclarationV1')), default=True),
)
