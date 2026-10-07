"""Independent MCP wire fixtures, not native-result or parity evidence."""
import io
import json
import unittest
import sys
import threading
import time
from pathlib import Path
from client import Client, ProtocolError, ToolError, CancelledError

NAMES = ('decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find',
         'annotate', 'recognize', 'relate')


def wire(ident, value):
    return json.dumps({'jsonrpc': '2.0', 'id': ident, 'result': value}) + '\n'


def result(value, failed=False):
    return {'content': [{'type': 'text', 'text': json.dumps(value)}],
            'structuredContent': value, 'isError': failed}


class Consumer(unittest.TestCase):
    def test_named_calls_send_real_tools_call_and_keep_false_null_partial(self):
        outputs = [wire(1, {'protocolVersion': '2025-11-25',
                           'capabilities': {'tools': {}}, 'serverInfo': {'name': 'fixture', 'version': '1'}})]
        values = [{'value': False}, {'value': None}, {'values': [None], 'failed_questions': 1}]
        outputs.extend(wire(i + 2, result(values[i % 3])) for i in range(len(NAMES)))
        outgoing = io.StringIO()
        client = Client(outgoing, io.StringIO(''.join(outputs)))
        self.addCleanup(client.close)
        client.initialize()
        for i, name in enumerate(NAMES):
            self.assertEqual(getattr(client, name)(question='@literal', evidence='x'), values[i % 3])
        messages = [json.loads(line) for line in outgoing.getvalue().splitlines()]
        self.assertEqual(messages[1], {'jsonrpc': '2.0', 'method': 'notifications/initialized'})
        self.assertEqual([m['params']['name'] for m in messages[2:]], list(NAMES))
        self.assertTrue(all(m['method'] == 'tools/call' for m in messages[2:]))
        self.assertEqual(messages[2]['params']['arguments']['question'], '@literal')

    def test_failed_tool_carries_safe_facts_and_never_becomes_a_value(self):
        native = {'error': {'kind': 'backend', 'message': 'backend refused', 'retryable': False},
                  'facts': {'requests_sent': 1}}
        client = Client(io.StringIO(), io.StringIO(wire(1, result(native, True))))
        self.addCleanup(client.close)
        with self.assertRaises(ToolError) as failure:
            client.decide(question='q', evidence='x')
        self.assertEqual(failure.exception.result, native)

    def test_content_mismatch_wrong_id_and_non_protocol_output_fail(self):
        broken = result({'value': False})
        broken['content'][0]['text'] = '{"value":true}'
        for text in (wire(1, broken), wire(99, result({'value': None})), 'log line\n'):
            with self.subTest(text=text), self.assertRaises(ProtocolError):
                client = Client(io.StringIO(), io.StringIO(text))
                self.addCleanup(client.close)
                client.decide(question='q', evidence='x')

    def test_malformed_initialization_and_tool_catalog_report_safe_protocol_errors(self):
        for value in (False, {'protocolVersion': '2025-11-25', 'capabilities': False},
                      {'protocolVersion': '2025-11-25', 'capabilities': {'tools': {}}}):
            client = Client(io.StringIO(), io.StringIO(wire(1, value)))
            self.addCleanup(client.close)
            with self.assertRaises(ProtocolError):
                client.initialize()
        client = Client(io.StringIO(), io.StringIO(wire(1, {'tools': False})))
        self.addCleanup(client.close)
        with self.assertRaises(ProtocolError):
            client.tools()

    def test_client_cancellation_has_no_payload_or_reason_echo(self):
        outgoing = io.StringIO()
        Client(outgoing, io.StringIO()).cancel(17)
        self.assertEqual(json.loads(outgoing.getvalue()), {'jsonrpc': '2.0',
                         'method': 'notifications/cancelled', 'params': {'requestId': 17}})

    def test_json_types_order_duplicates_and_nonfinite_numbers_are_not_normalized(self):
        for structured, text in [({'value': False}, '{"value":0}'),
                                 ({'value': None}, '{"value":false}'),
                                 ({'zèbre': 1, 'alpha': 2}, '{"alpha":2,"zèbre":1}'),
                                 ({'value': 1}, '{"value":1,"value":1}'),
                                 ({'value': None}, '{"value":NaN}'),
                                 ({'value': None}, '{"value":1e999}')]:
            reply = result(structured)
            reply['content'][0]['text'] = text
            client = Client(io.StringIO(), io.StringIO(wire(1, reply)))
            self.addCleanup(client.close)
            with self.subTest(text=text), self.assertRaises(ProtocolError):
                client.decide(question='q', evidence='x')
        duplicated = '{"jsonrpc":"2.0","id":1,"id":1,"result":{}}\n'
        client = Client(io.StringIO(), io.StringIO(duplicated))
        self.addCleanup(client.close)
        with self.assertRaises(ProtocolError):
            client.ping()

    def test_shared_scalar_and_error_fixtures_keep_named_method_types(self):
        # Shared expected values exercise the public consumer only. No engine
        # result, metadata, request count or parity claim is manufactured here.
        document = json.loads((Path(__file__).resolve().parents[2] / 'conformance/cases.json').read_text())
        cases = {case['id']: case for case in document['cases']}
        for ident in ('02-decide-no', '03-decide-band-unsure', '07-choose-unsure',
                      '10-tag-none', '11-score-middle'):
            case = cases[ident]
            expected = case['expect']['success']['answers'][0]['bare']
            outgoing = io.StringIO()
            client = Client(outgoing, io.StringIO(wire(1, result({'value': expected}))))
            self.addCleanup(client.close)
            observed = getattr(client, case['verb'])(question=case['question'], evidence='café 😀')
            self.assertIs(type(observed['value']), type(expected))
            self.assertEqual(observed['value'], expected)
            self.assertEqual(json.loads(outgoing.getvalue())['params']['name'], case['verb'])
        for ident in ('20-usage-fault', '21-backend-fault', '22-local-fault',
                      '23-cancelled-fault', '24-deadline-fault', '25-defect-fault'):
            kind = cases[ident]['expect']['error']['kind']
            error = {'error': {'kind': kind, 'message': 'safe fixture failure', 'retryable': False}}
            client = Client(io.StringIO(), io.StringIO(wire(1, result(error, True))))
            self.addCleanup(client.close)
            with self.assertRaises(ToolError) as failure:
                client.decide(question='q', evidence='x')
            self.assertEqual(failure.exception.result, error)

    def test_public_named_file_and_record_arguments_preserve_utf8_null_false_and_order(self):
        outgoing = io.StringIO()
        native = {'fixture': True, 'value': None}
        client = Client(outgoing, io.StringIO(wire(1, result(native))))
        self.addCleanup(client.close)
        records = [False, None, {'zèbre': '😀', 'alpha': 'café'}]
        client.annotate(question_file='questions/café.json', records=records,
                        options={'field': '/zèbre'})
        arguments = json.loads(outgoing.getvalue())['params']['arguments']
        self.assertEqual(arguments['question_file'], 'questions/café.json')
        self.assertIs(arguments['records'][0], False)
        self.assertIsNone(arguments['records'][1])
        self.assertEqual(list(arguments['records'][2]), ['zèbre', 'alpha'])

    def test_real_subprocess_initialization_named_calls_and_cancel_remain_responsive(self):
        # Handwritten wire fixture. It is not a ThinkThen execution/parity pass.
        code = r"""
import json,sys
for line in sys.stdin:
    request=json.loads(line)
    method=request['method']
    if 'id' not in request: continue
    if method=='initialize':
        value={'protocolVersion':'2025-11-25','capabilities':{'tools':{}},'serverInfo':{'name':'fixture','version':'1'}}
    elif method=='ping': value={}
    elif method=='tools/call':
        if request['params']['arguments'].get('evidence')=='hold': continue
        native={'fixture':True,'name':request['params']['name'],'value':False}
        value={'structuredContent':native,'content':[{'type':'text','text':json.dumps(native)}],'isError':False}
    print(json.dumps({'jsonrpc':'2.0','id':request['id'],'result':value}),flush=True)
"""
        with Client.launch((sys.executable, '-u', '-c', code), env={'PATH': '/usr/bin:/bin'}) as client:
            for name in NAMES:
                reply = getattr(client, name)(question='@literal', evidence='x')
                self.assertEqual(reply, {'fixture': True, 'name': name, 'value': False})
            outcomes = []
            def held():
                try:
                    client.decide(question='q', evidence='hold')
                except CancelledError:
                    outcomes.append('cancelled')
            worker = threading.Thread(target=held)
            worker.start()
            due = time.monotonic() + 3
            while client.pending_id is None and time.monotonic() < due:
                time.sleep(0.005)
            self.assertIsNotNone(client.pending_id)
            client.cancel(client.pending_id)
            worker.join(timeout=3)
            self.assertFalse(worker.is_alive())
            self.assertEqual(outcomes, ['cancelled'])
            self.assertEqual(client.ping(), {})


if __name__ == '__main__':
    unittest.main()
