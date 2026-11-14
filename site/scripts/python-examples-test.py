"""Exercise the site's Python example policy through its real checker."""
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

CHECK = Path(__file__).with_name('check-python-examples.py')


class PythonExamples(unittest.TestCase):
    def run_check(self, source, *, section='When to make a client', settings=False,
                  readme='```python\nimport thinkthen as tt\ntt.decide("question", "text")\n```'):
        with tempfile.TemporaryDirectory(prefix='python-example-policy-') as directory:
            root = Path(directory)
            page = root / 'site/src/content/docs/sdks/python.mdx'
            page.parent.mkdir(parents=True)
            page.write_text(f'## {section}\n\n<Example id="settings" />\n')
            program = root / 'site/examples' / ('settings' if settings else 'ordinary') / 'show.py'
            program.parent.mkdir(parents=True)
            program.write_text(source)
            target = root / 'sdk/python/README.md'
            target.parent.mkdir(parents=True)
            target.write_text(readme)
            return subprocess.run([sys.executable, CHECK, root], capture_output=True, text=True)

    def test_module_calls_pass(self):
        self.assertEqual(self.run_check('import thinkthen as tt\ntt.decide("q", "t")').returncode, 0)

    def test_client_calls_fail(self):
        for source in ('import thinkthen as sdk\nsdk.Client()',
                       'import thinkthen.aio as aio\naio.AsyncClient()',
                       'from thinkthen.aio import AsyncClient as Open\nOpen()',
                       'import thinkthen.aio\nthinkthen.aio.AsyncClient()',
                       'from thinkthen import aio as async_tt\nasync_tt.AsyncClient()',
                       'from thinkthen import AsyncClient as Open\nOpen()'):
            with self.subTest(source=source):
                result = self.run_check(source)
                self.assertEqual(result.returncode, 1, result.stderr)
                self.assertIn('show.py', result.stderr)

    def test_only_client_section_can_name_settings_example(self):
        source = 'import thinkthen as tt\ntt.Client(model="chosen")'
        self.assertEqual(self.run_check(source, settings=True).returncode, 0)
        result = self.run_check(source, section='Clients elsewhere', settings=True)
        self.assertEqual(result.returncode, 1, result.stderr)

    def test_approved_client_sections(self):
        for section in ('Settings', 'Usage', 'Async'):
            with self.subTest(section=section):
                self.assertEqual(self.run_check('import thinkthen as tt\ntt.Client()',
                                 section=section, settings=True).returncode, 0)

    def test_readme_requires_module_calls_outside_settings(self):
        readme = '```python\nimport thinkthen as tt\ntt.Client()\n```'
        result = self.run_check('print("ok")', readme=readme)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertIn('README.md', result.stderr)
        self.assertEqual(self.run_check('print("ok")',
                         readme='## When to make a client\n' + readme).returncode, 0)


if __name__ == '__main__':
    unittest.main()
