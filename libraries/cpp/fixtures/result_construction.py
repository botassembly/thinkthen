"""Check installed result construction boundaries across the generated family."""
import pathlib
import re
import subprocess
import sys
import tempfile

include = pathlib.Path(sys.argv[1]) / 'include'
classes = re.findall(r'class (\w+) : public Node', (include / 'thinkthen/results_generated.hpp').read_text())
assert classes, 'NO_GENERATED_RESULT_CLASSES'
with tempfile.TemporaryDirectory() as directory:
    source = pathlib.Path(directory) / 'construction.cpp'
    source.write_text('#include <thinkthen/client.hpp>\n#include <type_traits>\n' + '\n'.join(
        f'static_assert(!std::is_constructible_v<tt::results::{name}, tt::Data> && !std::is_constructible_v<tt::results::{name}, tt::Data, int>, "PUBLIC_RESULT_CONSTRUCTOR {name}");'
        for name in classes) + '\nstatic_assert(std::is_constructible_v<tt::Data, tt::Data::Object>);\nint main() { tt::inputs::RequestOriginalJson().set_value(tt::Data(false)); }\n')
    command = ['c++', '-std=c++17', '-Wall', '-Wextra', '-Werror', '-fsyntax-only', '-I', str(include), str(source)]
    subprocess.run(command, check=True)
    source.write_text('#include <thinkthen/client.hpp>\nint main() { tt::results::SessionPacket packet(tt::Data{{"kind", "terminal"}}); }\n')
    refused = subprocess.run(command, capture_output=True, text=True)
    assert refused.returncode != 0 and 'no matching' in refused.stderr, 'PUBLIC_PACKET_CONSTRUCTION_ACCEPTED'
print(f'PRIVATE_RESULT_CONSTRUCTION_PASS {len(classes)} classes; authored original Data remains public')
