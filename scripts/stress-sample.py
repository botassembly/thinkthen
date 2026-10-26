#!/usr/bin/env python3
"""Sample an owned Linux workload, including daemonized servers and Docker workers.

Usage: stress-sample.py [--output JSONL] [--rounds N] [--backend-port PORT] -- COMMAND
--rounds requires N synchronous completed-round calls to scripts/stress-checkpoint.
A failing growth verdict fails the command. Docker run receives an owned cidfile;
other container launch forms must run the sampler inside the container.
"""
import argparse
import json
import os
from pathlib import Path
import signal
import shlex
import socket
import subprocess
import sys
import tempfile

from build_targets import container_labels
from stress_monitor import Monitor
from stress_resources import MIN_ROUNDS, adopt_daemons


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path)
    parser.add_argument('--rounds', type=int)
    parser.add_argument('--backend-port', type=int)
    parser.add_argument('--groups', help='comma-separated required surface/backend checkpoint groups')
    parser.add_argument('command', nargs=argparse.REMAINDER)
    options = parser.parse_args()
    command = options.command[1:] if options.command[:1] == ['--'] else options.command
    if not command or not Path('/proc/self/status').exists():
        parser.error('a command and Linux /proc are required')
    if options.rounds is not None and options.rounds < MIN_ROUNDS:
        parser.error(f'--rounds must be at least {MIN_ROUNDS}')
    adopt_daemons()
    with tempfile.TemporaryDirectory(prefix='stress-container-') as temporary:
        cidfile = Path(temporary) / 'id'
        container = Path(command[0]).name == 'docker'
        if container:
            if command[1:2] != ['run']:
                parser.error('run the sampler inside the container for Docker commands other than run')
            command[2:2] = ['--cidfile', str(cidfile),
                            *container_labels(Path(__file__).resolve().parent.parent)]
        process = None
        accounted_container = False
        def roots():
            nonlocal accounted_container
            owned = [os.getpid()]
            if container and cidfile.exists():
                result = subprocess.run(['docker', 'inspect', '--format', '{{.State.Pid}}',
                    cidfile.read_text().strip()], capture_output=True, text=True)
                if result.returncode == 0:
                    pid = int(result.stdout.strip())
                    if pid:
                        if not Path(f'/proc/{pid}/status').exists():
                            raise RuntimeError('container workers are outside this PID namespace; run the sampler inside the container')
                        owned.append(pid)
                        accounted_container = True
                elif not accounted_container and process is not None and process.poll() is None:
                    raise RuntimeError('cannot account for owned Docker container')
            return owned
        output = options.output.open('w') if options.output else None
        # Construct before launch so even the first completed round has a listener.
        monitor = Monitor(roots, options.backend_port, output)
        try:
            process = subprocess.Popen(command, env=dict(os.environ,
            THINKTHEN_STRESS_CHECKPOINT=monitor.path,
            THINKTHEN_STRESS_CHECKPOINT_COMMAND='exec ' + shlex.join([sys.executable, str(Path(__file__).with_name('stress-checkpoint'))])), start_new_session=True)
            status = process.wait()
            report = monitor.finish(options.rounds, options.groups.split(',') if options.groups else None)
            if container and not accounted_container:
                raise RuntimeError('container ended without an owned worker sample')
        except BaseException:
            if process is not None:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.wait()
            if monitor.thread.is_alive():
                monitor.finish()
            raise
        finally:
            if container and cidfile.exists():
                subprocess.run(['docker', 'rm', '-f', cidfile.read_text().strip()],
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            if output:
                output.close()
        print(json.dumps(dict(host=socket.gethostname(), load=os.getloadavg(), cache='prepared build',
                              exit_code=status, **report)))
        return status or (1 if report.get('growth', {}).get('passed') is False else 0)


if __name__ == '__main__':
    sys.exit(main())
