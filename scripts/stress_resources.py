"""Linux ownership and distinct TCP connection accounting for offline stress runs."""
import ctypes
import os
from pathlib import Path

METRICS = ('memory_bytes', 'descriptors', 'backend_connections', 'threads', 'processes')
CRITERION = 'retained-round-growth-per-record-floor'
SHORT_GATE_CRITERION = 'retained-round-growth-short-gate-report'
EXTENDED_CRITERION = 'retained-round-explicit-memory-window-per-record-floor'
MEMORY_BYTES_PER_RECORD_FLOOR = 1
MEMORY_WINDOWS = 4
MIN_ROUNDS = 2 * MEMORY_WINDOWS
# Linux include/linux/sched.h: https://github.com/torvalds/linux/blob/v6.12/include/linux/sched.h
PF_EXITING = 0x00000004


def adopt_daemons():
    """Keep double-forked servers in this runner's owned descendant tree."""
    if ctypes.CDLL(None, use_errno=True).prctl(36, 1, 0, 0, 0):
        raise OSError(ctypes.get_errno(), 'cannot become a child subreaper')


def process_state(directory):
    stat = (directory / 'stat').read_text()
    fields = stat.rsplit(')', 1)[1].split()
    return dict(pid=int(directory.name), parent_pid=int(fields[1]),
                name=stat.split('(', 1)[1].rsplit(')', 1)[0],
                start_time_ticks=int(fields[19]), state=fields[0], flags=int(fields[6]))


class AccessLost(Exception):
    """An owned process denied a resource read; retain both sides of the read."""
    def __init__(self, identity, error):
        try:
            after = process_state(Path('/proc') / str(identity['pid']))
        except (FileNotFoundError, ProcessLookupError):
            after = dict(state='absent')
        except PermissionError:
            after = dict(state='inaccessible')
        self.exited = (after['state'] == 'absent' or
                       (after.get('start_time_ticks') == identity.get('start_time_ticks') and
                        (after['state'] in ('Z', 'X', 'x') or after.get('flags', 0) & PF_EXITING)))
        self.failure = dict(process=identity.copy(), access=str(error.filename),
                            error=type(error).__name__, errno=error.errno,
                            state_after=after, disposition='process_exit' if self.exited else 'unverifiable')
        super().__init__(f"resource observation lost access: {self.failure}")


def sample(roots, backend_port=None, exclude=(), required_pid=None, details=None, tracked=None, events=None):
    tracked = {} if tracked is None else tracked
    processes = {}
    for directory in Path('/proc').iterdir():
        if not directory.name.isdecimal():
            continue
        try:
            identity = process_state(directory)
            processes[identity['pid']] = (identity['parent_pid'], directory, identity)
        except PermissionError as error:
            pid = int(directory.name)
            if pid in tracked or pid in roots:
                raise AccessLost(tracked.get(pid, dict(pid=pid)), error) from error
        except (FileNotFoundError, ProcessLookupError, ValueError, IndexError):
            continue
    for pid in list(tracked):
        if pid not in processes or tracked[pid]['start_time_ticks'] != processes[pid][2]['start_time_ticks']:
            if events is not None:
                events.append(dict(event='process_exit', process=tracked[pid].copy(),
                    state_after=processes[pid][2] if pid in processes else dict(state='absent')))
            del tracked[pid]
    selected = set(roots) | tracked.keys()
    while True:
        children = {pid for pid, (parent, _, _) in processes.items() if parent in selected}
        if children <= selected:
            break
        selected |= children
    result = dict(memory_bytes=0, descriptors=0, socket_descriptors=0,
                  distinct_sockets=0, backend_connections=0, threads=0, processes=0)
    observed = set()
    inodes = set()
    namespaces = {}
    for pid in selected - set(exclude):
        if pid not in processes:
            continue
        _, directory, identity = processes[pid]
        try:
            command = (directory / 'cmdline').read_bytes().split(b'\0')
            if len(command) > 1 and command[1] == os.fsencode(Path(__file__).with_name('stress-checkpoint')):
                continue
            tracked[pid] = identity
            status = dict(line.split(':', 1) for line in (directory / 'status').read_text().splitlines())
            if status['State'].strip().startswith('Z'):
                result['processes'] += 1
                if details is not None:
                    details[pid] = dict(name=status['Name'].strip(), memory_bytes=0,
                                       descriptors=0, threads=0, state='zombie')
                continue
            memory = int(status.get('VmRSS', '0 kB').split()[0]) * 1024
            threads = int(status['Threads'])
            name = (directory / 'comm').read_text().strip()
            descriptors = list((directory / 'fd').iterdir())
            namespace = os.readlink(directory / 'ns/net')
            process_inodes = set()
            socket_descriptors = 0
            for fd in descriptors:
                try:
                    target = os.readlink(fd)
                    if target.startswith('socket:['):
                        socket_descriptors += 1
                        process_inodes.add((namespace, target[8:-1]))
                except FileNotFoundError:
                    pass
            namespaces[namespace] = (Path('/proc/self') if namespace == os.readlink('/proc/self/ns/net') else directory)
            result['memory_bytes'] += memory
            result['threads'] += threads
            result['descriptors'] += len(descriptors)
            result['socket_descriptors'] += socket_descriptors
            inodes.update(process_inodes)
            result['processes'] += 1
            observed.add(pid)
            if details is not None:
                details[pid] = dict(name=name, memory_bytes=memory, descriptors=len(descriptors), threads=threads)
        except PermissionError as error:
            loss = AccessLost(identity, error)
            if not loss.exited:
                raise loss from error
            if events is not None:
                events.append(dict(event='access_denied', **loss.failure))
                events.append(dict(event='process_exit', process=identity.copy(), state_after=loss.failure['state_after']))
            continue
        except FileNotFoundError:
            continue
    if required_pid is not None and required_pid not in observed:
        raise ValueError(f'required live workload PID {required_pid} was not sampled')
    connections = set()
    if backend_port is not None:
        for namespace, directory in namespaces.items():
            for protocol in ('tcp', 'tcp6'):
                try:
                    rows = (directory / 'net' / protocol).read_text().splitlines()[1:]
                except PermissionError as error:
                    pid = os.getpid() if directory == Path('/proc/self') else int(directory.name)
                    raise AccessLost(tracked.get(pid, dict(pid=pid)), error) from error
                except FileNotFoundError:
                    continue
                for line in rows:
                    fields = line.split()
                    if (namespace, fields[9]) in inodes and int(fields[2].split(':')[1], 16) == backend_port:
                        connections.add((namespace, protocol, fields[1], fields[2]))
    result['distinct_sockets'] = len(inodes)
    result['backend_connections'] = len(connections) if backend_port is not None else None
    return result


def memory_bounds(rounds, memory_window=None, *, short_gate=False):
    start, end = (rounds // 2 + 1, rounds) if memory_window is None else memory_window
    if (type(start) is not int or type(end) is not int or not 1 <= start < end <= rounds
            or (memory_window is not None and (short_gate or end - start + 1 < MEMORY_WINDOWS))):
        raise ValueError('memory window requires valid completed-round endpoints and full accounting')
    return start, end


def growth(samples, verified_records=None, *, short_gate=False, memory_window=None):
    """Compare retained resources after equal completed workload rounds.

    The first half is warmup. RSS fails only when the maxima of four ordered,
    nonempty windows in the later half all strictly rise and endpoint growth
    reaches MEMORY_BYTES_PER_RECORD_FLOOR per verified record between those
    endpoints. Explicit short gate checks report memory growth without failing it.
    Counts belong to synchronous completed rounds. A single retained RSS
    step can pass. Other resources still fail when their later-half minimum
    exceeds their warmup maximum, including a one-time retained resource leak.

    An explicit inclusive memory_window instead compares its endpoints against
    the record floor without requiring four rising maxima. Other resource
    checks retain their original half-run split.

    No process-exit sample participates. Finite RSS windows can miss growth
    hidden by allocator/page noise, growth outside the measured window, and
    reachable leaks. Passing does not prove the absence of leaks.
    """
    if len(samples) < MIN_ROUNDS:
        raise ValueError(f'growth verdict requires at least {MIN_ROUNDS} completed rounds')
    if any(value[key] is None for value in samples for key in METRICS):
        raise ValueError('growth verdict requires complete resource samples and a backend port')
    split = len(samples) // 2
    start, end = memory_bounds(len(samples), memory_window, short_gate=short_gate)
    later = samples[start - 1:end]
    checks = {}
    for key in METRICS:
        warm_max = max(value[key] for value in samples[:split])
        retained_min = min(value[key] for value in samples[split:])
        checks[key] = dict(warmup_max=warm_max, retained_min=retained_min,
                           passed=retained_min <= warm_max)
    maxima = [max(row['memory_bytes'] for row in
                  later[len(later) * index // MEMORY_WINDOWS:len(later) * (index + 1) // MEMORY_WINDOWS])
              for index in range(MEMORY_WINDOWS)]
    memory = checks['memory_bytes']
    if memory_window is not None:
        memory.update(measurement_start_round=start, measurement_end_round=end,
                      warmup_max=max(row['memory_bytes'] for row in samples[:start - 1]) if start > 1 else None,
                      retained_min=min(row['memory_bytes'] for row in later))
    memory.update(**(dict(report_only=True) if short_gate else
                     dict(floor_bytes_per_verified_record=MEMORY_BYTES_PER_RECORD_FLOOR)))
    memory.update(late_window_maxima=maxima,
                  late_half_growth_bytes=later[-1]['memory_bytes'] - later[0]['memory_bytes'])
    counts_valid = (verified_records is not None and len(verified_records) == len(samples)
                    and all(type(count) is int and count > 0 for count in verified_records))
    if counts_valid:
        # The first late checkpoint is the baseline, so its completed records
        # precede the measured interval and must not dilute its growth.
        records = sum(verified_records[start:end])
        floor = records * MEMORY_BYTES_PER_RECORD_FLOOR
        memory.update(verified_records_between_checkpoints=records,
                      bytes_per_verified_record=memory['late_half_growth_bytes'] / records,
                      passed=short_gate or not ((memory_window is not None or
                          all(before < after for before, after in zip(maxima, maxima[1:])))
                          and memory['late_half_growth_bytes'] >= floor))
    else:
        memory.update(passed=False, verdict='unverifiable', reason='missing or invalid verified record counts')
    return dict(criterion=EXTENDED_CRITERION if memory_window is not None else SHORT_GATE_CRITERION if short_gate else CRITERION,
                passed=all(c['passed'] for c in checks.values()),
                warmup_rounds=split, measured_rounds=len(samples) - split, metrics=checks,
                **({} if counts_valid else dict(verdict='unverifiable')))
