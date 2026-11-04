#!/usr/bin/env python3
"""Run the published Windows checksum command in a pinned Microsoft PS7 image."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import zipfile

# Resolved from Microsoft's registry; manifest bytes independently hash to this digest.
# https://mcr.microsoft.com/v2/powershell/manifests/7.5-ubuntu-22.04
IMAGE = 'mcr.microsoft.com/powershell@sha256:a3affe99603400235501b8da8be5f9e40152d4db6557f698a91da0280f9e1469'
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
site = Path(__file__).resolve().parents[1]
data = json.loads((site / 'src/data/site.json').read_text())
command = data['install']['windows']
filename = f"thinkthen-{data['version']}-windows-x86_64.zip"
args.output.parent.mkdir(parents=True, exist_ok=True)
with tempfile.TemporaryDirectory(prefix='thinkthen-install-checksum-') as temporary:
    root = Path(temporary)
    for case in ['match', 'mismatch']:
        folder = root / case
        folder.mkdir()
        archive = folder / filename
        with zipfile.ZipFile(archive, 'w') as fixture:
            fixture.writestr('fixture.txt', 'Checksum logic fixture; not a Windows program.\n')
        digest = hashlib.sha256(archive.read_bytes()).hexdigest()
        wanted = digest if case == 'match' else '0' * 64
        (folder / 'SHA256SUMS').write_text(f'{wanted}  {filename}\n')
        (folder / 'check.ps1').write_text(command + '\n')
    (root / 'run.ps1').write_text('''$ErrorActionPreference = "Stop"
$results = foreach ($case in @("match", "mismatch")) {
    Set-Location "/proof/$case"
    $answer = & ./check.ps1
    [pscustomobject]@{case=$case; stdout="$answer"}
}
@{powershell=$PSVersionTable.PSVersion.ToString(); cases=@($results)} | ConvertTo-Json -Depth 5
''')
    docker = ['docker', 'run', '--rm']
    if os.environ.get('JOB_LABEL_FILE'):
        docker += ['--label-file', os.environ['JOB_LABEL_FILE']]
    docker += ['--network', 'none', '--memory', '512m', '--cpus', '1',
               '--entrypoint', 'pwsh', '-v', f'{root}:/proof:ro', IMAGE,
               '-NoProfile', '-NonInteractive', '-File', '/proof/run.ps1']
    result = subprocess.run(docker, text=True, capture_output=True)
    report = dict(image=IMAGE, command=command, exit=result.returncode,
                  stdout=result.stdout, stderr=result.stderr,
                  scope='Published checksum logic on fixture ZIPs; not Windows execution.')
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    assert result.returncode == 0, (result.returncode, result.stdout, result.stderr)
    observed = json.loads(result.stdout)
    assert observed['powershell'].startswith('7.'), observed
    assert observed['cases'] == [dict(case='match', stdout='Checksum matches'),
                                 dict(case='mismatch', stdout='No match')], observed
    report.update(observed)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
print(f'Published Windows checksum: match and mismatch passed; {args.output}')
