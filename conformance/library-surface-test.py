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
            for filename, owner in [('json.hpp', 'Json'), ('result_value.hpp', 'Node')]:
                path = client.with_name(filename)
                path.write_text(path.read_text().replace('class ' + owner + ' {',
                    'class ' + owner + ' { public: void extraPublicExport(); private:', 1))
            extra = original.replace('class Call {', 'class Call { public: void extraPublicExport(); private:', 1)
            extra = extra.replace('class AnswerCall : public Call {',
                                  'class AnswerCall : public Call { public: void extraPublicExport(); private:', 1)
            errors = invoke(extra)
            for owner in ['Json', 'Node', 'Call', 'AnswerCall']:
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
            generated = template.declarations(generate.prepare(generate.graph(result, template.ROOTS)))
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
                             + 'public class InputRequestOptions { public string SurpriseOption {get;} }\n'
                             + 'public class InputEngineSettings { public string SurpriseSetting {get;} }\n')
            unexpected = invoke(original)
            for method in methods:
                self.assertIn('unlisted public call/export: Other.' + method, unexpected)
            for owner, field in [('Other', 'FactsJson'), ('Other', 'Failure'),
                                 ('InputRequestOptions', 'SurpriseOption'), ('InputEngineSettings', 'SurpriseSetting')]:
                self.assertIn('unlisted public field: ' + owner + '.' + field, unexpected)
            extra.unlink()
            inputs = root / 'libraries/csharp/src/RequestInputs.g.cs'
            input_source = inputs.read_text()
            inputs.write_text(input_source.replace('InputPresence<string> Context {', 'InputPresence<string> SurpriseOption {')
                              .replace('InputPresence<string> Backend {', 'InputPresence<string> SurpriseSetting {'))
            drift = invoke(original)
            for kind, field in [('option', 'context'), ('setting', 'backend')]:
                self.assertTrue(any(error.startswith('missing ' + kind + ': ' + field) for error in drift))
            for owner, before, after in [('InputRequestOptions', 'Context', 'SurpriseOption'),
                                         ('InputEngineSettings', 'Backend', 'SurpriseSetting')]:
                self.assertIn('missing generated declaration: RequestInputs.g.cs:' + owner + '.' + before, drift)
                self.assertIn('unlisted generated declaration: RequestInputs.g.cs:' + owner + '.' + after, drift)
            inputs.write_text(input_source)
            renamed = invoke(original.replace('DecideAsync(', 'SurpriseAsync('))
            self.assertIn('missing generated declaration: NamedCalls.g.cs:Engine.DecideAsync', renamed)
            self.assertIn('unlisted generated declaration: NamedCalls.g.cs:Engine.SurpriseAsync', renamed)
            accessor = invoke(original.replace('Choice(this OwnedCall', 'SurpriseAccessor(this OwnedCall'))
            self.assertIn('missing generated declaration: NamedCalls.g.cs:AnswerAccessors.Choice', accessor)
            self.assertIn('unlisted generated declaration: NamedCalls.g.cs:AnswerAccessors.SurpriseAccessor', accessor)

    def test_java_record_components_are_checked_at_the_cli(self):
        manifest = json.loads((ROOT / 'conformance/library-surface.json').read_text())
        with tempfile.TemporaryDirectory(prefix='library-surface-records-') as folder:
            root = Path(folder)
            for name in ['sdlc', 'specification']:
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
