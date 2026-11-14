"""Build an external Rust guide consumer using the workspace's locked dependencies."""
from contextlib import contextmanager
from pathlib import Path
import json
import os
import shutil
import subprocess
import tempfile

from guide_examples import ROOT


@contextmanager
def consumer(source):
    with tempfile.TemporaryDirectory(prefix='rust-guide-consumer-') as directory:
        folder = Path(directory)
        (folder / 'src').mkdir()
        (folder / 'Cargo.toml').write_text('\n'.join([
            '[package]', 'name="ideal-guide-consumer"', 'version="0.0.0"', 'edition="2024"',
            '[workspace]', '[dependencies]',
            'engine={package="thinkthen",path=' + json.dumps(str(ROOT / 'crates/thinkthen')) + ',default-features=false,features=["stream"]}',
            'serde_json="1"', 'futures="=0.3.31"', '']))
        shutil.copy2(ROOT / 'Cargo.lock', folder / 'Cargo.lock')
        (folder / 'src/main.rs').write_text(source)
        env = dict(os.environ)
        target = Path(env.get('CARGO_TARGET_DIR', ROOT / 'target')).resolve() / 'guide-consumer'
        env['CARGO_TARGET_DIR'] = str(target)
        result = subprocess.run(['cargo', 'build', '--offline', '--manifest-path', str(folder / 'Cargo.toml')],
                                cwd=ROOT, env=env, capture_output=True, text=True, timeout=180)
        if result.returncode:
            raise AssertionError('Rust guide consumer did not compile:\n' + result.stderr[-12000:])
        yield folder, env, target / 'debug/ideal-guide-consumer'


def compile_refusal(folder, env, source):
    (folder / 'src/main.rs').write_text(source)
    return subprocess.run(['cargo', 'check', '--offline', '--manifest-path', str(folder / 'Cargo.toml')],
                          cwd=ROOT, env=env, capture_output=True, text=True, timeout=90)


def rank_fixture():
    """Serve the service guide's illustrative scale distributions in its record order."""
    fixture=json.loads((ROOT/'construction/guide-rank.json').read_text())
    request=fixture['request']
    rows=fixture['rows']
    replies=[]
    for row in rows:
        record=request['records'][row['position']-1]
        replies.append(dict(contains=record,distribution=dict(zip(request['question']['levels'],row['detail']['levels']))))
    return dict(replies=replies),rows
