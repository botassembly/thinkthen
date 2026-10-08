"""Compare copied PHP declarations and actual FFI layouts with the live C header."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
spec = importlib.util.spec_from_file_location('c_abi', ROOT / 'sdlc/scripts/check-c-exports.py')
abi = importlib.util.module_from_spec(spec)
spec.loader.exec_module(abi)


def copied_abi(copy, scratch):
    header = scratch / 'php-copy.h'
    # PHP's built-in size_t/uintN_t declarations come from the target C headers.
    header.write_text('#include <stddef.h>\n#include <stdint.h>\n' + copy.read_text())
    return abi.header_abi(header)


def ffi_layouts(copy, library, records, scratch):
    script = scratch / 'layouts.php'
    script.write_text('''<?php
$f = FFI::cdef(file_get_contents($argv[1]), $argv[2]);
$out = [];
foreach (json_decode($argv[3], true) as $name => $record) {
    $type = $f->type($name);
    $fields = [];
    foreach ($record['fields'] as $path => $unused) {
        $field = $type; $offset = 0;
        foreach (explode('.', $path) as $part) {
            $offset += $field->getStructFieldOffset($part);
            $field = $field->getStructFieldType($part);
        }
        $fields[$path] = ['offset' => $offset, 'width' => $field->getSize()];
    }
    $out[$name] = ['size' => $type->getSize(), 'alignment' => $type->getAlignment(), 'fields' => $fields];
}
echo json_encode($out, JSON_THROW_ON_ERROR);
''')
    command = [os.environ.get('THINKTHEN_PHP_BIN', '/usr/bin/php8.3'), '-n', '-d', 'extension=ffi',
               '-d', 'ffi.enable=1', str(script), str(copy), str(library), json.dumps(records)]
    return json.loads(subprocess.check_output(command, text=True, env=child_env()))


def check(header, copy, library):
    native = abi.header_abi(header)
    with tempfile.TemporaryDirectory(prefix='thinkthen-php-abi-') as folder:
        scratch = Path(folder)
        copied = copied_abi(copy, scratch)
        # PHP represents every public carrier/import, and the enums retained in its cdef.
        expected = {**native, 'constants': {n: v for n, v in native['constants'].items() if n.startswith(('THINKTHEN_DECLARATION_', 'THINKTHEN_PROPERTY_', 'THINKTHEN_LOAD_'))}}
        abi.compare_abi(expected, copied)
        actual = ffi_layouts(copy, library, copied['records'], scratch)
        layouts = {n: {'size': r['size'], 'alignment': r['alignment'],
                      'fields': {f: {'offset': v['offset'], 'width': v['width']} for f, v in r['fields'].items()}}
                   for n, r in native['records'].items() if n in copied['records']}
        if layouts != actual:
            raise ValueError('PHP FFI carrier layout differs from the target C compiler')
    abi.check_exports(header, library)
    plants = [('field order', 'const char *data; size_t len;', 'size_t len; const char *data;'),
              ('constant', 'THINKTHEN_DECLARATION_STRING_V1=1', 'THINKTHEN_DECLARATION_STRING_V1=99'),
              ('return', 'int thinkthen_result_row(', 'uint64_t thinkthen_result_row('),
              ('argument', 'size_t, size_t, thinkthen_details_v1 *', 'size_t, uint16_t, thinkthen_details_v1 *'),
              ('pointer depth', 'thinkthen_result **);', 'thinkthen_result *);'),
              ('omission', 'int thinkthen_result_rank_member_details(const thinkthen_result *, size_t, size_t, thinkthen_details_v1 *);', ''),
              ('by value', 'uint32_t media, thinkthen_optional_string_v1 filename', 'uint32_t media, const thinkthen_optional_string_v1 *filename')]
    text = copy.read_text()
    with tempfile.TemporaryDirectory(prefix='thinkthen-php-abi-plants-') as folder:
        for name, before, after in plants:
            if before not in text:
                raise ValueError(f'PHP ABI plant has no declaration: {name}')
            planted = Path(folder) / 'abi.h'
            planted.write_text(text.replace(before, after, 1))
            try:
                abi.compare_abi(expected, copied_abi(planted, Path(folder)))
            except ValueError as error:
                if not str(error).startswith('C ABI mismatch:'):
                    raise
            else:
                raise ValueError(f'PHP ABI drift accepted: {name}')
    print(f'PHP C ABI: {len(copied["records"])} actual FFI layouts, {len(copied["constants"])} represented constants, {len(copied["functions"])} exact prototypes match')


if __name__ == '__main__':
    header, copy, library = map(Path, sys.argv[1:]) if len(sys.argv) == 4 else (
        ROOT / 'libraries/c/include/thinkthen.h', ROOT / 'libraries/php/src/native/abi.h',
        ROOT / 'libraries/c/target/debug/libthinkthen_c.so')
    check(header, copy, library)
