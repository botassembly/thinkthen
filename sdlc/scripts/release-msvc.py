#!/usr/bin/env python3
"""Select the runner's installed x64 MSVC tools before native C operations."""
import os
from pathlib import Path
import runpy

CAPTURE = runpy.run_path(str(Path(__file__).with_name('release-bounded.py')))['capture']


def run(args, environment):
    result = CAPTURE(args, env=environment, text=True, timeout=60)
    if result.returncode:
        tool = Path(args[0]).name if not isinstance(args, str) else args.split()[0]
        raise ValueError(f'release-msvc: {tool} failed (exit {result.returncode}): {result.stdout}{result.stderr}')
    return result.stdout


def main():
    environment = {name: os.environ[name] for name in ('PATH', 'SystemRoot', 'SystemDrive', 'TEMP', 'TMP',
        'COMSPEC', 'ProgramFiles', 'ProgramFiles(x86)', 'ProgramW6432') if name in os.environ}

    finder = Path(os.environ['ProgramFiles(x86)']) / 'Microsoft Visual Studio/Installer/vswhere.exe'
    installation = run([str(finder), '-latest', '-products', '*', '-requires',
        'Microsoft.VisualStudio.Component.VC.Tools.x86.x64', '-property', 'installationPath'], environment).strip()
    setup = Path(installation) / 'VC/Auxiliary/Build/vcvarsall.bat'
    if not installation or not setup.is_file():
        raise SystemExit('release-msvc: installed x64 compiler environment is missing')
    output = run(f'cmd.exe /d /s /c ""{setup}" x64 >nul && set"', environment)
    values = {name.upper(): value for line in output.splitlines()
              if '=' in line for name, value in [line.split('=', 1)]}
    for name in ('PATH', 'INCLUDE', 'LIB', 'LIBPATH'):
        value = values.get(name)
        if not value or '\n' in value or '\r' in value:
            raise SystemExit(f'release-msvc: missing {name}')
        print(f'{name}={value}')


if __name__ == '__main__':
    main()
