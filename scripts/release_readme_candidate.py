"""Run literal README commands against the installer's private candidate mirror."""
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess


def blocks(readme, windows):
    def section(title):
        return readme.split('## ' + title + '\n', 1)[1].split('\n## ', 1)[0]
    def fence(body, language):
        return re.search(r'```' + language + r'\n(.*?)\n```', body, re.S)[1]
    return (fence(section('Install'), 'powershell' if windows else 'sh'),
            fence(section('Ask a question'), 'sh'),
            fence(section('Ask a question'), 'text') + '\n')


def fetch_setup(work, base, windows):
    """Map only the documented installer URI, inside the command's own process."""
    if windows:
        return """function Readme-CandidateFetch {
    param([string]$Uri)
    if ($Uri -cne 'https://thinkthen.dev/install.ps1') { throw 'Unexpected README fetch URI' }
    Microsoft.PowerShell.Utility\\Invoke-RestMethod -Uri '""" + base + "/install.ps1'\n}\nSet-Alias -Name irm -Value Readme-CandidateFetch -Option (Get-Alias irm).Options -Force\n"
    real_curl = shutil.which('curl')
    if real_curl is None:
        raise ValueError('curl is required for the README installer proof')
    shim = work / 'fetch'
    shim.mkdir()
    curl = shim / 'curl'
    curl.write_text('#!/bin/sh\n'
                    'if [ "$#" -ne 2 ] || [ "$1" != "-fsSL" ] || '
                    '[ "$2" != "https://thinkthen.dev/install.sh" ]; then\n'
                    '  echo "Unexpected README fetch URI or options" >&2; exit 2\nfi\n'
                    'exec ' + shlex.quote(real_curl) + ' -fsSL ' + shlex.quote(base + '/install.sh') + '\n')
    curl.chmod(0o755)
    # The installer needs its real curl for the already-local asset/API URLs.
    return 'curl() { ' + shlex.quote(str(curl)) + ' "$@"; }\n'


def run_readme(readme_path, binary, env, home, mirror, base, installed, receipt):
    windows = os.name == 'nt'
    install, question, expected = blocks(readme_path.read_text(encoding='utf-8'), windows)
    installer = 'install.ps1' if windows else 'install.sh'
    shutil.copy2(Path(__file__).resolve().parent.parent / installer, mirror / installer)
    setup = fetch_setup(home, base, windows)
    environment = dict(env, THINKTHEN_INSTALL_DIR=str(installed.parent))
    shell = ['powershell.exe', '-NoProfile', '-NonInteractive', '-Command'] if windows else ['sh', '-eu', '-c']
    prefix = "$ErrorActionPreference = 'Stop'\n" if windows else ''
    print('README Install (literal):\n' + install, flush=True)
    result = subprocess.run(shell + [prefix + setup + install], env=environment,
                            cwd=home, capture_output=True, timeout=120)
    if result.returncode:
        raise ValueError('README candidate installation failed: ' + result.stderr.decode(errors='replace'))
    digest = hashlib.sha256(binary.read_bytes()).hexdigest()
    record = json.loads(receipt.read_text(encoding='utf-8-sig'))
    if (hashlib.sha256(installed.read_bytes()).hexdigest() != digest
            or record['state'] != 'installed' or record['new_sha256'] != digest
            or Path(record['executable_path']) != installed):
        raise ValueError('README installed binary/receipt differs from the candidate')
    environment.update(PATH=str(installed.parent) + os.pathsep + env.get('PATH', os.defpath),
                       THINKTHEN_BACKEND='stub')
    for name in ('THINKTHEN_API_KEY', 'OPENAI_API_KEY'):
        environment.pop(name, None)
    print('README Ask a question (literal):\n' + question, flush=True)
    suffix = '\nexit $LASTEXITCODE' if windows else ''
    result = subprocess.run(shell + [prefix + question + suffix], env=environment, cwd=home,
                            capture_output=True, timeout=30)
    if result.returncode or result.stdout.replace(b'\r\n', b'\n') != expected.encode() or result.stderr:
        raise ValueError('README first answer differs: ' + repr((result.returncode, result.stdout, result.stderr)))
    print('README: literal installation and first answer pass; installed candidate SHA256=' + digest, flush=True)
