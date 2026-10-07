"""Installed public MCP behavior using one owned offline backend and scratch home."""
import base64
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import threading
import unittest
from client import Client, CancelledError, ToolError

ROOT = Path(__file__).resolve().parents[2]
BINARY, BACKEND = sys.argv[1:3]
# unittest sees only its own arguments.
sys.argv[1:] = []


class Installed(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(prefix='thinkthen-mcp-behavior-')
        self.home = Path(self.scratch.name)
        self.env = {'PATH': '/usr/bin:/bin', 'LANG': 'C.UTF-8', 'HOME': str(self.home),
                    'XDG_CONFIG_HOME': str(self.home / 'config'), 'XDG_CACHE_HOME': str(self.home / 'cache'),
                    'XDG_STATE_HOME': str(self.home / 'state'), 'THINKTHEN_API_KEY': 'sk-mcp-loopback-only'}
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
            self.assertEqual(self.count(), 3)

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
            found = client.find(question={'find': 'Which?', 'on': '/body'},
                                records=[{'body': 'a', 'id': 1}, {'body': 'b', 'id': 2}])
            self.assertEqual(found['value']['value'], {'body': 'a', 'id': 1})
            decision = client.decide(question={'decide': 'Q?', 'model': 'old'}, evidence='x',
                                     options={'model': 'selected', 'threshold': '0.95:1', 'attempts': True})
            self.assertIs(decision['value']['value'], False)
            self.assertTrue(decision['facts']['attempts'])

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
            from conformance import arguments, CASES
            by_id = {c['id']: c for c in document['cases']}
            for name in document['parity']['functions']:
                method = getattr(client, name)
                before = self.count()
                with self.assertRaises(Exception) as rejected:
                    method(question='SECRET-QUESTION', evidence='SECRET-EVIDENCE', api_key='SECRET-KEY')
                self.assertNotIn('SECRET', str(rejected.exception))
                self.assertEqual(self.count(), before)
                case = next(by_id[c] for c in CASES if by_id[c]['verb'] == name)
                with self.assertRaises(ToolError) as failure:
                    method(**arguments(case))
                result = failure.exception.result
                self.assertEqual(result['error']['kind'], 'backend')
                self.assertGreater(result['facts']['requests_sent'], 0)
                self.assertNotIn('sk-mcp-loopback-only', json.dumps(result))
            for name in ('tag', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate'):
                before = self.count()
                with self.assertRaises(ToolError):
                    getattr(client, name)(question='Q?', images=[str(self.home / 'missing.png')])
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

    def test_eof_joins_the_owned_started_attempt(self):
        client = self.launch('arm/held', '--no-cache')
        process = client.process
        client._send({'jsonrpc': '2.0', 'id': 100, 'method': 'tools/call',
                      'params': {'name': 'decide', 'arguments': {'question': 'Q?', 'evidence': 'x'}}})
        self.backend_line('wait 1')
        self.assertEqual(self.backend.stdout.readline().strip(), 'wait 1')
        process.stdin.close()
        self.backend_line('release')
        self.assertEqual(process.wait(timeout=3), 0)
        client.close()
        self.assertEqual(self.count(), 1)


if __name__ == '__main__':
    unittest.main()
