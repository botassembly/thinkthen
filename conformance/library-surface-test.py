#!/usr/bin/env python3
"""Plant contract omissions and extra exports at the checker CLI boundary."""
import importlib.util
import json
import os
import shutil
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
CHECK = ROOT / 'conformance/library-surface.py'


class PublicSurface(unittest.TestCase):
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
            result = {'$ref': '#/$defs/completesessionPacket', '$defs': {'completesessionPacket': {'type': 'object', 'properties': {'payload': {'type': 'string'}}}}}
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
            spec.loader.exec_module(template)
            generated = template.declarations(result['$defs'])
            write('libraries/typescript/results_generated.d.ts', generated)
            declarations = ('export interface Controls { readonly context?: string; readonly [key:string]: unknown; }\n'
                            'export interface Settings { readonly backend?: string; }\n'
                            'export class Client { decide(): void; }\n')
            declarations += ''.join('export function ' + name + '(): void;\n' for name in ['plan', 'questionFile', 'questionName', 'questionReference', 'item', 'files', 'usagePersistence', 'finishUsageStatus'])
            def invoke(source=declarations, graph=generated):
                write('libraries/typescript/native.d.ts', source)
                write('libraries/typescript/results_generated.d.ts', graph)
                run = subprocess.run(['python3', str(CHECK), '--root', str(root), '--json'],
                                     capture_output=True, text=True,
                                     env=dict(os.environ, HOME=folder))
                return run.returncode, json.loads(run.stdout)['libraries']['typescript']
            code, errors = invoke()
            self.assertEqual((code, errors), (0, []))
            code, errors = invoke(declarations.replace('readonly context?: string;', '') + '\n// context exists in prose only\nexport interface Other { context: string; }')
            self.assertEqual(code, 1)
            self.assertTrue(any(error.startswith('missing option: context') for error in errors))
            code, errors = invoke(declarations + '\nexport function extraPublicExport(): void;')
            self.assertEqual(code, 1)
            self.assertIn('unlisted public call/export: .extraPublicExport', errors)
            code, errors = invoke(declarations.replace('readonly backend?: string;', ''))
            self.assertTrue(any(error.startswith('missing setting: backend') for error in errors))
            code, errors = invoke(declarations.replace('export function plan(): void;', ''))
            self.assertIn('missing helper: plan', errors)
            code, errors = invoke(graph=generated.replace(' readonly "payload"?: string;', ''))
            self.assertTrue(any('missing generated declaration:' in error and '.payload' in error for error in errors))
            code, errors = invoke(graph=generated.replace(' readonly "payload"?: string;', ' readonly "payload"?: string;\n surprise: string;'))
            self.assertTrue(any('unlisted generated declaration:' in error and '.surprise' in error for error in errors))

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
                                     capture_output=True, text=True, env=dict(os.environ, HOME=folder))
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
            'c': 'typedef struct RequestOptions { const char *context; } RequestOptions; void extraPublicExport(void);',
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
