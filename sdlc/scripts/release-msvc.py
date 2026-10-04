#!/usr/bin/env python3
"""Select the runner's installed x64 MSVC tools before native C operations."""
import os
from pathlib import Path
import subprocess

environment = {name: os.environ[name] for name in ('PATH', 'SystemRoot', 'SystemDrive', 'TEMP', 'TMP',
    'COMSPEC', 'ProgramFiles', 'ProgramFiles(x86)', 'ProgramW6432') if name in os.environ}

finder = Path(os.environ['ProgramFiles(x86)']) / 'Microsoft Visual Studio/Installer/vswhere.exe'
installation = subprocess.check_output([str(finder), '-latest', '-products', '*', '-requires',
    'Microsoft.VisualStudio.Component.VC.Tools.x86.x64', '-property', 'installationPath'], text=True, env=environment, timeout=60).strip()
setup = Path(installation) / 'VC/Auxiliary/Build/vcvarsall.bat'
if not installation or not setup.is_file():
    raise SystemExit('release-msvc: installed x64 compiler environment is missing')
result = subprocess.run(f'cmd.exe /d /s /c ""{setup}" x64 >nul && set"',
                        capture_output=True, text=True, check=True, timeout=60, env=environment)
values = {name.upper(): value for line in result.stdout.splitlines()
          if '=' in line for name, value in [line.split('=', 1)]}
for name in ('PATH', 'INCLUDE', 'LIB', 'LIBPATH'):
    value = values.get(name)
    if not value or '\n' in value or '\r' in value:
        raise SystemExit(f'release-msvc: missing {name}')
    print(f'{name}={value}')
