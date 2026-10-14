#!/usr/bin/env python3
"""Reject files outside a feature consumer's actual Cargo module graph."""
from pathlib import Path
import subprocess
import tempfile
import unittest
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from conformance.children.children import child_env

CHECK = Path(__file__).with_name('feature-inventory.py')


class FeatureInventory(unittest.TestCase):
    def test_consumer_modules_and_unowned_files(self):
        with tempfile.TemporaryDirectory(prefix='feature-inventory-') as folder:
            root = Path(folder)
            env = child_env(home=folder)
            surface = root / 'libraries/polars'
            (surface / 'consumer/src').mkdir(parents=True)
            for name, source in {
                'README.md': 'A feature consumer', 'check.sh': '#!/bin/sh',
                'consumer/Cargo.toml': '[package]\nname="consumer"\nversion="0.0.0"\npublish=false\n',
                'consumer/Cargo.lock': '', 'consumer/src/main.rs': 'mod columns;',
                'consumer/src/columns.rs': '',
            }.items():
                (surface / name).write_text(source)
            subprocess.run(['git', 'init', '-q'], cwd=root, check=True, env=env)
            subprocess.run(['git', 'add', '.'], cwd=root, check=True, env=env)
            def invoke():
                return subprocess.run(['python3', str(CHECK), 'libraries/polars'],
                                      cwd=root, capture_output=True, text=True, env=env)
            self.assertEqual(invoke().returncode, 0)
            for name in ('unknown.json', 'consumer/src/unowned.rs'):
                with self.subTest(name=name):
                    planted = surface / name
                    planted.write_text('unowned')
                    result = invoke()
                    self.assertEqual(result.returncode, 1)
                    self.assertIn(name, result.stderr)
                    planted.unlink()
            (surface / 'consumer/src/main.rs').write_text('')
            result = invoke()
            self.assertEqual(result.returncode, 1)
            self.assertIn('columns.rs', result.stderr)
            for comment in ('// mod columns;', '/*\nmod columns;\n*/',
                            '/* outer /* inner */\nmod columns;\n*/',
                            'const TEXT: &str = r###"\nmod columns;\n"###;',
                            'const TEXT: &str = "\nmod columns;\n";'):
                with self.subTest(source=comment):
                    (surface / 'consumer/src/main.rs').write_text(comment)
                    result = invoke()
                    self.assertEqual(result.returncode, 1)
                    self.assertIn('columns.rs', result.stderr)
            (surface / 'consumer/src/main.rs').write_text(
                'const QUOTE: char = \'"\';\n#[path = "columns.rs"]\nmod columns;')
            self.assertEqual(invoke().returncode, 0)
            (surface / 'check.sh').unlink()
            self.assertIn('check.sh', invoke().stderr)


if __name__ == '__main__':
    unittest.main()
