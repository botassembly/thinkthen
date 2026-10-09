"""Installed public MCP behavior using one owned offline backend and scratch home."""
import base64
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import threading
import unittest
from client import Client, CancelledError, ProtocolError, ToolError

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
BINARY, BACKEND = sys.argv[1:3]
# unittest sees only its own arguments.
sys.argv[1:] = []


class Installed(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(prefix='thinkthen-mcp-behavior-')
        self.home = Path(self.scratch.name)
        self.env = child_env(home=self.home, LANG='C.UTF-8',
                             THINKTHEN_API_KEY='sk-mcp-loopback-only')
        self.backend = subprocess.Popen((BACKEND,), stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        stderr=subprocess.DEVNULL, text=True, env=self.env)
        self.port = self.backend.stdout.readline().strip()
        self.assertTrue(self.port.isdecimal())

    def tearDown(self):
        self.backend.stdin.close()
        self.backend.wait(timeout=5)
        self.backend.stdout.close()
        self.scratch.cleanup()

    def backend_line(self, line):
        self.backend.stdin.write(line + '\n')
        self.backend.stdin.flush()

    def count(self):
        self.backend_line('count')
        return int(self.backend.stdout.readline())

    def launch(self, route='generic', *options):
        return Client.launch((BINARY, 'mcp', '--url', f'http://127.0.0.1:{self.port}/{route}/v1',
                              '--max-retries', '0', *options), env=self.env)

    def test_repeated_image_rows_hit_aggregate_bound_before_later_paths_or_sends(self):
        image = self.home / 'image.png'
        image.write_bytes((ROOT / 'specification/fixtures/images/above-spike.png').read_bytes())
        rows = [{'images': [str(image)]} for _ in range(160)]
        rows.append({'images': [str(self.home / 'absent.png')]})
        with self.launch('generic', '--no-cache') as client:
            with self.assertRaises(ToolError) as failure:
                client.decide(question='q', inputs=rows)
            self.assertEqual(failure.exception.result['error']['kind'], 'usage')
            self.assertEqual(failure.exception.result['error']['message'],
                             'retained attachments exceed the input byte ceiling')
            self.assertEqual(client.ping(), {})
            # A later selector still reaches its ordinary local refusal after this call releases its budget.
            with self.assertRaises(ToolError) as missing:
                client.decide(question_file=str(self.home / 'absent.json'), evidence='x')
            self.assertEqual(missing.exception.result['error']['kind'], 'local')
        self.assertEqual(self.count(), 0)

    @unittest.skipUnless(sys.platform.startswith('linux'), 'owned FIFO/stdin cases use Linux')
    def test_nonregular_selectors_refuse_and_next_framed_tool_remains_usable(self):
        fifo = self.home / 'question.fifo'
        os.mkfifo(fifo)
        with self.launch('generic', '--no-cache') as client:
            process = client.process
            for path in (fifo, Path('/dev/stdin'), self.home):
                with self.subTest(path=path):
                    with self.assertRaises(ToolError) as failure:
                        client.decide(question_file=str(path), evidence='x')
                    self.assertEqual(failure.exception.result['error']['kind'], 'local')
                    self.assertEqual(failure.exception.result['error']['message'],
                                     'the question file could not be read')
                    result = client.decide(question='q', evidence='x')
                    self.assertIsInstance(result['value']['value'], bool)
                    self.assertEqual(client.ping(), {})
        self.assertEqual(process.returncode, 0)
        self.assertEqual(self.count(), 3)

    def test_protocol_errors_preserve_readable_ids_and_omit_unreadable_ids(self):
        cases = [
            ('{"jsonrpc":"1.0","id":"readable","method":"PRIVATE"}\n', 'readable', -32600),
            ('{"jsonrpc":"2.0","id":-7,"method":false}\n', -7, -32600),
            ('{"jsonrpc":"2.0","id":18446744073709551615,"method":"ping","extra":"PRIVATE"}\n',
             18446744073709551615, -32600),
            ('["2.0",1,"ping",{}]\n', None, -32600),
            ('["readable"]\n', None, -32600),
            ('{"jsonrpc":"2.0","id":null,"method":"ping"}\n', None, -32600),
            ('{"jsonrpc":"2.0","id":true,"method":"ping"}\n', None, -32600),
            ('{"jsonrpc":"2.0","id":1.5,"method":"ping"}\n', None, -32600),
            ('{"jsonrpc":"2.0","id":1,"id":2,"method":"ping"}\n', None, -32600),
            ('{"jsonrpc":"2.0","id":3,"method":\n', None, -32700),
        ]
        with self.launch() as client:
            for wire, ident, code in cases:
                with self.subTest(wire=wire):
                    client.writer.write(wire.encode())
                    client.writer.flush()
                    reply = client._responses.get(timeout=3)
                    self.assertIsInstance(reply, dict)
                    self.assertEqual(reply['error']['code'], code)
                    if ident is None:
                        self.assertNotIn('id', reply)
                    else:
                        self.assertIs(type(reply['id']), type(ident))
                        self.assertEqual(reply['id'], ident)
                    self.assertNotIn('PRIVATE', json.dumps(reply))
            self.assertEqual(client.ping(), {})
        self.assertEqual(self.count(), 0)

    def test_dynamic_explicit_field_overrides_saved_on_without_disclosing_private_evidence(self):
        from http.server import BaseHTTPRequestHandler, HTTPServer
        captured = []
        class Handler(BaseHTTPRequestHandler):
            def do_POST(self):
                captured.append(json.loads(self.rfile.read(int(self.headers['Content-Length']))))
                body = json.dumps({'model': 'selected', 'answers': {'q1': {
                    'type': 'choice', 'probabilities': {'zebra': 0.9, 'alpha': 0.1}}}}).encode()
                self.send_response(200)
                self.send_header('Content-Length', str(len(body)))
                self.send_header("Connection", "close")
                self.end_headers()
                self.wfile.write(body)
            def log_message(self, *_):
                pass
        server = HTTPServer(('127.0.0.1', 0), Handler)
        worker = threading.Thread(target=lambda: server.serve_forever(poll_interval=0.05))
        worker.start()
        record = {'public': 'Only selected evidence.', 'private': 'PRIVATE-SAVED-EVIDENCE',
                  'context': {'ready': False}, 'choices': ['zebra', 'alpha']}
        question = {'choose': 'Which?', 'on': '/private', 'model': 'old', 'threshold': 0.95,
                    'name': 'override', 'wording_version': 7, 'item_schema': {'type': 'string'},
                    'context_schema': {'type': 'object', 'properties': {}}}
        command = (BINARY, 'mcp', '--url', f'http://127.0.0.1:{server.server_port}/v1',
                   '--no-cache', '--max-retries', '0')
        try:
            with Client.launch(command, env=self.env) as client:
                reply = client.choose(question=question, records=[record], options={
                    'field': '/public', 'options_field': '/choices', 'context_field': '/context',
                    'model': 'selected', 'batch': 1})
                row = reply['value'][0]
                self.assertEqual(row['input'], record)
                self.assertIsNone(row['value'])
                self.assertEqual(row['question']['name'], 'override')
                self.assertEqual(row['question']['wording_version'], 7)
                self.assertEqual(row['question']['item_schema'], {'type': 'string'})
                self.assertEqual(row['question']['context_schema'], question['context_schema'])
                self.assertEqual(row['threshold'], 0.95)
                self.assertEqual(reply['facts']['requests_sent'], 1)
        finally:
            server.shutdown()
            worker.join(timeout=2)
            server.server_close()
        self.assertFalse(worker.is_alive())
        self.assertEqual(len(captured), 1)
        self.assertEqual(captured[0]['model'], 'selected')
        self.assertEqual(captured[0]['state'], {'ready': False})
        self.assertEqual(captured[0]['questions']['q1']['instructions'],
                         'The text is "Only selected evidence.". Which?')
        self.assertNotIn('PRIVATE-SAVED-EVIDENCE', json.dumps(captured))
        self.assertEqual(self.count(), 0)

    def test_materialized_arrays_admit_every_atomic_item_and_annotation_member_before_sends(self):
        questions = {'decide': {'decide': 'Q?'}, 'choose': {'choose': 'Q?', 'options': ['a', 'b']},
                     'tag': {'tag': 'Q?', 'labels': ['a', 'b']}, 'score': {'score': 'Q?', 'levels': ['a', 'b']},
                     'filter': {'decide': 'Q?'}}
        with self.launch('generic', '--no-cache', '--jobs', '1') as client:
            for name, question in questions.items():
                declared = dict(question, item_schema={'type': 'string'})
                with self.assertRaises(ToolError) as failure:
                    getattr(client, name)(question=declared, records=['first', 'second', False],
                                          options={'batch': 1})
                self.assertEqual(failure.exception.result['error']['kind'], 'usage')
                self.assertEqual(failure.exception.result['error']['message'], 'the item does not match item_schema')
                self.assertEqual(self.count(), 0)
            checks = {'version': 1, 'questions': {'valid': {'decide': 'Q?'},
                      'declared': {'decide': 'Q?', 'item_schema': {'type': 'string'}}}}
            with self.assertRaises(ToolError) as failure:
                client.annotate(question=checks, records=['first', 'second', False], options={'batch': 1})
            self.assertEqual(failure.exception.result['error']['kind'], 'usage')
            self.assertEqual(failure.exception.result['error']['message'], 'the item does not match item_schema')
            self.assertEqual(self.count(), 0)
            source = self.home / 'incremental.jsonl'
            source.write_text('{"body":"first"}\n{"body":"second"}\n{"body":false}\n')
            with self.assertRaises(ToolError) as failure:
                client.decide(question={'decide': 'Q?', 'item_schema': {'type': 'string'}},
                              source={'paths': [str(source)], 'unit': 'line'},
                              options={'field': '/body', 'batch': 1})
            self.assertEqual(failure.exception.result['error']['kind'], 'usage')
            self.assertEqual(self.count(), 2)
            prefix = failure.exception.result['completed']
            self.assertEqual([row['index'] for row in prefix], [0, 1])
            self.assertTrue(all(len(row['answer_id']) == 64 for row in prefix))
            self.assertEqual(failure.exception.result['facts']['records'], 2)
            self.assertEqual(failure.exception.result['facts']['requests_sent'], self.count())

    def test_both_source_rank_branches_bound_original_bytes_before_projection(self):
        paths = [self.home / f'large-{n}.json' for n in range(2)]
        content = json.dumps({'public': 'x', 'private': 'z' * (8 * 1024 * 1024)})
        for path in paths:
            path.write_text(content)
        tail = self.home / 'unread-tail.txt'
        tail.write_bytes(b'\xff')
        with self.launch('generic', '--no-cache') as client:
            for question in ('Q?', {'version': 1, 'questions': {'q': {'decide': 'Q?'}}}):
                with self.assertRaises(ToolError) as failure:
                    client.rank(question=question, source={'paths': [str(p) for p in (*paths, tail)], 'unit': 'file'},
                                options={'field': '/public', 'batch': 1})
                self.assertEqual(failure.exception.result['error']['kind'], 'usage')
                self.assertEqual(failure.exception.result['error']['message'],
                                 'source rank reads at most 16 MiB across all input records')
                self.assertEqual(self.count(), 0)

    def test_file_size_limit_reports_recording_and_cache_failure_and_keeps_owned_session_alive(self):
        for mode in ('record', 'cache'):
            folder = self.home / f'limited-{mode}'
            before = self.count()
            storage = ('--no-cache', '--record', str(folder)) if mode == 'record' else ('--cache', str(folder))
            command = ('/usr/bin/prlimit', '--fsize=1024:1024', '--', BINARY, 'mcp', '--url',
                       f'http://127.0.0.1:{self.port}/generic/v1', '--max-retries', '0', *storage)
            process = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                       stderr=subprocess.PIPE, env=self.env)
            client = Client(process.stdin, process.stdout, process)
            try:
                client.initialize()
                with self.assertRaises(ToolError) as failure:
                    client.decide(question='Q?', evidence='x' * 8192)
                self.assertEqual(failure.exception.result['error']['kind'], 'local')
                self.assertEqual(failure.exception.result['error']['message'], 'the recording folder could not be written')
                self.assertEqual(failure.exception.result['facts']['requests_sent'], 1)
                self.assertEqual(self.count() - before, 1)
                self.assertEqual(client.ping(), {})
            finally:
                process.stdin.close()
                self.assertEqual(process.wait(timeout=3), 0)
                diagnostics = process.stderr.read().decode()
                process.stderr.close()
                client.close()
            self.assertNotIn('sk-mcp-loopback-only', diagnostics)
            # The native store may remain; no transient write or journal survives.
            self.assertEqual(sorted(p.name for p in folder.iterdir()),
                             ['thinkthen.sqlite'])

    def test_names_files_and_literal_at_text_use_the_native_loaders(self):
        directory = self.home / 'config/thinkthen/questions'
        directory.mkdir(parents=True)
        q = {'decide': 'Refund?', 'name': 'refund', 'wording_version': 2}
        path = directory / 'refund.json'
        path.write_text(json.dumps(q))
        with self.launch('generic', '--no-cache') as client:
            named = client.decide(question_name='refund', evidence='Please refund.')
            self.assertEqual(named['value']['question']['name'], 'refund')
            self.assertEqual(named['value']['question']['wording_version'], 2)
            file = client.decide(question_file=str(path), evidence='Please refund.')
            self.assertEqual(named['value']['question'], file['value']['question'])
            self.assertIs(file['value']['value'], True)
            if os.name == 'posix':
                link = self.home / 'question-link.json'
                link.symlink_to(path)
                linked = client.decide(question_reference='@' + str(link), evidence='Please refund.')
                self.assertEqual(linked['value']['question'], file['value']['question'])
            literal = client.decide(question='@refund', evidence='Please refund.')
            self.assertEqual(literal['value']['question']['text'], '@refund')
            before = self.count()
            with self.assertRaises(ToolError) as failure:
                client.decide(question='q', question_name='refund', evidence='x')
            self.assertNotIn('facts', failure.exception.result)
            self.assertEqual(self.count(), before)
        self.assertEqual(json.loads(path.read_text()), q)

    def test_native_fields_context_and_candidate_order_preserve_originals(self):
        record = {'body': 'Please refund.', 'context': {'ready': False}, 'options': {'zebra': 'last', 'alpha': 'first'},
                  'private': 'unsent'}
        with self.launch('generic', '--no-cache') as client:
            result = client.choose(question={'choose': 'Which?', 'context_schema': {'type': 'object', 'properties': {}}},
                                   records=[record], options={'field': '/body', 'context_field': '/context',
                                                              'options_field': '/options', 'batch': 1})
            row = result['value'][0]
            self.assertEqual(row['input'], record)
            self.assertEqual(list(row['answer']['probabilities']), ['zebra', 'alpha'])
            self.assertEqual(row['question']['context_schema'], {'type': 'object', 'properties': {}})
            schema = {'type': 'object', 'properties': {}}
            questions = {'version': 1, 'questions': {'q': {'decide': 'Q?', 'context_schema': schema}}}
            for method in (client.annotate, client.rank):
                typed = method(question=questions, records=[record],
                               options={'field': '/body', 'context_field': '/context', 'batch': 1})
                self.assertEqual(typed['value'][0]['input'], record)
                self.assertEqual(typed['facts']['requests_sent'], 1)
            explicit = client.choose(question={'choose': 'Which?', 'context_schema': schema},
                                     inputs=[{'json': record, 'context': {'ready': False},
                                              'options': ['zebra', 'alpha']}],
                                     options={'field': '/body', 'batch': 1})
            self.assertEqual(explicit['value'][0]['input'], record)
            self.assertEqual(list(explicit['value'][0]['answer']['probabilities']), ['zebra', 'alpha'])
            self.assertEqual(explicit['value'][0]['index'], 0)
            self.assertEqual(self.count(), 4)
            for inputs in ([{'text': 'first'}, {'text': 'later', 'context': None}],
                           [{'json': record, 'context': 'c'}]):
                before = self.count()
                with self.assertRaises(ToolError) as refusal:
                    client.decide(question='Q?', inputs=inputs,
                                  options={'context_field': '/context'} if len(inputs) == 1 else {})
                self.assertEqual(refusal.exception.result['error']['kind'], 'usage')
                self.assertEqual(self.count(), before)

    def test_dynamic_saved_on_pointer_uses_native_composition(self):
        record = {'body': 'x', 'options': ['red', 'blue']}
        with self.launch('generic', '--no-cache') as client:
            reply = client.choose(question={'choose': 'Which?', 'on': '/body'}, records=[record],
                                  options={'options_field': '/options'})
            self.assertEqual(reply['value'][0]['input'], record)
            self.assertEqual(reply['value'][0]['value'], 'red')
            self.assertEqual(self.count(), 1)

    def test_dynamic_explicit_model_replaces_the_saved_model(self):
        with self.launch('generic', '--no-cache') as client:
            reply = client.choose(question={'choose': 'Which?', 'model': 'old'},
                                  records=[{'body': 'x', 'options': ['red', 'blue']}],
                                  options={'field': '/body', 'options_field': '/options', 'model': 'selected'})
            self.assertEqual(reply['value'][0]['meta']['model'], 'selected')
            self.assertEqual(self.count(), 1)

    def test_saved_rank_find_sets_and_explicit_controls_execute_native_methods(self):
        path = self.home / 'records.jsonl'
        path.write_text('{"body":"a","id":1}\n')
        with self.launch('generic', '--no-cache') as client:
            projected = client.decide(question={'decide': 'Q?', 'on': '/body'},
                                      source={'paths': [str(path)], 'unit': 'line'})
            self.assertEqual(projected['value'][0]['input'], {'body': 'a', 'id': 1})
            ranked = client.rank(question={'score': 'Grade?', 'levels': ['low', 'high']},
                                 records=['a', 'b'], options={'batch': 1, 'top': 1})
            self.assertEqual(len(ranked['value']), 1)
            self.assertEqual(ranked['value'][0]['value'], 1)
            set_rank = client.rank(question={'version': 1, 'questions': {'z': {'decide': 'Q?'}}},
                                   records=['a', 'b'], options={'batch': 'max', 'top': 0})
            self.assertEqual(set_rank['value'], [])
            full_set = client.rank(question={'version': 1, 'questions': {'z': {'decide': 'Q?'}}},
                                   records=['a', 'b'], options={'batch': 'max'})
            self.assertEqual([row['index'] for row in full_set['value']], [0, 1])
            self.assertEqual([row['value'] for row in full_set['value']], [1, 2])
            self.assertEqual([row['question_name'] for row in full_set['value']], ['z', 'z'])
            self.assertEqual([row['answer']['probability'] for row in full_set['value']], [0.9, 0.9])
            self.assertTrue(all(len(row['answer_id']) == 64 for row in full_set['value']))
            found = client.find(question={'find': 'Which?', 'on': '/body'},
                                records=[{'body': 'a', 'id': 1}, {'body': 'b', 'id': 2}])
            self.assertEqual(found['value']['value'], {'body': 'a', 'id': 1})
            decision = client.decide(question={'decide': 'Q?', 'model': 'old'}, evidence='x',
                                     options={'model': 'selected', 'threshold': '0.95:1', 'attempts': True})
            self.assertIs(decision['value']['value'], False)
            self.assertTrue(decision['facts']['attempts'])
            before = self.count()
            with self.assertRaises(ToolError) as initial:
                client.decide(question='Q?', evidence='private-cancel-evidence', options={'cancelled': True})
            self.assertEqual(initial.exception.result['error']['kind'], 'cancelled')
            self.assertNotIn('facts', initial.exception.result)
            self.assertNotIn('private-cancel-evidence', str(initial.exception.result))
            self.assertEqual(self.count(), before)
            for invalid in ('true', None, 1):
                with self.assertRaises(ProtocolError):
                    client.decide(question='Q?', evidence='x', options={'cancelled': invalid})
                self.assertEqual(self.count(), before)
            active = client.decide(question='Q?', evidence='x', options={'cancelled': False})
            self.assertIs(active['value']['value'], True)
            self.assertEqual(active['facts']['requests_sent'], 1)
            self.assertEqual(self.count(), before + 1)

    def test_file_folder_text_windows_and_files_only_keep_physical_locations(self):
        folder = self.home / 'inputs'
        folder.mkdir()
        (folder / 'b.txt').write_text('b1\nb2\n')
        (folder / 'a.txt').write_text('a1\na2\n')
        with self.launch('generic', '--no-cache') as client:
            result = client.decide(question='Q?', source={'paths': [str(folder)], 'unit': 'window', 'window': 2},
                                   options={'batch': 1})
            self.assertEqual([Path(row['source']['file']).name for row in result['value']], ['a.txt', 'b.txt'])
            self.assertEqual([(r['source']['first_line'], r['source']['last_line']) for r in result['value']], [(1, 2), (1, 2)])
            filtered = client.filter(question='Q?', source={'paths': [str(folder)], 'unit': 'line'},
                                     options={'files_only': True, 'batch': 1})
            self.assertEqual([Path(row['source']['file']).name for row in filtered['value']], ['a.txt', 'b.txt'])

    def test_admitted_images_keep_original_order_duplicates_and_zero_send_replay(self):
        images = [ROOT / 'specification/fixtures/images' / name for name in ('red.png', 'blue.png', 'red.png')]
        recording = self.home / 'images-recording'
        questions = {'decide': 'Is red visible?',
                     'choose': {'choose': 'Which?', 'options': ['red', 'blue']},
                     'score': {'score': 'How red?', 'levels': ['none', 'all']}}
        with self.launch('generic', '--backend', 'liquid', '--model', 'd1', '--no-cache',
                         '--record', str(recording)) as client:
            for name, question in questions.items():
                reply = getattr(client, name)(question=question, evidence='Compare originals.',
                                              images=[str(p) for p in images])
                original = reply['value']['input']
                self.assertEqual(original['text'], 'Compare originals.')
                self.assertEqual([base64.b64decode(i['base64']) for i in original['images']],
                                 [p.read_bytes() for p in images])
                self.assertEqual(reply['facts']['requests_sent'], 1)
            source = client.decide(question='Is red visible?',
                                   source={'paths': [str(images[0])], 'unit': 'file', 'media': 'image'})
            location = source['value'][0]['input']['location']
            self.assertEqual(location['file'], str(images[0]))
            self.assertNotIn('first_line', location)
            self.assertNotIn('last_line', location)
        before = self.count()
        self.assertEqual(before, 4)
        env = dict(self.env)
        env.pop('THINKTHEN_API_KEY')
        command = (BINARY, 'mcp', '--backend', 'liquid', '--model', 'd1', '--url',
                   f'http://127.0.0.1:{self.port}/generic/v1', '--replay', str(recording), '--no-cache')
        with Client.launch(command, env=env) as client:
            for name, question in questions.items():
                reply = getattr(client, name)(question=question, evidence='Compare originals.',
                                              images=[str(p) for p in images])
                self.assertEqual(reply['facts']['requests_sent'], 0)
                self.assertEqual(reply['value']['meta']['origin'], 'replay')
            with self.assertRaises(ToolError):
                client.decide(question=questions['decide'], evidence='Compare originals.',
                              images=[str(images[1]), str(images[0]), str(images[0])])
        self.assertEqual(self.count(), before)

    def test_every_tool_refuses_credentials_images_and_started_failures_safely(self):
        document = json.loads((ROOT / 'conformance/cases.json').read_text())
        with self.launch('arm/refuse', '--no-cache') as client:
            from conformance import arguments, fixture
            import parity
            by_id = {c['id']: c for c in document['cases']}
            for name in document['parity']['functions']:
                method = getattr(client, name)
                before = self.count()
                with self.assertRaises(Exception) as rejected:
                    method(question='SECRET-QUESTION', evidence='SECRET-EVIDENCE', api_key='SECRET-KEY')
                self.assertNotIn('SECRET', str(rejected.exception))
                self.assertEqual(self.count(), before)
                row = next(row for row in parity.required_cases(parity.inventory(), 'mcp').values()
                           if row['verb'] == name and row['kind'] == 'behavior')
                case = fixture(row, by_id, {})
                with self.assertRaises(ToolError) as failure:
                    method(**arguments(case, self.home, {'batch': 1}))
                result = failure.exception.result
                self.assertEqual(result['error']['kind'], 'backend', (name, result))
                self.assertGreater(result['facts']['requests_sent'], 0)
                self.assertNotIn('sk-mcp-loopback-only', json.dumps(result))
            for name in ('tag', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate'):
                before = self.count()
                with self.assertRaises(ToolError):
                    getattr(client, name)(question='Q?', images=[str(self.home / 'missing.png')])
                self.assertEqual(self.count(), before)
            before = self.count()
            with self.assertRaises(ToolError) as failure:
                client.decide(question='Q?', images=[str(self.home / 'missing.png')] * 9)
            self.assertEqual(failure.exception.result['error']['kind'], 'usage')
            self.assertEqual(failure.exception.result['error']['message'], 'image evidence requires 1 to 8 images')
            self.assertEqual(self.count(), before)

    def test_default_cache_record_and_strict_replay_preserve_zero_sends(self):
        with self.launch() as client:
            first = client.decide(question='Q?', evidence='x')
            cached = client.decide(question='Q?', evidence='x')
            self.assertEqual(first['value']['answer_id'], cached['value']['answer_id'])
            self.assertEqual(cached['value']['meta']['origin'], 'cache')
            self.assertEqual(cached['facts']['requests_sent'], 0)
            self.assertEqual(self.count(), 1)
        with self.launch('generic', '--refresh-cache') as client:
            refreshed = client.decide(question='Q?', evidence='x')
            self.assertEqual(refreshed['facts']['requests_sent'], 1)
            self.assertEqual(self.count(), 2)
        recording = self.home / 'recording'
        with self.launch('generic', '--record', str(recording), '--no-cache') as client:
            client.decide(question='Recorded?', evidence='x')
        before = self.count()
        replay_env = dict(self.env)
        replay_env.pop('THINKTHEN_API_KEY')
        command = (BINARY, 'mcp', '--url', f'http://127.0.0.1:{self.port}/generic/v1',
                   '--replay', str(recording), '--no-cache')
        with Client.launch(command, env=replay_env) as client:
            replay = client.decide(question='Recorded?', evidence='x')
            self.assertEqual(replay['value']['meta']['origin'], 'replay')
            self.assertEqual(replay['facts']['requests_sent'], 0)
            with self.assertRaises(ToolError) as miss:
                client.decide(question='Absent?', evidence='x')
            self.assertEqual(miss.exception.result['error']['kind'], 'local')
        self.assertEqual(self.count(), before)

    def test_invalid_text_deadline_and_budgets_send_nothing(self):
        with self.launch('generic', '--no-cache') as client:
            for args in ({'question': {'decide': 'Q?', 'on': '/payload', 'item_schema': {'type': 'object', 'properties': {}}},
                          'evidence': '{"payload":{}}'},
                         {'question': 'Q?', 'evidence': 'x', 'options': {'deadline_ms': 0}},
                         {'question': 'Q?', 'evidence': 'x', 'options': {'max_requests_total': 0}}):
                with self.assertRaises(ToolError) as failure:
                    client.decide(**args)
                self.assertNotIn('value', failure.exception.result)
                self.assertEqual(self.count(), 0)

    def test_cancelled_sent_call_joins_and_the_same_session_continues(self):
        with self.launch('arm/held', '--no-cache') as client:
            outcome = []
            def call():
                try:
                    client.decide(question='Q?', evidence='x')
                except CancelledError:
                    outcome.append('cancelled')
            worker = threading.Thread(target=call)
            worker.start()
            self.backend_line('wait 1')
            self.assertEqual(self.backend.stdout.readline().strip(), 'wait 1')
            client.cancel(client.pending_id)
            self.backend_line('release')
            worker.join(timeout=3)
            self.assertFalse(worker.is_alive())
            self.assertEqual(outcome, ['cancelled'])
            self.assertEqual(client.ping(), {})
            reply = client.decide(question='Next?', evidence='x')
            self.assertIs(reply['value']['value'], True)
            self.assertEqual(self.count(), 2)

    def test_eof_refuses_another_call_and_joins_the_owned_started_attempt(self):
        for final in (b'\n', b''):
            with self.subTest(final=final):
                before = self.count()
                client = self.launch('arm/held', '--no-cache')
                try:
                    process = client.process
                    client._send({'jsonrpc': '2.0', 'id': 100, 'method': 'tools/call',
                                  'params': {'name': 'decide', 'arguments': {'question': 'Q?', 'evidence': 'x'}}})
                    self.backend_line(f'wait {before + 1}')
                    self.assertEqual(self.backend.stdout.readline().strip(), f'wait {before + 1}')
                    # A complete call is refused while the first attempt is held.
                    # Without LF, even complete JSON is an incomplete transport frame.
                    pending = {'jsonrpc': '2.0', 'id': 101, 'method': 'tools/call',
                               'params': {'name': 'decide', 'arguments': {'question': 'Next?', 'evidence': 'x'}}}
                    process.stdin.write(json.dumps(pending).encode() + final)
                    process.stdin.flush()
                    if final:
                        reply = client._responses.get(timeout=3)
                        self.assertEqual(reply['id'], 101)
                        self.assertEqual(reply['error']['code'], -32001)
                    process.stdin.close()
                    self.backend_line('release')
                    self.assertEqual(process.wait(timeout=3), 0 if final else 5)
                    self.assertEqual(self.count(), before + 1)
                finally:
                    client.close()


if __name__ == '__main__':
    unittest.main()
