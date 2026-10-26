#!/usr/bin/env python3
"""Translate the shared stress adapter protocol for the compiled C consumer."""
import json
import os
from pathlib import Path
import subprocess
import sys

case = json.loads(Path(os.environ['THINKTHEN_STRESS_CASE']).read_text())
scenario = case['scenario']
# The synthetic key is already admitted through the canonical client settings.
parameters = '{} {} {} {} {} {} {} {}\n'.format(
    case['records'], case['concurrency'], case['rounds'], int(case['cancel']),
    int(scenario['status'] == 'answered'), int(scenario['name'] == 'refused'),
    case['probability'], scenario.get('code', '-'))
with subprocess.Popen([sys.argv[1], 'workload'], stdin=subprocess.PIPE, text=True) as process:
    process.stdin.write(json.dumps(case['client']) + '\n' + parameters)
    process.stdin.flush()
    if case['cancel']:
        assert sys.stdin.readline() == 'cancel\n'
        process.stdin.write('x')
        process.stdin.flush()
    process.stdin.close()
    sys.exit(process.wait())
