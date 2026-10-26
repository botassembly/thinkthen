#!/usr/bin/env python3
"""Observe one matched original C# soak with external snapshots and EventPipe.

Usage: csharp-trace.py ORIGINAL_STAGE VERIFIED_TOOL_STAGE OUTPUT_DIRECTORY
No consumer or SDK assembly is rebuilt. Decode only after both normal exits.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import time

from stress_monitor import Monitor

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('adapters', ROOT / 'scripts/stress-adapters.py')
adapters = importlib.util.module_from_spec(spec)
spec.loader.exec_module(adapters)
ROUNDS = 60


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


def prepare_decoder(tools, output, env):
    decoder = output / 'decoder'
    decoder.mkdir()
    for name in ('Probe.csproj','NuGet.Config'):
        shutil.copyfile(tools / 'probe' / name, decoder / name)
    shutil.copyfile(ROOT / 'scripts/csharp-trace-read.cs', decoder / 'Program.cs')
    command = [str(Path(env['DOTNET_ROOT']) / 'dotnet'),'build',str(decoder / 'Probe.csproj'),
               '--configfile',str(decoder / 'NuGet.Config'),'-o',str(output / 'decoder-bin'),'--disable-build-servers']
    with (output / 'decoder-build.log').open('w') as log:
        subprocess.run(command,env=env,stdout=log,stderr=subprocess.STDOUT,check=True)


def main():
    original, tools, output = [Path(value).resolve() for value in sys.argv[1:]]
    output.mkdir(parents=True,exist_ok=False)
    provenance = json.loads((tools / 'provenance.json').read_text())
    if provenance['version'] != '10.0.750501' or provenance['signature_exit'] != 0:
        raise ValueError('tool preflight did not pass')
    shutil.copytree(original / 'csharp/bin', output / 'original-bin')
    files = ('Consumer.dll','ThinkThen.dll','runtimes/linux-x64/native/libthinkthen_ffi.so')
    hashes = {}
    for name in files:
        before = hashlib.sha256((original / 'csharp/bin' / name).read_bytes()).hexdigest()
        after = hashlib.sha256((output / 'original-bin' / name).read_bytes()).hexdigest()
        if before != after:
            raise ValueError('original artifact changed')
        hashes[name] = after
    hashes['package'] = hashlib.sha256((original / 'csharp/feed/Botassembly.ThinkThen.0.2.0.nupkg').read_bytes()).hexdigest()
    save(output / 'inputs.json',dict(source_revision=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),
        original_stage=str(original),hashes=hashes,tool=provenance,
        relationship='Same original installed consumer/package/native as the saved 20-round failure; 60 healthy rounds with external runtime diagnostics. Private instrumented run remains separate.'))
    env = dict(os.environ,DOTNET_CLI_HOME=str(output / 'home'),NUGET_PACKAGES=str(output / 'packages'))
    prepare_decoder(tools,output,env)
    dotnet = str(Path(env['DOTNET_ROOT']) / 'dotnet')
    tool = str(Path(provenance['tool_directory']) / 'dotnet-trace.dll')
    for name, arguments in [('version',['--version']),('help',['collect','--help'])]:
        result = subprocess.run([dotnet,tool,*arguments],env=env,capture_output=True,text=True)
        (output / ('tool-' + name + '.log')).write_text(result.stdout + result.stderr)
        result.check_returncode()
    fixture = adapters.Fixture(['--stress','healthy'])
    collector = process = monitor = None
    # Keep the socket path short enough for Unix-domain socket limits.
    socket_path = '/tmp/csharp-trace-' + str(os.getpid()) + '.sock'
    started = time.monotonic()
    try:
        table = json.loads((ROOT / 'conformance/stress.json').read_text())
        healthy = next(value for value in table['scenarios'] if value['name'] == 'healthy')
        save(output / 'case.json',dict(client=dict(table['client'],backend='systemone',url=fixture.url,api_key='loopback-only'),
            scenario=healthy,records=table['records']['full'],probability=table['replies'][0]['probability'],concurrency=1,rounds=ROUNDS,cancel=False))
        with (output / 'collector.stdout').open('w') as cout, (output / 'collector.stderr').open('w') as cerr, (output / 'samples.jsonl').open('w') as samples:
            command = [dotnet,tool,'collect','--diagnostic-port',socket_path,'--buffersize','4',
                       '--providers','Microsoft-Windows-DotNETRuntime:0x1:4','--rundown','false','--output',str(output / 'raw.nettrace')]
            save(output / 'collector-command.json',command)
            collector = subprocess.Popen(command,env=env,stdin=subprocess.DEVNULL,stdout=cout,stderr=cerr)
            owned = [fixture.process.pid]
            monitor = Monitor(lambda: owned,int(fixture.url.split(':')[2].split('/')[0]),samples)
            target_env = dict(env,DOTNET_DiagnosticPorts=socket_path,DOTNET_PerfMapEnabled='3',
                THINKTHEN_STRESS_CASE=str(output / 'case.json'),THINKTHEN_MEMORY_OUTPUT=str(output),
                THINKTHEN_STRESS_CHECKPOINT=monitor.path,THINKTHEN_STRESS_CONCURRENCY='1',
                THINKTHEN_STRESS_CHECKPOINT_COMMAND='exec ' + shlex.join([sys.executable,str(ROOT / 'scripts/csharp-trace-checkpoint.py')]))
            with (output / 'consumer.stdout').open('w') as stdout, (output / 'consumer.stderr').open('w') as stderr:
                process = subprocess.Popen([dotnet,str(output / 'original-bin/Consumer.dll'),'--stress-adapter'],env=target_env,
                    cwd=output,stdin=subprocess.DEVNULL,stdout=stdout,stderr=stderr)
                owned.append(process.pid)
                status = process.wait()
            save(output / 'consumer-exit.json',dict(pid=process.pid,exit_code=status,elapsed_seconds=time.monotonic()-started))
            collector_status = collector.wait(timeout=60)
            save(output / 'collector-exit.json',dict(pid=collector.pid,exit_code=collector_status))
            for name in (f'perf-{process.pid}.map',f'perfinfo-{process.pid}.map'):
                path = Path('/tmp') / name
                if path.exists():
                    shutil.copyfile(path,output / ('final-' + name))
            if status != 0 or collector_status != 0:
                raise RuntimeError('consumer or collector did not exit normally')
            actual = json.loads((output / 'consumer.stdout').read_text())
            if actual != dict(calls=ROUNDS,rows=table['records']['full']*ROUNDS,failed=0,cancelled=False):
                raise ValueError('original consumer returned unexpected totals')
            save(output / 'resources.json',monitor.finish(ROUNDS,
                verified_records={'workload': [actual['rows'] // ROUNDS] * ROUNDS}))
            monitor = None
    finally:
        if process is not None and process.poll() is None:
            process.kill(); process.wait()
        if collector is not None and collector.poll() is None:
            collector.terminate()
            try:
                collector.wait(timeout=10)
            except subprocess.TimeoutExpired:
                collector.kill()
                collector.wait()
            save(output / 'collector-abnormal.json',dict(exit_code=collector.returncode))
        try:
            if monitor is not None:
                monitor.finish()
        finally:
            try:
                fixture.close()
                save(output / 'fixture-exit.json',dict(exit_code=fixture.process.returncode))
            finally:
                Path(socket_path).unlink(missing_ok=True)
                if process is not None:
                    for name in (f'perf-{process.pid}.map', f'perfinfo-{process.pid}.map'):
                        (Path('/tmp') / name).unlink(missing_ok=True)
    # The workload, collector, and fixture have all exited before decoding begins.
    with (output / 'decode.log').open('w') as log:
        parsed = subprocess.run([dotnet,str(output / 'decoder-bin/Probe.dll'),str(output / 'raw.nettrace'),
            str(output / 'gc-events.jsonl'),str(output / 'trace-summary.json')],env=env,stdout=log,stderr=subprocess.STDOUT)
    save(output / 'decoder-exit.json',dict(exit_code=parsed.returncode))
    parsed.check_returncode()
    save(output / 'complete.json',dict(workload=actual,elapsed_seconds=time.monotonic()-started))
    print(json.dumps(dict(output=str(output),workload=actual)),flush=True)


if __name__ == '__main__':
    main()
