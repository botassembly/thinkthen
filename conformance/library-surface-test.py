#!/usr/bin/env python3
"""Plant contract omissions and extra exports at the checker CLI boundary."""
import importlib.util
import json
import shutil
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
CHECK = ROOT / 'conformance/library-surface.py'
sys.path.insert(0, str(ROOT))
from conformance.children.children import child_env


class PublicSurface(unittest.TestCase):
    def test_native_adapter_omissions_and_extra_members(self):
        mutations = {
            'c': [('include/thinkthen.h', 'thinkthen_session_new_typed_with_surface(', 'missing_dispatch(', 'missing function: decide'),
                  ('include/thinkthen.h', '} thinkthen_request_item_v1;', '} missing_item_v1;', 'missing helper: item')],
            'ruby': [('lib/thinkthen/session.rb', 'define_singleton_method("question_#{kind}")', 'define_singleton_method("absent_#{kind}")', 'missing helper: question_file')] + [
                     ('lib/thinkthen/session.rb', 'private_constant :Carrier', literal, 'unlisted public type/export: .Carrier')
                     for literal in ('if false\n      private_constant :Carrier\n    end',
                                     'private_constant :Carrier if false',
                                     'while false\n      private_constant :Carrier\n    end',
                                     'false and private_constant :Carrier',
                                     'begin\n      nil\n    rescue StandardError\n      private_constant :Carrier\n    end',
                                     'class << self\n      Carrier = Class.new\n      private_constant :Carrier\n    end',
                                     'private_class_method def self.hide_carrier\n      private_constant :Carrier\n    end',
                                     'def _hide_carrier\n      private\n      private_constant :Carrier\n    end',
                                     'def self._hide_carrier\n      ThinkThen::Client.private_constant :Carrier\n    end',
                                     'deferred = -> {\n      private_constant :Carrier\n    }',
                                     'deferred = lambda do\n      private_class_method :open\n      self.private_constant :Carrier\n    end',
                                     'Deferred = proc do\n      Client.private_constant :Carrier\n    end',
                                     'Object.const_set(:Carrier, Class.new)\n    Object.private_constant :Carrier',
                                     'module Other\n      class Carrier; end\n      private_constant :Carrier\n    end\n    private_constant :Other',
                                     'Object.const_set(:Other, Module.new)\n    Object::Other.const_set(:Carrier, Class.new)\n    Object::Other.private_constant :Carrier',
                                     'Object.const_set(:Other, Module.new)\n    Other.const_set(:Carrier, Class.new)\n    Other.private_constant :Carrier',
                                     '# visible Carrier', '# private_constant :Carrier',
                                     '"private_constant :Carrier"', '%q{private_constant :Carrier}',
                                     '%Q(private_constant :Carrier)', '%[private_constant :Carrier]',
                                     '%q<private_constant :Carrier>', '%q|private_constant :Carrier|',
                                     '%q#private_constant :Carrier#', '%Q!private_constant :Carrier!',
                                     '%q"private_constant :Carrier"', '%q=private_constant :Carrier=',
                                     r'%q{nested {text} \} private_constant :Carrier}',
                                     r'%q|escaped \| private_constant :Carrier|',
                                     '%w[private_constant :Carrier]', '%W[private_constant :Carrier]',
                                     '%i[private_constant :Carrier]', '%I[private_constant :Carrier]',
                                     '%r{private_constant :Carrier}',
                                     '%x{private_constant :Carrier}', '%s{private_constant :Carrier}',
                                     '`private_constant :Carrier`', '/private_constant :Carrier/',
                                     'literal = /private_constant :Carrier/',
                                     'puts /private_constant :Carrier/',
                                     'x = true ? /private_constant :Carrier/ : /other/',
                                     'x = -> { return /private_constant :Carrier/ }',
                                     'x = /a/ && /private_constant :Carrier/',
                                     'x = "#{"private_constant :Carrier"}"',
                                     'x = %Q{#{"}"} private_constant :Carrier}',
                                     r'/[\/] private_constant :Carrier/',
                                     '<<TEXT\nprivate_constant :Carrier\nTEXT',
                                     '<<-TEXT\nprivate_constant :Carrier\n  TEXT',
                                     '<<~\'TEXT\'\nprivate_constant :Carrier\n  TEXT',
                                     '<<"TEXT"\nprivate_constant :Carrier\nTEXT',
                                     '<<`TEXT`\nprivate_constant :Carrier\nTEXT',
                                     '<<\'END-HERE\'\nprivate_constant :Carrier\nEND-HERE',
                                     '[<<ONE, <<TWO]\nfirst literal\nONE\nprivate_constant :Carrier\nTWO',
                                     '\n=begin\nprivate_constant :Carrier\n=end')],
            'typescript': [('native.d.ts', 'readonly context?: R.JsonValue;', 'readonly extraField?: R.JsonValue;', 'unlisted public field: ItemFields.extraField')],
            'zig': [('src/authored.zig', 'pub fn plan(', 'pub fn missingPlan(', 'missing helper: plan'),
                    ('src/thinkthen.zig', 'pub fn usagePersistence(', 'pub fn absentUsage(', 'missing helper: usage_persistence'),
                    ('src/thinkthen.zig', 'closed: bool = false,', 'closed: bool = false, extraField: bool = false,', 'unlisted public field: Engine.extraField'),
                    ('src/results_generated.zig', 'cache_answers: u64,', 'absent_cache_answers: u64,', 'missing generated declaration: results_generated.zig:ImmediateFacts.cache_answers')],
        }
        for library, cases in mutations.items():
            with self.subTest(library=library), tempfile.TemporaryDirectory(prefix='library-surface-native-') as folder:
                root = Path(folder)
                for name in ('sdlc', 'specification', 'conformance'):
                    (root / name).symlink_to(ROOT / name, target_is_directory=True)
                header = root / 'libraries/c/include/thinkthen.h'
                header.parent.mkdir(parents=True)
                shutil.copyfile(ROOT / 'libraries/c/include/thinkthen.h', header)
                if library == 'c':
                    (root / 'libraries/c/src').symlink_to(ROOT / 'libraries/c/src', target_is_directory=True)
                    (root / 'libraries/c/cbindgen.toml').symlink_to(ROOT / 'libraries/c/cbindgen.toml')
                else:
                    for suffix in ('*.rb', '*.rbs', '*.d.ts', '*.zig'):
                        for path in (ROOT / 'libraries' / library).rglob(suffix):
                            if 'target' in path.parts or 'node_modules' in path.parts:
                                continue
                            output = root / path.relative_to(ROOT)
                            output.parent.mkdir(parents=True, exist_ok=True)
                            shutil.copyfile(path, output)
                def invoke():
                    run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--library', library, '--json'],
                                         capture_output=True, text=True, env=child_env(home=folder))
                    self.assertIn(run.returncode, (0, 1), run.stderr)
                    return json.loads(run.stdout)['libraries'][library]
                self.assertEqual(invoke(), [])
                originals = {filename: (root / 'libraries' / library / filename).read_text()
                             for filename, *_ in cases}
                for filename, before, after, diagnostic in cases:
                    with self.subTest(filename=filename, after=after, diagnostic=diagnostic):
                        path = root / 'libraries' / library / filename
                        original = originals[filename]
                        self.assertIn(before, original)
                        path.write_text(original.replace(before, after, 1))
                        self.assertIn(diagnostic, invoke())
                        if library == 'ruby' and before == 'private_constant :Carrier':
                            path.write_text(original.replace(before, after + '\n    ' + before, 1))
                            self.assertEqual(invoke(), [])
                        path.write_text(original)

    def test_dart_mixins_and_generated_schema_still_detect_omissions(self):
        with tempfile.TemporaryDirectory(prefix='library-surface-dart-') as folder:
            root = Path(folder)
            for name in ('sdlc', 'specification', 'conformance'):
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            shutil.copytree(ROOT / 'libraries/dart/lib', root / 'libraries/dart/lib')
            header = root / 'libraries/c/include/thinkthen.h'
            header.parent.mkdir(parents=True)
            shutil.copyfile(ROOT / 'libraries/c/include/thinkthen.h', header)
            def invoke():
                run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--library', 'dart', '--json'],
                                     capture_output=True, text=True, env=child_env(home=folder))
                self.assertIn(run.returncode, (0, 1), run.stderr)
                return json.loads(run.stdout)['libraries']['dart']
            self.assertEqual(invoke(), [])
            client = root / 'libraries/dart/lib/src/session/client.dart'
            original = client.read_text()
            self.assertIn('final class Cancellation {', original)
            client.write_text(original.replace('final class Cancellation {', 'final class Cancellation {\n  bool extraPublicField = false;', 1))
            self.assertIn('unlisted public field: Cancellation.extraPublicField', invoke())
            self.assertIn('with NamedCalls', original)
            client.write_text(original.replace('with NamedCalls', 'with\n    NamedCalls', 1))
            self.assertNotIn('missing function: decide', invoke())
            client.write_text(original.replace('with NamedCalls', '', 1))
            self.assertIn('missing function: decide', invoke())
            client.write_text(original)
            generated = root / 'libraries/dart/lib/src/session/calls_generated.dart'
            original = generated.read_text()
            self.assertIn('decide(', original)
            generated.write_text(original.replace('decide(', 'missingDecide(', 1))
            errors = invoke()
            self.assertIn('missing function: decide', errors)
            self.assertTrue(any(error.startswith('missing generated declaration: calls_generated.dart:NamedCalls.decide')
                                for error in errors), errors)
            generated.write_text(original)
            results = root / 'libraries/dart/lib/src/session/results_generated.dart'
            original = results.read_text()
            self.assertIn('get callId', original)
            results.write_text(original.replace('get callId', 'get missingCallId', 1))
            self.assertTrue(any(error.startswith('missing generated declaration: results_generated.dart:')
                                and '.callId' in error for error in invoke()))

    def test_python_typed_keyword_sources_and_private_exports(self):
        with tempfile.TemporaryDirectory(prefix='library-surface-python-') as folder:
            root = Path(folder)
            for name in ('sdlc', 'specification', 'conformance'):
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            shutil.copytree(ROOT / 'libraries/python/thinkthen', root / 'libraries/python/thinkthen',
                            ignore=shutil.ignore_patterns('__pycache__', '*.so'))
            header = root / 'libraries/c/include/thinkthen.h'
            header.parent.mkdir(parents=True)
            shutil.copyfile(ROOT / 'libraries/c/include/thinkthen.h', header)
            def invoke():
                run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--library', 'python', '--json'],
                                     capture_output=True, text=True, env=child_env(home=folder))
                self.assertIn(run.returncode, (0, 1), run.stderr)
                return json.loads(run.stdout)['libraries']['python']
            self.assertEqual(invoke(), [])
            generated = root / 'libraries/python/thinkthen/_named_generated.py'
            original = generated.read_text()
            for before, after, missing in [('attempts: bool', 'extra_keyword: bool', '.attempts'),
                                            ('backend: str | None', 'backend: int | None', '.backend.str | None')]:
                with self.subTest(before=before):
                    self.assertIn(before, original)
                    generated.write_text(original.replace(before, after, 1))
                    self.assertTrue(any(error.startswith('missing generated declaration: _named_generated.py:')
                                        and missing in error for error in invoke()))
            generated.write_text(original)
            stub = root / 'libraries/python/thinkthen/__init__.pyi'
            source = stub.read_text()
            stub.write_text(source.replace('Unpack[EngineSettings]', 'Unpack[UnknownSettings]'))
            self.assertTrue(any(error.startswith('missing setting: backend') for error in invoke()))
            stub.write_text(source.replace("__all__ = [", "__all__ = ['_private_codec', "))
            self.assertIn('unlisted public call/export: ._private_codec', invoke())

    def test_rbs_keyword_types_and_ruby_private_class_visibility(self):
        with tempfile.TemporaryDirectory(prefix='library-surface-ruby-') as folder:
            root = Path(folder)
            for name in ('sdlc', 'specification', 'conformance'):
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            shutil.copytree(ROOT / 'libraries/ruby/lib', root / 'libraries/ruby/lib')
            shutil.copytree(ROOT / 'libraries/ruby/sig', root / 'libraries/ruby/sig')
            header = root / 'libraries/c/include/thinkthen.h'
            header.parent.mkdir(parents=True)
            shutil.copyfile(ROOT / 'libraries/c/include/thinkthen.h', header)
            def invoke(**values):
                run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--library', 'ruby', '--json'],
                                     capture_output=True, text=True, env=child_env(home=folder, **values))
                self.assertIn(run.returncode, (0, 1), run.stderr)
                return set(json.loads(run.stdout)['libraries']['ruby'])
            baseline = invoke()
            runtime = root / 'libraries/ruby/lib/thinkthen/session.rb'
            original_runtime = runtime.read_text()
            for directive in ('self.private_constant :Carrier',
                              'ThinkThen::Client.private_constant :Carrier',
                              '::ThinkThen::Client.private_constant :Carrier',
                              'Client.private_constant :Carrier'):
                with self.subTest(directive=directive):
                    runtime.write_text(original_runtime.replace('private_constant :Carrier', directive, 1))
                    self.assertEqual(invoke(), baseline)
            runtime.write_text(original_runtime)
            marker = root / 'source-was-executed'
            runtime.write_text(original_runtime + f'\nFile.write({str(marker)!r}, "loaded")\n')
            ruby = shutil.which('ruby')
            self.assertIsNotNone(ruby)
            wrapper = root / 'ruby-lexer'
            wrapper.write_text('#!/bin/sh\n'
                               '[ -z "${RUBYOPT+x}" ] && [ -z "${RUBYLIB+x}" ] && '
                               '[ -z "${TEST_CREDENTIAL+x}" ] || exit 19\n'
                               'exec "' + ruby + '" "$@"\n')
            wrapper.chmod(0o755)
            self.assertEqual(invoke(RUBY=str(wrapper), RUBYOPT='-rmissing_ambient_config',
                                    RUBYLIB=str(root), TEST_CREDENTIAL='fake'), baseline)
            self.assertFalse(marker.exists())
            runtime.write_text(original_runtime)
            unavailable = subprocess.run(
                [sys.executable, str(CHECK), '--root', str(root), '--library', 'ruby', '--json'],
                capture_output=True, text=True,
                env=child_env(home=folder, RUBY=str(root / 'missing-ruby')))
            self.assertEqual(unavailable.returncode, 2)
            self.assertIn('Ruby source lexer unavailable', unavailable.stderr)
            generated = root / 'libraries/ruby/sig/thinkthen/results_generated.rbs'
            original = generated.read_text()
            for before, after in [('?attempts: bool', '?extra_keyword: bool'),
                                  ('?backend: String', '?backend: Integer'),
                                  ('def schema: () -> (String)', 'def schema: () -> (Integer)')]:
                with self.subTest(before=before):
                    self.assertIn(before, original)
                    generated.write_text(original.replace(before, after, 1))
                    self.assertTrue(any(error.startswith('missing generated declaration: results_generated.rbs:')
                                        for error in invoke() - baseline))
            generated.write_text(original)
            overload = '                 | (?backend: String'
            self.assertIn(overload, original)
            generated.write_text(original.replace(overload, overload.replace('String', 'Integer'), 1))
            self.assertIn('unlisted generated declaration: results_generated.rbs:Client.open.backend.Integer',
                          invoke() - baseline)
            generated.write_text(original)
            runtime = root / 'libraries/ruby/lib/thinkthen/session.rb'
            source = runtime.read_text()
            hidden = source.replace('    def self.original(value)',
                                    '    private_class_method :dump\n    def self.original(value)', 1)
            runtime.write_text(hidden)
            self.assertNotIn('unlisted public call/export: Client.dump', invoke())
            runtime.write_text(hidden.replace('private_class_method :dump', ''))
            self.assertIn('unlisted public call/export: Client.dump', invoke())

    def test_ruby_method_visibility_domains_and_reopened_bodies(self):
        cases = [
            ('private\ndef self.extra_public_api; end', True),
            ('def self.extra_public_api; end\nprivate_class_method :extra_public_api', False),
            ('def extra_public_api; end\ndef self.extra_public_api; end\nprivate_class_method :extra_public_api', True),
            ('def extra_public_api; end\ndef self.extra_public_api; end\nprivate :extra_public_api\nprivate_class_method :extra_public_api', False),
            ('private\nend\nclass Client\ndef extra_public_api; end', True),
            ('private\nend\nclass Client\nprivate\ndef extra_public_api; end', False),
            ('class << self\nprivate\nend\nclass << self\ndef extra_public_api; end\nend', True),
            ('class << self\nprivate\nend\nclass << self\nprivate\ndef extra_public_api; end\nend', False),
            ('def self.extra_public_api; end\nclass << self\nprivate :extra_public_api\nend\ndef extra_public_api; end', True),
            ('def self.extra_public_api; end\nclass << self\nprivate :extra_public_api\nend\nprivate def extra_public_api; end', False),
            ('private def extra_public_api; end\ndef self.extra_public_api; end', True),
            ('private def extra_public_api; end\nprivate_class_method def self.extra_public_api; end', False),
            ('def self.extra_public_api; end\nprivate_class_method :extra_public_api\npublic_class_method :extra_public_api', True),
            ('class << self\nprivate\ndef extra_public_api; end\nend', False),
            ('class << self\ndef self.extra_public_api; end\nprivate_class_method :extra_public_api\nend', False),
            ('class << self\ndef extra_public_api; end\ndef self.extra_public_api; end\nprivate_class_method :extra_public_api\nend', True),
            ('private\npublic\ndef extra_public_api; end', True),
            ('def extra_public_api; end\nend\nclass Client\nprivate :extra_public_api', False),
        ]
        with tempfile.TemporaryDirectory(prefix='library-surface-ruby-domains-') as folder:
            root = Path(folder)
            for name in ('sdlc', 'specification', 'conformance'):
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            shutil.copytree(ROOT / 'libraries/ruby/lib', root / 'libraries/ruby/lib')
            shutil.copytree(ROOT / 'libraries/ruby/sig', root / 'libraries/ruby/sig')
            header = root / 'libraries/c/include/thinkthen.h'
            header.parent.mkdir(parents=True)
            shutil.copyfile(ROOT / 'libraries/c/include/thinkthen.h', header)
            runtime = root / 'libraries/ruby/lib/thinkthen/session.rb'
            original = runtime.read_text()
            for body, public in cases:
                with self.subTest(body=body, public=public):
                    runtime.write_text(original + '\nmodule ThinkThen\nclass Client\n' + body + '\nend\nend\n')
                    run = subprocess.run([sys.executable, str(CHECK), '--root', str(root), '--library', 'ruby', '--json'],
                                         capture_output=True, text=True, env=child_env(home=folder))
                    self.assertIn(run.returncode, (0, 1), run.stderr)
                    errors = json.loads(run.stdout)['libraries']['ruby']
                    expected = ['unlisted public call/export: Client.extra_public_api'] if public else []
                    self.assertEqual(errors, expected)

    def test_ada_generated_builders_types_and_private_visibility(self):
        with tempfile.TemporaryDirectory(prefix='library-surface-ada-') as folder:
            root = Path(folder)
            for name in ('sdlc', 'specification', 'conformance', 'crates'):
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            shutil.copytree(ROOT / 'libraries/ada/src', root / 'libraries/ada/src')
            header = root / 'libraries/c/include/thinkthen.h'
            header.parent.mkdir(parents=True)
            shutil.copyfile(ROOT / 'libraries/c/include/thinkthen.h', header)
            def invoke():
                run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--library', 'ada', '--json'],
                                     capture_output=True, text=True, env=child_env(home=folder))
                self.assertIn(run.returncode, (0, 1), run.stderr)
                return set(json.loads(run.stdout)['libraries']['ada'])
            baseline = invoke()
            generated = root / 'libraries/ada/src/thinkthen-inputs.ads'
            original = generated.read_text()
            setter = next(line for line in original.splitlines()
                          if line.startswith('function With_Attempts (Item : Thinkthen.Inputs.Options;'))
            for changed in (setter.replace('Field : Thinkthen.Inputs.Boolean_Value', 'Field : String'),
                            setter.replace('Field : Thinkthen.Inputs.Boolean_Value', 'Field : Thinkthen.Requests.T_Boolean'),
                            ''):
                with self.subTest(changed=changed):
                    generated.write_text(original.replace(setter, changed, 1))
                    self.assertTrue(any(error.startswith('missing generated declaration: thinkthen-inputs.ads:')
                                        for error in invoke() - baseline))
            generated.write_text(original)
            hidden = original.replace('\nprivate\n', '\nprivate\nprocedure Private_Tail_Plant;\n', 1)
            self.assertNotEqual(hidden, original)
            generated.write_text(hidden)
            self.assertEqual(invoke(), baseline)
            generated.write_text(hidden.replace('\nprivate\n', '\n', 1))
            self.assertIn('unlisted public call/export: .Private_Tail_Plant', invoke() - baseline)
            generated.write_text(original)
            private = root / 'libraries/ada/src/thinkthen-private_plant.ads'
            private.write_text('private package Thinkthen.Private_Plant is\n'
                               'type Private_Package_Plant is null record;\nend Thinkthen.Private_Plant;\n')
            self.assertEqual(invoke(), baseline)
            private.write_text(private.read_text().replace('private package', 'package', 1))
            self.assertIn('unlisted public type/export: .Private_Package_Plant', invoke() - baseline)

    def test_cobol_outer_guard_preserves_real_public_macros(self):
        with tempfile.TemporaryDirectory(prefix='library-surface-cobol-') as folder:
            root = Path(folder)
            for name in ('sdlc', 'specification', 'conformance'):
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            shutil.copytree(ROOT / 'libraries/cobol', root / 'libraries/cobol')
            header = root / 'libraries/c/include/thinkthen.h'
            header.parent.mkdir(parents=True)
            shutil.copyfile(ROOT / 'libraries/c/include/thinkthen.h', header)
            bridge = root / 'libraries/cobol/src/tt_session.h'
            def invoke():
                result = subprocess.run(['python3', str(CHECK), '--root', str(root), '--library', 'cobol', '--json'], capture_output=True, text=True, env=child_env(home=folder))
                self.assertIn(result.returncode, (0, 1), result.stderr)
                return result.returncode, json.loads(result.stdout)['libraries']['cobol']
            self.assertEqual(invoke(), (0, []))
            original = bridge.read_text()
            bridge.write_text(original.replace('#endif', '#define TT_PUBLIC_EMPTY\n#define TT_PUBLIC_NUMBER 17\n#define TT_UNMATCHED\n#endif'))
            code, errors = invoke()
            self.assertEqual(code, 1)
            for name in ('TT_PUBLIC_EMPTY', 'TT_PUBLIC_NUMBER', 'TT_UNMATCHED'):
                self.assertIn('unlisted public type/export: .' + name, errors)
            self.assertFalse(any('TT_COBOL_SESSION_H' in error for error in errors))
            bridge.write_text(original.replace('#endif', ''))
            code, errors = invoke()
            self.assertEqual(code, 1)
            self.assertIn('unlisted public type/export: .TT_COBOL_SESSION_H', errors)

    def test_r_namespace_and_nested_formal_defaults(self):
        with tempfile.TemporaryDirectory(prefix='library-surface-r-') as folder:
            root = Path(folder)
            for name in ['sdlc', 'specification', 'conformance']:
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            shutil.copytree(ROOT / 'libraries/r/thinkthen/R', root / 'libraries/r/thinkthen/R')
            namespace = root / 'libraries/r/thinkthen/NAMESPACE'
            namespace.write_text((ROOT / 'libraries/r/thinkthen/NAMESPACE').read_text())
            header = root / 'libraries/c/include/thinkthen.h'
            header.parent.mkdir(parents=True)
            shutil.copyfile(ROOT / 'libraries/c/include/thinkthen.h', header)
            source = root / 'libraries/r/thinkthen/R/surface-fixture.R'
            source.write_text('''# export(fakeCommentExport)
export(fakeSourceExport)
.tt_decide <- function(hidden = NULL) {}
extraPublicExport <- function(context = NULL, enabled = FALSE,
    close = function() invisible(NULL), later = list(a = 1, b = 2), ...) {}
''')
            def invoke():
                run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--library', 'r', '--json'],
                                     capture_output=True, text=True, env=child_env(home=folder))
                self.assertIn(run.returncode, (0, 1), run.stderr)
                return json.loads(run.stdout)['libraries']['r']
            baseline = invoke()
            self.assertFalse(any(error.endswith(('.NULL', '.FALSE', '.hidden')) for error in baseline))
            self.assertFalse(any('fake' in error or 'extraPublicExport' in error for error in baseline))
            generated = root / 'libraries/r/thinkthen/R/inputs-generated.R'
            original = generated.read_text()
            generated.write_text(original.replace('function(attempts = NULL', 'function(extra_option = NULL', 1))
            self.assertIn('missing option: attempts in tt_options', invoke())
            generated.write_text(original.replace('function(entity_definition = NULL', 'function(extra_field = NULL', 1))
            self.assertIn('unlisted public field: tt_recognize_question.extra_field', invoke())
            generated.write_text(original)
            feed = root / 'libraries/r/thinkthen/R/feeds.R'
            original_feed = feed.read_text()
            for default in ['"records"', '"#"', "'#'", '"escaped\\\"#,)]}"']:
                with self.subTest(default=default):
                    feed.write_text(original_feed.replace('name = "records",', 'name = ' + default + ',', 1))
                    self.assertEqual(invoke(), baseline)
                    for extra in ['NULL', 'list(a = c(1, 2), b = function() { "#" })']:
                        feed.write_text(original_feed.replace('name = "records",',
                            'name = ' + default + ', # Ignore extra_hidden = NULL, )\n'
                            '                    extra_option = ' + extra + ',', 1))
                        self.assertEqual(set(invoke()) - set(baseline),
                                         {'unlisted public field: tt_feed.extra_option'})
            feed.write_text(original_feed)
            namespace.write_text(namespace.read_text() + '\nexport(extraPublicExport)\n')
            errors = invoke()
            self.assertIn('unlisted public call/export: .extraPublicExport', errors)
            for name in ['context', 'enabled', 'close', 'later']:
                self.assertIn('unlisted public field: extraPublicExport.' + name, errors)
            namespace.write_text(namespace.read_text().replace('export(tt_decide)', ''))
            self.assertIn('missing function: decide', invoke())
            namespace.write_text(namespace.read_text().replace('export(tt_plan)', ''))
            self.assertIn('missing helper: plan', invoke())

    def test_cpp_public_inheritance_and_accessibility(self):
        with tempfile.TemporaryDirectory(prefix='library-surface-cpp-') as folder:
            root = Path(folder)
            for name in ['sdlc', 'specification', 'conformance']:
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            shutil.copytree(ROOT / 'libraries/cpp/include', root / 'libraries/cpp/include')
            target = root / 'libraries/c/include/thinkthen.h'
            target.parent.mkdir(parents=True)
            shutil.copyfile(ROOT / 'libraries/c/include/thinkthen.h', target)
            client = root / 'libraries/cpp/include/thinkthen/client.hpp'
            calls = root / 'libraries/cpp/include/thinkthen/calls_generated.hpp'
            original, generated = client.read_text(), calls.read_text()
            def invoke(source=original, mixin=generated):
                client.write_text(source)
                calls.write_text(mixin)
                run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--library', 'cpp', '--json'],
                                     capture_output=True, text=True, env=child_env(home=folder))
                self.assertIn(run.returncode, (0, 1), run.stderr)
                return json.loads(run.stdout)['libraries']['cpp']
            baseline = invoke()
            self.assertFalse([e for e in baseline if e.startswith(('missing ', 'unlisted generated'))])
            for name in ['Owner', 'T', 'EngineDeleter', 'SessionDeleter', 'PacketDeleter', 'Parser']:
                self.assertNotIn('unlisted public type/export: .' + name, baseline)
            errors = invoke(mixin=generated.replace(' decide(', ' missingDecide('))
            self.assertIn('missing function: decide', errors)
            self.assertIn('missing generated declaration: calls_generated.hpp:NamedCalls.decide', errors)
            for replacement in ['class Client {', 'class Client : private NamedCalls<Client> {']:
                errors = invoke(original.replace('class Client : public NamedCalls<Client> {', replacement))
                self.assertIn('missing function: decide', errors)
                self.assertIn('missing helper: plan', errors)
            unrelated = original.replace('class Client : public NamedCalls<Client> {', 'class Client {')
            errors = invoke(unrelated + '\nnamespace tt { struct Other : NamedCalls<Other> {}; }\n')
            self.assertIn('missing function: decide', errors)
            hidden = ('\nnamespace tt::detail { struct Hidden { void extraHidden(); }; void hiddenFree(); }\n'
                      'namespace tt { class Other { class Nested { public: void hiddenNested(); };'
                      ' void hiddenPrivate(); protected: void hiddenProtected(); public: void extraPublicExport(); }; }\n')
            errors = invoke(original + hidden)
            self.assertIn('unlisted public call/export: Other.extraPublicExport', errors)
            self.assertEqual(set(errors) - set(baseline), {
                'unlisted public type/export: .Other', 'unlisted public call/export: Other.extraPublicExport'})
            # Native category adapters accept only their documented members.
            for filename, owner in [('value.hpp', 'Data'), ('result_value.hpp', 'Node')]:
                path = client.with_name(filename)
                path.write_text(path.read_text().replace('class ' + owner + ' {',
                    'class ' + owner + ' { public: void extraPublicExport(); private:', 1))
            extra = original.replace('class Call {', 'class Call { public: void extraPublicExport(); private:', 1)
            extra = extra.replace('class AnswerCall : public Call {',
                                  'class AnswerCall : public Call { public: void extraPublicExport(); private:', 1)
            errors = invoke(extra)
            for owner in ['Data', 'Node', 'Call', 'AnswerCall']:
                self.assertIn('unlisted public call/export: ' + owner + '.extraPublicExport', errors)

    def test_scoped_options_helpers_settings_generated_members_and_extra_exports(self):
        with tempfile.TemporaryDirectory(prefix='library-surface-') as folder:
            root = Path(folder)
            def write(path, source):
                target = root / path
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(source)
            (root / 'sdlc').mkdir()
            (root / 'sdlc/generators').symlink_to(ROOT / 'sdlc/generators', target_is_directory=True)
            write('libraries/c/include/thinkthen.h', '')
            definitions = {
                'RequestVersion': {'oneOf': [{'const': 'fixture'}]},
                'RequestFunction': {'oneOf': [{'const': 'decide'}]},
                'RequestCall': {'type': 'object', 'properties': {'function': {'$ref': '#/$defs/RequestFunction'}, 'options': {'$ref': '#/$defs/RequestOptions'}}},
                'RequestOptions': {'type': 'object', 'properties': {'context': {'type': 'string'}}},
                'EngineSettings': {'type': 'object', 'properties': {'backend': {'type': 'string'}}},
                'RequestSessionDescriptor': {'type': 'object', 'properties': {}},
            }
            request = {'$ref': '#/$defs/RequestCall', '$defs': definitions}
            definitions['RequestDefinition'] = definitions['RequestCall']
            result = {'$ref': '#/$defs/completesessionPacket', '$defs': {
                'completesessionPacket': {'oneOf': [
                    {'type': 'object', 'properties': {'kind': {'const': 'aggregate'},
                        'function': {'const': 'decide'}, 'value': {'type': 'boolean'}},
                     'required': ['kind', 'function', 'value']},
                    {'type': 'object', 'properties': {'kind': {'const': 'observation'},
                        'payload': {'type': 'string'},
                        'judgment': {'$ref': '#/$defs/completesessionJudgment'}},
                     'required': ['kind', 'judgment']}]},
                'completesessionJudgment': {'oneOf': [
                    {'type': 'object', 'x-request-function': function,
                     'properties': {'kind': {'const': function}, 'value': {'type': 'boolean'}},
                     'required': ['kind', 'value']} for function in ('decide', 'choose')]},
                'completeplan': {'type': 'object', 'properties': {}}}}
            # Keep the aggregate's receipt contract owned by the real producer.
            canonical = json.loads((ROOT / 'crates/thinkthen/src/public/results/complete.schema.json').read_text())
            aggregate = next(variant for variant in canonical['$defs']['completesessionPacket']['oneOf']
                             if variant.get('properties', {}).get('kind', {}).get('const') == 'aggregate'
                             and variant['properties']['function']['const'] == 'decide')
            result['$defs']['completesessionPacket']['oneOf'][0]['properties']['value'] = aggregate['properties']['value']
            result['$defs'] = dict(canonical['$defs'], **result['$defs'])
            write('request.json', json.dumps(request))
            write('result.json', json.dumps(result))
            manifest = json.loads((ROOT / 'conformance/library-surface.json').read_text())
            manifest['authority'] = {'request': 'request.json', 'result': 'result.json'}
            config = manifest['libraries']['typescript']
            manifest['libraries'] = {'typescript': config}
            config['declarations'] = ['libraries/typescript/*.d.ts']
            config['option_containers'] = ['Controls']
            config['setting_containers'] = ['Settings']
            config['function_containers'] = ['Client']
            config['representation_types'] = ['Settings']
            write('conformance/library-surface.json', json.dumps(manifest))
            generated = ('export type JsonValue = null | boolean | number | NativeNumber | string | readonly JsonValue[] | { readonly [key: string]: JsonValue };\n'
                         'export class NativeNumber { readonly raw: string; private constructor(); }\n'
                         'export class NativeSessionPacket {\n readonly "payload"?: string;\n has(key: string): boolean;\n readonly [key: string]: unknown;\n}\n')
            # Produce the remaining native declarations through the same renderer.
            spec = importlib.util.spec_from_file_location('fixture_typescript', ROOT / 'sdlc/generators/results/templates/typescript.py')
            template = importlib.util.module_from_spec(spec)
            original_path = sys.path[:]
            sys.path[:0] = [str(Path(spec.origin).parent), str(Path(spec.origin).parent.parent)]
            try:
                spec.loader.exec_module(template)
                import generate
            finally:
                sys.path[:] = original_path
            generated = template.declarations(generate.prepare(generate.graph(result, template.ROOTS)),
                                              generate.prepare(generate.graph(request, template.INPUT_ROOTS)))
            write('libraries/typescript/results_generated.d.ts', generated)
            declarations = ('export interface Controls { readonly context?: string; readonly [key:string]: unknown; }\n'
                            'export interface Settings { readonly backend?: string; }\n'
                            'export class Client { decide(): void; static questionFile(): void; static questionName(): void;\n'
                            'static questionReference(): void; static item(): void; static files(): void; }\n')
            declarations += ''.join('export function ' + name + '(): void;\n' for name in ['plan', 'usagePersistence', 'finishUsageStatus'])
            def invoke(source=declarations, graph=generated):
                write('libraries/typescript/native.d.ts', source)
                write('libraries/typescript/results_generated.d.ts', graph)
                run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--json'],
                                     capture_output=True, text=True,
                                     env=child_env(home=folder))
                return run.returncode, json.loads(run.stdout)['libraries']['typescript']
            code, errors = invoke()
            self.assertEqual((code, errors), (0, []))
            code, errors = invoke(declarations.replace('readonly context?: string;', '') + '\n// context exists in prose only\nexport interface Other { context: string; }')
            self.assertEqual(code, 1)
            self.assertTrue(any(error.startswith('missing option: context') for error in errors))
            code, errors = invoke(declarations + '\nexport function extraPublicExport(): void;')
            self.assertEqual(code, 1)
            self.assertIn('unlisted public call/export: .extraPublicExport', errors)
            code, errors = invoke(declarations.replace('export class Client', 'class Client'))
            self.assertIn('missing function: decide', errors)
            private_helper = declarations.replace('export class Client', 'class Client').replace('decide(): void;', 'decide(): void; plan(): void;').replace('export function plan(): void;', '')
            code, errors = invoke(private_helper)
            self.assertIn('missing helper: plan', errors)
            code, errors = invoke(declarations.replace('export class Client', 'class InternalClient') + '\nexport { InternalClient as Client };')
            self.assertEqual((code, errors), (0, []))
            code, errors = invoke(declarations.replace('export class Client', 'class Client') + '\nexport { Client };')
            self.assertEqual((code, errors), (0, []))
            code, errors = invoke(declarations.replace('decide(): void;', 'decide(): void; cancel(): void;'))
            self.assertEqual((code, errors), (0, []))
            code, errors = invoke(declarations.replace('readonly context?: string;', 'readonly context?: string; cancel(): void;'))
            self.assertIn('unlisted public call/export: Controls.cancel', errors)
            code, errors = invoke(declarations.replace('readonly context?: string;', 'readonly context?: string; value(): string;'))
            self.assertIn('unlisted public call/export: Controls.value', errors)
            code, errors = invoke(declarations + '\nexport class Completed { cancel(): void; }')
            self.assertIn('unlisted public call/export: Completed.cancel', errors)
            code, errors = invoke(declarations.replace('readonly backend?: string;', ''))
            self.assertTrue(any(error.startswith('missing setting: backend') for error in errors))
            code, errors = invoke(declarations.replace('export function plan(): void;', ''))
            self.assertIn('missing helper: plan', errors)
            code, errors = invoke(graph=generated.replace(' readonly "payload"?: string;', ''))
            self.assertTrue(any('missing generated declaration:' in error and '.payload' in error for error in errors))
            code, errors = invoke(graph=generated.replace(' readonly "payload"?: string;', ' readonly "payload"?: number;'))
            self.assertTrue(any('missing generated declaration:' in error and '.payload.' in error for error in errors), errors)
            self.assertIn('"choose"', generated)
            code, errors = invoke(graph=generated.replace('"choose"', '"cho ose"', 1))
            self.assertEqual(code, 1)
            self.assertTrue(any('unlisted generated declaration:' in error and '"cho ose"' in error
                                for error in errors), errors)
            inherited = declarations.replace('decide(): void;', '') + '\nexport interface Client extends R.TypedCalls {}\nexport interface TypedCalls { decide(): void; }\n'
            self.assertNotIn('missing function: decide', invoke(inherited)[1])
            self.assertIn('missing function: decide', invoke(inherited.replace(' extends R.TypedCalls', ''))[1])
            code, errors = invoke(graph=generated.replace(' readonly "payload"?: string;', ' readonly "payload"?: string;\n surprise: string;'))
            self.assertTrue(any('unlisted generated declaration:' in error and '.surprise' in error for error in errors))

    def test_go_lifecycle_calls_belong_to_native_owners(self):
        manifest = json.loads((ROOT / 'conformance/library-surface.json').read_text())
        with tempfile.TemporaryDirectory(prefix='library-surface-go-') as folder:
            root = Path(folder)
            for name in ['sdlc', 'specification']:
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            for path in [ROOT / 'libraries/c/include/thinkthen.h', *ROOT.glob('libraries/go/*.go')]:
                target = root / path.relative_to(ROOT)
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(path, target)
            target = root / 'libraries/go/extra.go'
            target.write_text('package thinkthen\nfunc (v RequestOptions) Cancel() {}\n'
                              'type hidden struct {}\ntype Exported struct { hidden }\ntype ExportedAlias = hidden\n'
                              'func (v hidden) ExtraPublicExport() {}\n'
                              'type returned struct {}\nfunc NewReturned() returned { return returned{} }\n'
                              'func (v returned) ExtraReturnedExport() {}\n'
                              'type unreachable struct {}\nfunc (v unreachable) InvisibleExport() {}\n'
                              'func (v ownedJSON) ExtraRepresentationExport() {}\n')
            run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--manifest',
                                  str(ROOT / 'conformance/library-surface.json'), '--library', 'go', '--json'],
                                 capture_output=True, text=True, env=child_env(home=folder))
            errors = json.loads(run.stdout)['libraries']['go']
            self.assertEqual(run.returncode, 1)
            self.assertFalse(any('schema declaration derivation failed:' in error for error in errors))
            self.assertIn('unlisted public call/export: RequestOptions.Cancel', errors)
            self.assertIn('unlisted public call/export: Exported.ExtraPublicExport', errors)
            self.assertIn('unlisted public call/export: ExportedAlias.ExtraPublicExport', errors)
            self.assertIn('unlisted public call/export: returned.ExtraReturnedExport', errors)
            self.assertIn('unlisted public call/export: Facts.ExtraRepresentationExport', errors)
            self.assertFalse(any('InvisibleExport' in error for error in errors))
            self.assertNotIn('unlisted public call/export: Client.Close', errors)
            self.assertFalse(any('nativeEngine.' in error or 'ownedJSON.' in error for error in errors))
            generated = root / 'libraries/go/owned_results_generated.go'
            source = generated.read_text()
            source = source.replace('Context           *string', 'Surprise          *string')
            generated.write_text(source)
            run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--manifest',
                                  str(ROOT / 'conformance/library-surface.json'), '--library', 'go', '--json'],
                                 capture_output=True, text=True, env=child_env(home=folder))
            errors = json.loads(run.stdout)['libraries']['go']
            self.assertIn('missing generated declaration: owned_results_generated.go:RequestOptions.Context', errors)
            self.assertIn('unlisted generated declaration: owned_results_generated.go:RequestOptions.Surprise', errors)

    def test_csharp_generated_named_calls_are_checked_at_the_cli(self):
        manifest = json.loads((ROOT / 'conformance/library-surface.json').read_text())
        with tempfile.TemporaryDirectory(prefix='library-surface-csharp-') as folder:
            root = Path(folder)
            for name in ['sdlc', 'specification']:
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            paths = [ROOT / 'libraries/c/include/thinkthen.h']
            for pattern in manifest['libraries']['csharp']['declarations']:
                paths.extend(ROOT.glob(pattern))
            for path in paths:
                target = root / path.relative_to(ROOT)
                target.parent.mkdir(parents=True, exist_ok=True)
                if path.is_relative_to(ROOT / 'sdlc'):
                    continue
                shutil.copyfile(path, target)
            calls = root / 'libraries/csharp/src/NamedCalls.g.cs'
            original = calls.read_text()
            def invoke(source):
                calls.write_text(source)
                run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--manifest',
                                      str(ROOT / 'conformance/library-surface.json'), '--library', 'csharp', '--json'],
                                     capture_output=True, text=True, env=child_env(home=folder))
                return json.loads(run.stdout)['libraries']['csharp']
            baseline = invoke(original)
            self.assertEqual(baseline, [])
            extra = root / 'libraries/csharp/src/Extra.cs'
            methods = ['Open', 'ParseQuestion', 'StartSession', 'ExecuteAsync', 'PushAsync', 'ReadAsync',
                       'Decision', 'Choice', 'Tags', 'DecideValueAsync', 'ExtraPublicExport']
            extra.write_text('public class Other {\n'
                             + ''.join('public void ' + method + '() {}\n' for method in methods)
                             + 'public string FactsJson {get;} public string Failure {get;} }\n'
                             + 'public class RequestOptions { public string SurpriseOption {get;} }\n'
                             + 'public class EngineSettings { public string SurpriseSetting {get;} }\n')
            unexpected = invoke(original)
            for method in methods:
                self.assertIn('unlisted public call/export: Other.' + method, unexpected)
            for owner, field in [('Other', 'FactsJson'), ('Other', 'Failure'),
                                 ('RequestOptions', 'SurpriseOption'), ('EngineSettings', 'SurpriseSetting')]:
                self.assertIn('unlisted public field: ' + owner + '.' + field, unexpected)
            extra.write_text('public class TransportBase { public string TransportJson {get;} }\n'
                             'public class Engine : TransportBase {}\n')
            self.assertIn('unlisted public field: TransportBase.TransportJson', invoke(original))
            extra.unlink()
            inputs = root / 'libraries/csharp/src/RequestInputs.g.cs'
            input_source = inputs.read_text()
            inputs.write_text(input_source.replace('AuthoredPresence<string> Context {', 'AuthoredPresence<string> SurpriseOption {')
                              .replace('AuthoredPresence<string> Backend {', 'AuthoredPresence<string> SurpriseSetting {'))
            drift = invoke(original)
            for kind, field in [('option', 'context'), ('setting', 'backend')]:
                self.assertTrue(any(error.startswith('missing ' + kind + ': ' + field) for error in drift))
            for owner, before, after in [('RequestOptions', 'Context', 'SurpriseOption'),
                                         ('EngineSettings', 'Backend', 'SurpriseSetting')]:
                self.assertIn('missing generated declaration: RequestInputs.g.cs:' + owner + '.' + before, drift)
                self.assertIn('unlisted generated declaration: RequestInputs.g.cs:' + owner + '.' + after, drift)
            inputs.write_text(input_source)
            renamed = invoke(original.replace('DecideAsync(', 'SurpriseAsync('))
            self.assertIn('missing generated declaration: NamedCalls.g.cs:Engine.DecideAsync', renamed)
            self.assertIn('unlisted generated declaration: NamedCalls.g.cs:Engine.SurpriseAsync', renamed)
            field = invoke(original.replace('TProbability Probability {', 'TProbability SurpriseProbability {'))
            self.assertIn('missing generated declaration: NamedCalls.g.cs:CallResult.Probability', field)
            self.assertIn('unlisted generated declaration: NamedCalls.g.cs:CallResult.SurpriseProbability', field)

    def test_java_record_components_are_checked_at_the_cli(self):
        manifest = json.loads((ROOT / 'conformance/library-surface.json').read_text())
        with tempfile.TemporaryDirectory(prefix='library-surface-records-') as folder:
            root = Path(folder)
            for name in ['sdlc', 'specification', 'crates']:
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            paths = [ROOT / 'libraries/c/include/thinkthen.h']
            for pattern in manifest['libraries']['jvm']['declarations']:
                paths.extend(ROOT.glob(pattern))
            for path in paths:
                target = root / path.relative_to(ROOT)
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(path, target)
            records = root / 'libraries/jvm/session/thinkthen/Results.java'
            original = records.read_text()
            def invoke(source):
                records.write_text(source)
                run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--manifest',
                                      str(ROOT / 'conformance/library-surface.json'), '--library', 'jvm', '--json'],
                                     capture_output=True, text=True, env=child_env(home=folder))
                return json.loads(run.stdout)['libraries']['jvm']
            baseline = invoke(original)
            missing = invoke(original.replace('record BatchInteger(BigInteger value)',
                                              'record BatchInteger(BigInteger renamed)'))
            self.assertIn('missing generated declaration: Results.java:BatchInteger.value', missing)
            self.assertIn('unlisted generated declaration: Results.java:BatchInteger.renamed', missing)
            extra = invoke(original.replace('record BatchInteger(BigInteger value)',
                                            'record BatchInteger(BigInteger value, java.util.Map<String, java.util.List<Results.BatchInteger>>[] surprise)'))
            self.assertIn('unlisted generated declaration: Results.java:BatchInteger.surprise', extra)
            self.assertNotIn('missing generated declaration: Results.java:BatchInteger.value', baseline)
            self.assertEqual(baseline, [])
            invoke(original)
            for filename, container, facade in [('thinkthen/NamedCalls.java', 'Engine', ''),
                                                   ('kotlin/KotlinNamedCalls.kt', 'KotlinEngine', ' (kotlin)'),
                                                   ('scala/ScalaNamedCalls.scala', 'ScalaEngine', ' (scala)')]:
                base = root / 'libraries/jvm/session' / filename
                source = base.read_text()
                base.write_text(source.replace('decide(', 'surprise('))
                self.assertIn('missing function: decide' + facade, invoke(original))
                base.write_text(source)
            engine = root / 'libraries/jvm/session/thinkthen/Engine.java'
            source = engine.read_text()
            engine.write_text(source.replace('extends NamedCalls', 'extends AbsentCalls'))
            self.assertIn('missing function: decide', invoke(original))
            engine.write_text(source)
            unrelated = root / 'libraries/jvm/session/thinkthen/Unrelated.java'
            unrelated.write_text('public class Unrelated { public void execute() {} }')
            self.assertIn('unlisted public call/export: Unrelated.execute', invoke(original))
            unrelated.unlink()
            inputs = root / 'libraries/jvm/session/thinkthen/Inputs.java'
            source = inputs.read_text()
            inputs.write_text(source.replace(' context(', ' surpriseOption(').replace(' backend(', ' surpriseSetting('))
            errors = invoke(original)
            self.assertTrue(any(error.startswith('missing option: context') for error in errors))
            self.assertTrue(any(error.startswith('missing setting: backend') for error in errors))
            inputs.write_text(source)
            private = root / 'libraries/jvm/session/thinkthen/Json.java'
            private.write_text(private.read_text().replace('class Json', 'public class Json'))
            self.assertIn('unlisted public type/export: .Json', invoke(original))

    def test_php_traits_and_explicit_internal_namespaces(self):
        spec = importlib.util.spec_from_file_location('surface_php', CHECK)
        checker = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(checker)
        source = ('namespace ThinkThen\\Inputs; class Presence { public function __construct('
                  'public bool $present, public string|null $value, private mixed $hidden = null) {} } '
                  'namespace ThinkThen\\Internal; class Hidden { public function extraHidden() {} } '
                  'namespace ThinkThen\\InternalPublic; class Visible { public function extraVisible() {} }')
        declared = checker.READ.declarations(source, 'php')
        self.assertTrue({checker.READ.Declaration('Presence', name, 'field') for name in ['present', 'value']} <= declared)
        self.assertFalse(any(item.name in ['hidden', 'Hidden', 'extraHidden'] for item in declared))
        self.assertIn(checker.READ.Declaration('Visible', 'extraVisible', 'call'), declared)
        with tempfile.TemporaryDirectory(prefix='library-surface-php-') as folder:
            root = Path(folder)
            for name in ['sdlc', 'specification', 'conformance']:
                (root / name).symlink_to(ROOT / name, target_is_directory=True)
            shutil.copytree(ROOT / 'libraries/php', root / 'libraries/php', ignore=shutil.ignore_patterns('target'))
            target = root / 'libraries/c/include/thinkthen.h'
            target.parent.mkdir(parents=True)
            shutil.copyfile(ROOT / 'libraries/c/include/thinkthen.h', target)
            manifest = json.loads((ROOT / 'conformance/library-surface.json').read_text())
            manifest['libraries']['php']['function_containers'] = ['Client']
            manifest['libraries']['php']['option_containers'] = ['Options']
            manifest['libraries']['php']['setting_containers'] = ['Settings']
            selected = root / 'manifest.json'
            selected.write_text(json.dumps(manifest))
            calls = root / 'libraries/php/src/session/inputs_generated.php'
            original = calls.read_text()
            def invoke(source):
                calls.write_text(source)
                result = subprocess.run(['python3', str(CHECK), '--root', str(root), '--manifest',
                                         str(selected), '--library', 'php', '--json'],
                                        capture_output=True, text=True, env=child_env(home=folder))
                self.assertIn(result.returncode, [0, 1], result.stderr)
                return json.loads(result.stdout)['libraries']['php']
            baseline = invoke(original)
            self.assertFalse(any(error.startswith('missing function:') for error in baseline), baseline)
            errors = invoke(original.replace('trait NamedCalls\n{', 'trait NamedCalls\n{ public function extraPublicExport() {}', 1))
            self.assertIn('unlisted generated declaration: inputs_generated.php:NamedCalls.extraPublicExport', errors)
            self.assertIn('unlisted public call/export: NamedCalls.extraPublicExport', errors)
            errors = invoke(original.replace('function decide(', 'function missingDecide(', 1))
            self.assertIn('missing function: decide', errors)

    def test_native_declarations_exclude_comments_and_private_fields(self):
        spec = importlib.util.spec_from_file_location('surface_cli', CHECK)
        checker = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(checker)
        cases = {
            'python': 'class RequestOptions:\n    context: str\n    def extraPublicExport(self): pass\n',
            'typescript': 'export interface RequestOptions { context: string; } export function extraPublicExport(): void;',
            'java': 'public class RequestOptions { public String context; private String hidden; public void extraPublicExport() {} }',
            'kotlin': 'class RequestOptions {\nval context: String\nprivate val hidden: String\nfun extraPublicExport() {}\n}',
            'scala': 'class RequestOptions {\nval context: String\nprivate val hidden: String\ndef extraPublicExport(): Unit = {}\n}',
            'csharp': 'public class RequestOptions { public string Context {get;set;} private string hidden; public void extraPublicExport() {} }',
            'go': 'type RequestOptions struct {\nContext string\nhidden string\n}\nfunc ExtraPublicExport() {}',
            'swift': 'public struct RequestOptions {\npublic var context: String\nprivate var hidden: String\npublic func extraPublicExport() {}\n}',
            'cpp': 'class RequestOptions { public: std::string context; void extraPublicExport(); private: std::string hidden; };',
            'c': 'typedef struct RequestOptions { const struct Text *context; } RequestOptions; struct Result *extraPublicExport(void);',
            'zig': 'pub const RequestOptions = struct {\ncontext: []const u8,\npub fn extraPublicExport() void {}\n};',
            'php': 'class RequestOptions { public string $context; private string $hidden; public function extraPublicExport() {} }',
            'dart': 'class RequestOptions {\nString get context => value;\nString _hidden;\nvoid extraPublicExport() {}\n}',
            'objc': '@interface RequestOptions : NSObject\n@property(nonatomic) NSString *context;\n- (void)extraPublicExport;\n@end',
            'ada': 'package Surface is\ntype RequestOptions is record\ncontext : String;\nend record;\nprocedure ExtraPublicExport (Value : String);\nend Surface;',
            'ruby': 'class RequestOptions\n  attr_reader :context\n  def extraPublicExport; end\nend',
            'r': 'export(extraPublicExport)\nRequestOptions <- function(context = NULL) {}',
            'cobol': '01 REQUESTOPTIONS.\n05 CONTEXT PIC X(20).\n01 EXTRA-PUBLIC-EXPORT PIC X.\n',
        }
        for language, source in cases.items():
            with self.subTest(language=language):
                declarations = checker.READ.declarations(source, language)
                names = {checker.canonical(item.name) for item in declarations}
                self.assertIn('context', names)
                self.assertIn('extra_public_export', names)
                self.assertNotIn('hidden', names)

    def test_jvm_facades_cannot_borrow_missing_calls_or_method_helpers(self):
        spec = importlib.util.spec_from_file_location('surface_cli_facades', CHECK)
        checker = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(checker)
        manifest = json.loads((ROOT / 'conformance/library-surface.json').read_text())
        config = manifest['libraries']['jvm']
        declared = set()
        for language, container in [('java', 'Engine'), ('kotlin', 'KotlinEngine'), ('scala', 'ScalaEngine')]:
            for name in ['decide', 'plan', 'usagePersistence', 'finishUsageStatus']:
                declared.add(checker.READ.Declaration(container, name, 'call'))
        self.assertEqual(checker.facade_errors(declared, ['decide'], manifest, config), [])
        for container, facade in [('KotlinEngine', 'kotlin'), ('ScalaEngine', 'scala')]:
            with self.subTest(facade=facade):
                missing = declared - {checker.READ.Declaration(container, 'decide', 'call'),
                                      checker.READ.Declaration(container, 'plan', 'call')}
                errors = checker.facade_errors(missing, ['decide'], manifest, config)
                self.assertIn('missing function: decide (' + facade + ')', errors)
                self.assertIn('missing helper: plan (' + facade + ')', errors)

    def test_result_getters_cannot_replace_input_builders(self):
        spec = importlib.util.spec_from_file_location('surface_cli_builders', CHECK)
        checker = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(checker)
        manifest = json.loads((ROOT / 'conformance/library-surface.json').read_text())
        manifest['helpers'] = {'question_file': 'builder', 'question_name': 'builder',
                               'question_reference': 'builder', 'item': 'builder', 'files': 'builder'}
        config = manifest['libraries']['go']
        source = ('type OwnedAtomic struct {}\n'
                  'func (v OwnedAtomic) QuestionName() string { return "unused" }\n'
                  'type RequestQuestionFile struct { Path string }\n'
                  'type RequestQuestionName struct { Name string }\n'
                  'type RequestQuestionReference struct { Reference string }\n'
                  'type RequestItem struct { Original string }\n'
                  'type RequestSource struct { Paths []string }\n')
        declared = checker.READ.declarations(source, 'go')
        self.assertEqual(checker.helper_errors(declared, manifest, config), [])
        for name, helper in [('RequestQuestionFile', 'question_file'), ('RequestQuestionName', 'question_name'),
                             ('RequestQuestionReference', 'question_reference'), ('RequestItem', 'item'), ('RequestSource', 'files')]:
            with self.subTest(helper=helper):
                missing = {d for d in declared if d.name != name}
                self.assertIn('missing helper: ' + helper, checker.helper_errors(missing, manifest, config))
        fake_fields = {checker.READ.Declaration('OwnedAtomic', name, 'field')
                       for name in config['helper_names'].values()}
        self.assertEqual(len(checker.helper_errors(fake_fields, manifest, config)), 5)
        manifest['helpers'] = {'plan': 'call', 'usage_persistence': 'call'}
        getters = checker.READ.declarations('type OwnedFacts struct {}\n'
                    'func (v OwnedFacts) Plan() string { return \"unused\" }\n'
                    'func (v OwnedFacts) UsagePersistence() string { return \"unused\" }\n', 'go')
        self.assertEqual(checker.helper_errors(getters, manifest, config),
                         ['missing helper: plan', 'missing helper: usage_persistence'])

    def test_public_enum_members_and_constants_remain_visible(self):
        spec = importlib.util.spec_from_file_location('surface_cli_enums', CHECK)
        checker = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(checker)
        cases = {
            'java': 'public enum State { FIRST("first"), SECOND("second"); private State(String s) {} }',
            'csharp': 'public enum State { FIRST = 1, SECOND = 2 }',
            'cpp': 'enum class State { FIRST = 1, SECOND = 2 };',
            'typescript': 'export enum State { FIRST = 1, SECOND = 2 }',
            'dart': 'enum State { FIRST, SECOND }',
            'zig': 'pub const State = enum { FIRST, SECOND };',
            'php': 'enum State: int { case FIRST = 1; case SECOND = 2; }',
        }
        for language, source in cases.items():
            with self.subTest(language=language):
                actual = checker.READ.declarations(source, language)
                self.assertTrue({checker.READ.Declaration('State', member, 'field')
                                 for member in ['FIRST', 'SECOND']} <= actual)
        actual = checker.READ.declarations('const EXPORTED = 1; class Private { private const HIDDEN = 2; }', 'php')
        self.assertIn(checker.READ.Declaration('', 'EXPORTED', 'export'), actual)
        self.assertFalse(any(d.name == 'HIDDEN' for d in actual))


if __name__ == '__main__':
    unittest.main()
