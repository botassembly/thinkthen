#!/usr/bin/python3
"""Independent bounded raw stdio reader; runs as its own fresh process."""
import json
import os
from pathlib import Path
import selectors
import signal
import subprocess
import sys
import time

base = Path('/tmp/thinkthen-0425-rc13-qa')
binary, repo = sys.argv[1:]
expected = json.loads((Path(repo) / 'conformance/cases.json').read_text())['parity']['functions']
process = subprocess.Popen([binary, 'mcp', '--no-cache'], stdin=subprocess.PIPE,
                           stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
selector = selectors.DefaultSelector()
selector.register(process.stdout, selectors.EVENT_READ)
buffer = bytearray()

def send(value):
    process.stdin.write((json.dumps(value, separators=(',', ':')) + '\n').encode())
    process.stdin.flush()


def receive(ident):
    deadline = time.monotonic() + 15
    while b'\n' not in buffer:
        remaining = deadline - time.monotonic()
        assert remaining > 0 and selector.select(remaining), 'MCP response timed out'
        chunk = os.read(process.stdout.fileno(), 65536)
        assert chunk, 'MCP stdio closed'
        buffer.extend(chunk)
        assert len(buffer) < 1024 * 1024, 'unexpectedly large catalog/startup response'
    newline = buffer.index(10) + 1
    raw = bytes(buffer[:newline])
    del buffer[:newline]
    value = json.loads(raw)
    assert value.get('jsonrpc') == '2.0' and value.get('id') == ident and 'error' not in value, 'MCP response differs'
    return raw, value['result']


try:
    send({'jsonrpc': '2.0', 'id': 1, 'method': 'initialize', 'params': {
        'protocolVersion': '2025-11-25', 'capabilities': {},
        'clientInfo': {'name': 'release-qa-fresh-reader', 'version': '1'}}})
    _, initialized = receive(1)
    assert initialized['protocolVersion'] == '2025-11-25'
    send({'jsonrpc': '2.0', 'method': 'notifications/initialized'})
    send({'jsonrpc': '2.0', 'id': 2, 'method': 'tools/list'})
    raw, result = receive(2)
    (base / 'runtime-catalog-wire.jsonl').write_bytes(raw)
    assert len(raw) < 25000, 'actual full tools/list wire response reaches 25 KB'
    assert len(result['tools']) == 10 and [tool['name'] for tool in result['tools']] == expected
    assert all(isinstance(tool['inputSchema'], dict) for tool in result['tools'])
    process.stdin.close()
    assert process.wait(timeout=10) == 0, 'MCP failed to retire on EOF'
    stderr = process.stderr.read()
    (base / 'runtime-catalog.stderr').write_bytes(stderr)
    (base / 'runtime-catalog.json').write_text(json.dumps({
        'tool_count': 10, 'full_wire_bytes': len(raw), 'bound_bytes_exclusive': 25000,
        'reader': 'separate process, raw stdio, no shared client', 'process_reaped': True}, indent=2) + '\n')
finally:
    selector.close()
    if process.poll() is None:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            process.wait(timeout=2)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait()
    try:
        os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass
    for stream in (process.stdin, process.stdout, process.stderr):
        if not stream.closed:
            stream.close()
