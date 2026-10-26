"""Read glibc stack ownership from GDB without calling inferior functions."""
import json
import os
from pathlib import Path
import signal
import time

import gdb


def stacks(name):
    head = gdb.parse_and_eval('&_rtld_global.' + name)
    offset = int(gdb.parse_and_eval('(long)&((struct pthread *)0)->list'))
    pointer = gdb.lookup_type('struct pthread').pointer()
    node = head.dereference()['next']
    seen = set()
    result = []
    while int(node) != int(head):
        address = int(node)
        if address in seen or len(seen) >= 128:
            raise RuntimeError('invalid or unexpectedly large stack list')
        seen.add(address)
        thread = gdb.Value(address - offset).cast(pointer).dereference()
        start = int(thread['stackblock'])
        size = int(thread['stackblock_size'])
        result.append(dict(descriptor=address - offset, start=start, end=start + size,
            size=size, tid=int(thread['tid']), guard_bytes=int(thread['guardsize']),
            reported_guard_bytes=int(thread['reported_guardsize']),
            guard_mode=str(thread['stack_mode'])))
        node = node.dereference()['next']
    return result


class Checkpoint(gdb.Breakpoint):
    count = 0

    def stop(self):
        output = Path(os.environ['THINKTHEN_STACK_SNAPSHOT_OUTPUT'])
        pid = gdb.selected_inferior().pid
        started = time.monotonic()
        try:
            self.count += 1
            if self.count != 1:
                raise RuntimeError('unexpected extra completed round')
            snapshot = dict(pid=pid, cached=stacks('_dl_stack_cache'),
                active=stacks('_dl_stack_used'), user=stacks('_dl_stack_user'),
                cache_bytes=int(gdb.parse_and_eval('_rtld_global._dl_stack_cache_actsize')),
                cache_lock=int(gdb.parse_and_eval('_rtld_global._dl_stack_cache_lock')),
                in_flight=int(gdb.parse_and_eval('_rtld_global._dl_in_flight_stack')))
            for name in ('maps', 'smaps', 'smaps_rollup', 'status'):
                (output / name).write_text((Path('/proc') / str(pid) / name).read_text())
            snapshot['observer_seconds'] = time.monotonic() - started
            (output / 'stack-ownership.json').write_text(json.dumps(snapshot, indent=2))
            print('COMPLETED_ROUND_STACK_SNAPSHOT', flush=True)
        except BaseException as error:
            (output / 'measurement-failure.json').write_text(json.dumps(dict(error=repr(error))))
            os.kill(pid, signal.SIGKILL)
        return False


Checkpoint('system')
