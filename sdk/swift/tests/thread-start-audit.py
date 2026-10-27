#!/usr/bin/env python3
"""Count actual OS thread starts in a warmed installed Swift consumer."""
import os
from collections import Counter
import json
from pathlib import Path
import subprocess
import sys
import tempfile

consumer = Path(sys.argv[1]).resolve()
source = Path(__file__).resolve().parents[3] / 'scripts/tests/thread-start-audit.c'
with tempfile.TemporaryDirectory(prefix='thinkthen-swift-thread-starts-') as temporary:
    root = Path(temporary)
    library = root / ('audit.dylib' if sys.platform == 'darwin' else 'audit.so')
    flags = ['-dynamiclib'] if sys.platform == 'darwin' else ['-shared', '-fPIC', '-ldl']
    subprocess.run(['cc', '-Wall', '-Wextra', '-Werror', *flags, str(source), '-o', str(library)],
                   check=True)
    results = []
    for rows in (0, 1000):
        log = root / f'{rows}.log'
        env = dict(os.environ, THINKTHEN_THREAD_AUDIT=str(log),
                   THINKTHEN_THREAD_NAME='ThinkThen.row')
        env['DYLD_INSERT_LIBRARIES' if sys.platform == 'darwin' else 'LD_PRELOAD'] = str(library)
        child = subprocess.Popen([str(consumer), 'audit-thread-starts', str(rows)],
                                 stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=env)
        try:
            assert child.stdout.readline() == 'ready\n'
            log.write_text('')
            child.stdin.write('c')
            child.stdin.flush()
            assert child.stdout.readline() == f'rows {rows}\n'
            lines = log.read_text().splitlines()
            images = Counter(Path(line.split('\t')[1]).name for line in lines
                             if line.startswith('start\t'))
            results.append((images, lines.count('named')))
            print(json.dumps({'rows': rows, 'actual_pthread_entry_images': dict(images),
                              'named_swift_workers': lines.count('named')}, sort_keys=True))
            child.stdin.write('x')
            child.stdin.flush()
            assert child.wait(timeout=30) == 0
        finally:
            if child.poll() is None:
                child.kill()
                child.wait()
    baseline, stream = results
    assert baseline == (Counter(), 0), results
    images, named = stream
    bridge = images['Foundation'] + images['libFoundation.so']
    engine = images['libthinkthen_ffi.dylib'] + images['libthinkthen_ffi.so']
    assert named == 1 and bridge == 1, results
    assert engine > 0, results
    other = {image: count for image, count in images.items()
             if image not in ('Foundation', 'libFoundation.so', 'libthinkthen_ffi.dylib',
                              'libthinkthen_ffi.so')}
    assert not other, results
    print(f'Swift 1000 rows: actual pthread starts={sum(images.values())}; '
          f'Swift bridge={bridge}, shared engine={engine}, runtime additions=0; '
          'warmed zero-row control=0 starts')
