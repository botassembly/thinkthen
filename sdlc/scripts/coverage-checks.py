"""Focused regressions for the coverage driver's process ownership."""
import os
from pathlib import Path
import runpy
import signal
import sys
import tempfile
import unittest

DRIVER = runpy.run_path(str(Path(__file__).with_name('coverage')))


class ProcessOwnership(unittest.TestCase):
    def test_timeout_stops_and_reaps_descendants_with_diagnostics(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            script = """import os, pathlib, time
child = os.fork()
if child == 0:
    print('descendant diagnostic', flush=True)
    time.sleep(30)
else:
    pathlib.Path('child.pid').write_text(str(child))
    time.sleep(30)
"""
            for capture in (False, True):
                with self.assertRaisesRegex(RuntimeError, r'timed out \(exit -9\)'):
                    DRIVER['command']([sys.executable, '-c', script], root, os.environ,
                                      root / 'timeout.log', capture=capture, timeout=0.3)
                child = int((root / 'child.pid').read_text())
                with self.assertRaises(ProcessLookupError):
                    os.kill(child, 0)
                self.assertIn('descendant diagnostic', (root / 'timeout.log').read_text())

    def test_real_exit_and_diagnostics_survive(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for code, exception, message in ((77, DRIVER['MissingTool'], 'see'),
                                             (4, RuntimeError, 'exit 4'),
                                             (-signal.SIGTERM, RuntimeError, 'exit -15')):
                script = f"import os, sys; print('diagnostic', flush=True); "
                script += f'os.kill(os.getpid(), {-code})' if code < 0 else f'sys.exit({code})'
                with self.assertRaisesRegex(exception, message):
                    DRIVER['command']([sys.executable, '-c', script], root, os.environ, root / 'exit.log')
                self.assertIn('diagnostic', (root / 'exit.log').read_text())


if __name__ == '__main__':
    unittest.main()
