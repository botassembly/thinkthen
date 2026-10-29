"""Exercise native SQL consumers when the engine cannot name a returned code."""
import json
import subprocess
import sys

from testing import child_environment

surface, consumer, extension = sys.argv[1:]
if surface == 'duckdb':
    statements = ["SET thinkthen_backend='stub'",
                  "SELECT * FROM thinkthen_decide((SELECT NULL::VARCHAR AS record),'Ready?')",
                  'SET thinkthen_max_retries=4']
    commands = statements
elif surface == 'sqlite':
    statements = ["SELECT thinkthen_set('backend','stub')",
                  "SELECT * FROM thinkthen_decide('SELECT NULL AS record','Ready?')",
                  "SELECT thinkthen_set('max_retries',4)"]
    commands = [dict(sql=statement, parameters=[]) for statement in statements]
else:
    raise AssertionError(surface)
result = subprocess.run([consumer, extension], env=child_environment(),
    input=''.join(json.dumps(command) + '\n' for command in commands),
    text=True, capture_output=True, timeout=30)
assert result.returncode == 0, (surface, result.returncode, result.stdout, result.stderr)
responses = [json.loads(line) for line in result.stdout.splitlines()]
assert len(responses) == len(commands), responses
for response in responses[1:]:
    message = response.get('error') if surface == 'duckdb' else response['message']
    assert message and 'usage.abi' in message, (surface, response)
print(surface + ' refuses unnamed engine codes through its actual SQL host')
