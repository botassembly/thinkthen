"""Installed-command MCP initialization, catalog and no-key recorded call."""
import json
from pathlib import Path
import sys
import tempfile
from client import Client, ProtocolError, ToolError

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env


def run(binary):
    case = json.loads((ROOT / 'conformance/cases.json').read_text())['cases'][0]
    with tempfile.TemporaryDirectory(prefix='thinkthen-mcp-installed-') as folder:
        env = child_env(home=folder, LANG='C.UTF-8')
        recording = ROOT / case['exchanges'][0]['provenance']['path']
        with Client.launch((binary, 'mcp', '--replay', str(recording), '--no-cache'), env=env) as client:
            assert client.ping() == {}, 'ping failed'
            expected = json.loads((ROOT / 'conformance/cases.json').read_text())['parity']['functions']
            assert [tool['name'] for tool in client.tools()] == expected, 'ten-tool catalog differs'
            result = client.decide(question=case['question'], evidence=case['exchanges'][0]['evidence'])
            assert result['value']['schema'] == 'thinkthen.result/2', 'complete result unavailable'
            assert result['value']['value'] is True, 'recorded answer changed'
            assert result['value']['meta']['origin'] == 'replay', 'replay provenance lost'
            assert result['facts']['requests_sent'] == 0, 'replay reported sends'


if __name__ == '__main__':
    if len(sys.argv) != 2:
        raise SystemExit('usage: installed.py ABSOLUTE_THINKTHEN_BINARY')
    try:
        run(sys.argv[1])
    except (AssertionError, KeyError, OSError, ProtocolError, ToolError):
        print('MCP installed check failed: complete initialization/tool execution unavailable', file=sys.stderr)
        raise SystemExit(1) from None
