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
               ('THINKTHEN_API_KEY', 'OPENAI_API_KEY', 'THINKTHEN_BASE_URL')}
result = subprocess.run([scratch / 'installed/bin/thinkthen', 'decide', '--adapter', 'stub',
    '--details', 'Ready?'], input='evidence 5', text=True, capture_output=True, env=environment)
assert result.returncode in (0, 1, 3), result.stderr
assert json.loads(result.stdout)['status'] in ('answered', 'unsure'), result.stdout
(consumer / 'src').mkdir()
(consumer / 'Cargo.toml').write_text(
    '[package]\nname="library-consumer"\nversion="0.0.0"\nedition="2024"\n'
    '[dependencies]\nthinkthen={version="=' + version + '",default-features=false}\n')
(consumer / 'src/main.rs').write_text('''use thinkthen::{Backend, Decide, Engine, Record};
fn main() -> Result<(), thinkthen::Error> {
    let mut rows = Vec::new();
    Engine::new(Backend::stub()).call(&Decide::new("Ready?")?,
        [Record::text("evidence")], |row| rows.push(row))?;
    assert_eq!(rows.len(), 1);
    Ok(())
}
''')
graph = run('cargo', 'tree', '--offline', '--edges', 'normal', '--prefix', 'none',
            '--format', '{p}', capture_output=True, text=True).stdout
packages = {line.split()[0] for line in graph.splitlines()}
command_only = {'clap', 'clap_builder', 'clap_derive', 'ctrlc', 'command-group',
                'quick-xml', 'zip', 'tempfile'}
assert not packages & command_only, graph
run('cargo', 'run', '--offline', '--quiet')
print('crate: installed command calls stub; library consumer omits command dependencies')
