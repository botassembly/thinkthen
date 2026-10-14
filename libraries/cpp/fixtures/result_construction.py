"""Check installed result construction boundaries across the generated family."""
import pathlib
import re
import subprocess
import sys
import tempfile

include = pathlib.Path(sys.argv[1]) / 'include'
classes = re.findall(r'class (\w+)(?: final)? : public Node', (include / 'thinkthen/results_generated.hpp').read_text())
assert classes, 'NO_GENERATED_RESULT_CLASSES'
with tempfile.TemporaryDirectory() as directory:
    source = pathlib.Path(directory) / 'construction.cpp'
    bypasses = {
        'copy base assignment': ('int replace(tt::results::SessionPacket& packet, const tt::inputs::RequestOriginalJson& input) { static_cast<tt::results::Node&>(packet) = input; return 0; }', 'protected'),
        'move base assignment': ('void replace(tt::results::SessionPacket& packet, tt::inputs::RequestOriginalJson input) { static_cast<tt::results::Node&>(packet) = std::move(input); }', 'protected'),
        'derived storage mutation': ('struct Fabricated : tt::results::SessionPacket { explicit Fabricated(const tt::results::SessionPacket& packet): SessionPacket(packet) { value_ = tt::Data(false); } };', 'final'),
    }
    command = ['c++', '-std=c++17', '-Wall', '-Wextra', '-Werror', '-fsyntax-only', '-I', str(include), str(source)]
    accepted = []
    for name, (body, diagnostic) in bypasses.items():
        source.write_text('#include <thinkthen/client.hpp>\n' + body + '\n')
        refused = subprocess.run(command, capture_output=True, text=True)
        if refused.returncode == 0:
            accepted.append(name)
        else:
            assert diagnostic in refused.stderr, refused.stderr
    assert not accepted, f'PUBLIC_RESULT_MUTATION_ACCEPTED {accepted}'
    source.write_text('#include <thinkthen/client.hpp>\n#include <type_traits>\n'  + '\n'.join(
        f'static_assert(!std::is_constructible_v<tt::results::{name}, tt::Data> && !std::is_constructible_v<tt::results::{name}, tt::Data, int>, "PUBLIC_RESULT_CONSTRUCTOR {name}"); static_assert(std::is_final_v<tt::results::{name}> && std::is_copy_constructible_v<tt::results::{name}> && std::is_move_constructible_v<tt::results::{name}> && std::is_copy_assignable_v<tt::results::{name}> && std::is_move_assignable_v<tt::results::{name}>, "RESULT_VALUE_SEMANTICS {name}");'
        for name in classes) + '\nstatic_assert(std::is_constructible_v<tt::Data, tt::Data::Object>);\nint main() { tt::inputs::RequestOriginalJson().set_value(tt::Data(false)); }\n')
    subprocess.run(command, check=True)
    source.write_text('#include <thinkthen/client.hpp>\nint main() { tt::results::SessionPacket packet(tt::Data{{"kind", "terminal"}}); }\n')
    refused = subprocess.run(command, capture_output=True, text=True)
    assert refused.returncode != 0 and 'no matching' in refused.stderr, 'PUBLIC_PACKET_CONSTRUCTION_ACCEPTED'
print(f'PRIVATE_RESULT_CONSTRUCTION_PASS {len(classes)} classes; authored original Data remains public')
