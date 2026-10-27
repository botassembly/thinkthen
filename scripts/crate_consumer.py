"""Exercise the packaged crate from a consumer with an empty Cargo home."""
import json
import os
from pathlib import Path
import subprocess
import sys

scratch = Path(sys.argv[1]).resolve()
version = sys.argv[2]
consumer = scratch / 'consumer'
(consumer / '.cargo').mkdir(parents=True)
(consumer / '.cargo/config.toml').write_text(
    '[source.crates-io]\nreplace-with="release-packages"\n'
    '[source.release-packages]\ndirectory=' + json.dumps(str(scratch / 'vendor')) + '\n')


def run(*args, **kwargs):
    return subprocess.run(args, cwd=consumer, check=True, **kwargs)


run('cargo', 'install', '--config', consumer / '.cargo/config.toml', '--offline', '--locked', '--root', scratch / 'installed',
    '--version', version, 'thinkthen')
environment = {k: v for k, v in os.environ.items() if k not in
               ('THINKTHEN_API_KEY', 'OPENAI_API_KEY', 'THINKTHEN_BASE_URL', 'THINKTHEN_URL')}
result = subprocess.run([scratch / 'installed/bin/thinkthen', 'decide', '--backend', 'stub',
    '--json', 'Ready?'], input='evidence 5', text=True, capture_output=True, env=environment)
assert result.returncode == 0, result.stderr
assert json.loads(result.stdout)['status'] in ('answered', 'unsure'), result.stdout
(consumer / 'Cargo.toml').write_text(
    '[package]\nname="library-consumer"\nversion="0.0.0"\nedition="2024"\n'
    '[dependencies]\nthinkthen={version="=' + version + '",default-features=false}\n')
from library_trace import check
check(consumer)
graph = run('cargo', 'tree', '--offline', '--edges', 'normal', '--prefix', 'none',
            '--format', '{p}', capture_output=True, text=True).stdout
packages = {line.split()[0] for line in graph.splitlines()}
command_only = {'clap', 'clap_builder', 'clap_derive', 'ctrlc', 'command-group',
                'quick-xml', 'zip', 'tempfile'}
assert not packages & command_only, graph
print('crate: installed command calls stub; library consumer omits command dependencies')
