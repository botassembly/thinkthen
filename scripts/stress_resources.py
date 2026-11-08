"""Linux ownership and distinct TCP connection accounting for offline stress runs."""
import ctypes
import json
import os
from pathlib import Path

METRICS = ('memory_bytes', 'descriptors', 'backend_connections', 'threads', 'processes')
CRITERION = 'retained-round-growth-per-record-floor'
SHORT_GATE_CRITERION = 'retained-round-growth-short-gate-report'
EXTENDED_CRITERION = 'retained-round-explicit-memory-window-per-record-floor'
MEMORY_REPORT_CRITERION = 'retained-round-growth-exact-counts-memory-report'
EXTENDED_REPORT_CRITERION = 'retained-round-explicit-memory-window-memory-report'
MEMORY_BYTES_PER_RECORD_FLOOR = 1
MEMORY_WINDOWS = 4
MIN_ROUNDS = 2 * MEMORY_WINDOWS
EXTENDED_ROUNDS = 60
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
        except (FileNotFoundError, ProcessLookupError):
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
                except (FileNotFoundError, ProcessLookupError):
                    # Observed sockets need a readable table, even if their
                    # process vanished after we collected its descriptors.
                    if any(owner == namespace for owner, _ in inodes):
                        raise
                    continue
                for line in rows:
                    fields = line.split()
                    if (namespace, fields[9]) in inodes and int(fields[2].split(':')[1], 16) == backend_port:
                        connections.add((namespace, protocol, fields[1], fields[2]))
    result['distinct_sockets'] = len(inodes)
    result['backend_connections'] = len(connections) if backend_port is not None else None
    return result


def extended_size(environ=None):
    """Return the extended soak's rounds, judged window and record override.

    THINKTHEN_SOAK_ROUNDS (default 60) and THINKTHEN_SOAK_RECORDS (default: the
    table's full count) shrink the soak only to prove its wiring. The judged
    window is the last third, and at least four rounds: 41 to 60 for 60 rounds,
    5 to 8 for 8.
    """
    environ = os.environ if environ is None else environ
    rounds, records = environ.get('THINKTHEN_SOAK_ROUNDS', str(EXTENDED_ROUNDS)), environ.get('THINKTHEN_SOAK_RECORDS')
    if not rounds.isdecimal() or int(rounds) < MIN_ROUNDS or (records is not None and (not records.isdecimal() or int(records) < 1)):
        raise ValueError(f'THINKTHEN_SOAK_ROUNDS must be at least {MIN_ROUNDS} and THINKTHEN_SOAK_RECORDS a positive count')
    rounds = int(rounds)
    start = min(rounds - rounds // 3 + 1, rounds - MEMORY_WINDOWS + 1)
    return rounds, (start, rounds), None if records is None else int(records)


def full_size(environ=None):
    """Whether the extended soak runs at its release size, the only size a receipt may pass."""
    environ = os.environ if environ is None else environ
    return (environ.get('THINKTHEN_SOAK_ROUNDS', str(EXTENDED_ROUNDS)) == str(EXTENDED_ROUNDS)
            and 'THINKTHEN_SOAK_RECORDS' not in environ)


def memory_bounds(rounds, memory_window=None, *, short_gate=False):
    start, end = (rounds // 2 + 1, rounds) if memory_window is None else memory_window
    if (type(start) is not int or type(end) is not int or not 1 <= start < end <= rounds
            or (memory_window is not None and (short_gate or end - start + 1 < MEMORY_WINDOWS))):
        raise ValueError('memory window requires valid completed-round endpoints and full accounting')
    return start, end


def criterion(*, short_gate=False, memory_window=None, memory_report_only=False):
    if memory_window is not None:
        return EXTENDED_REPORT_CRITERION if memory_report_only else EXTENDED_CRITERION
    return SHORT_GATE_CRITERION if short_gate else MEMORY_REPORT_CRITERION if memory_report_only else CRITERION


def growth(samples, verified_records=None, *, short_gate=False, memory_window=None, memory_report_only=False):
    """Compare retained resources after equal completed workload rounds.

    The first half is warmup. The later half splits into four ordered,
    nonempty windows. RSS growth is the last window's high minus the first
    window's high, divided by the records completed after the first window.
    Window highs, not single round samples, keep one low allocator sample
    from inflating growth: saved C++ round samples alternated 41 and 62 MB.
    RSS fails only when the four highs all strictly rise and growth reaches
    MEMORY_BYTES_PER_RECORD_FLOOR per record. Explicit short gate checks
    report memory without failing it; counts fail only when their four
    late-window peaks never fall and rise at least twice. memory_report_only
    reports memory the same way but keeps the exact count checks below; with
    a memory_window it reports that window's figures, as a soak below
    release size does, since its few small rounds measure allocator noise.
    Counts belong to synchronous completed rounds. A single retained RSS
    step can pass. Other resources still fail when their later-half minimum
    exceeds their warmup maximum, including a one-time retained resource leak.

    An explicit inclusive memory_window instead compares its window highs
    against the record floor without requiring four rising maxima. Other resource
    checks retain their original half-run split.

    No process-exit sample participates. Finite RSS windows can miss growth
    hidden by allocator/page noise, growth outside the measured window, and
    reachable leaks. Passing does not prove the absence of leaks.
    """
    if memory_report_only and short_gate:
        raise ValueError('short gate checks already report memory')
    if len(samples) < MIN_ROUNDS:
        raise ValueError(f'growth verdict requires at least {MIN_ROUNDS} completed rounds')
    if any(value[key] is None for value in samples for key in METRICS):
        raise ValueError('growth verdict requires complete resource samples and a backend port')
    split = len(samples) // 2
    start, end = memory_bounds(len(samples), memory_window, short_gate=short_gate)
    later = samples[start - 1:end]
    windows = [later[len(later) * index // MEMORY_WINDOWS:len(later) * (index + 1) // MEMORY_WINDOWS]
               for index in range(MEMORY_WINDOWS)]
    checks = {}
    for key in METRICS:
        warm_max = max(value[key] for value in samples[:split])
        retained_min = min(value[key] for value in samples[split:])
        checks[key] = dict(warmup_max=warm_max, retained_min=retained_min,
                           passed=retained_min <= warm_max)
    if short_gate:
        for key in METRICS[1:]:
            peaks = [max(row[key] for row in window) for window in windows]
            steps = list(zip(peaks, peaks[1:]))
            checks[key].update(late_window_maxima=peaks,
                               passed=(any(before > after for before, after in steps) or
                                       sum(before < after for before, after in steps) < 2))
    maxima = [max(row['memory_bytes'] for row in window) for window in windows]
    memory = checks['memory_bytes']
    if memory_window is not None:
        memory.update(measurement_start_round=start, measurement_end_round=end,
                      warmup_max=max(row['memory_bytes'] for row in samples[:start - 1]) if start > 1 else None,
                      retained_min=min(row['memory_bytes'] for row in later))
    reported = short_gate or memory_report_only
    memory.update(**(dict(report_only=True) if reported else
                     dict(floor_bytes_per_verified_record=MEMORY_BYTES_PER_RECORD_FLOOR)))
    memory.update(late_window_maxima=maxima, growth_measure='late-window-highs',
                  late_half_growth_bytes=maxima[-1] - maxima[0])
    counts_valid = (verified_records is not None and len(verified_records) == len(samples)
                    and all(type(count) is int and count > 0 for count in verified_records))
    if counts_valid:
        # The first window's high may fall on its last round, so only records
        # completed after that window count against the growth.
        records = sum(verified_records[start - 1 + len(windows[0]):end])
        floor = records * MEMORY_BYTES_PER_RECORD_FLOOR
        memory.update(verified_records_between_checkpoints=records,
                      bytes_per_verified_record=memory['late_half_growth_bytes'] / records,
                      passed=reported or not ((memory_window is not None or
                          all(before < after for before, after in zip(maxima, maxima[1:])))
                          and memory['late_half_growth_bytes'] >= floor))
    else:
        memory.update(passed=False, verdict='unverifiable', reason='missing or invalid verified record counts')
    return dict(criterion=criterion(short_gate=short_gate, memory_window=memory_window,
                                    memory_report_only=memory_report_only),
                passed=all(c['passed'] for c in checks.values()),
                warmup_rounds=split, measured_rounds=len(samples) - split, metrics=checks,
                **({} if counts_valid else dict(verdict='unverifiable')))


SOAK_WINDOW = (41, 60)
SOAK_FIGURES = ('measurement_start_round', 'measurement_end_round', 'late_half_growth_bytes',
                'verified_records_between_checkpoints', 'bytes_per_verified_record', 'floor_bytes_per_verified_record')


def soak_attributions(root):
    """Read the reviewed allocator-retention attributions; each names its public known-limits row."""
    table = json.loads((Path(root) / 'conformance/soak-attributions.json').read_text())
    entries = table['soaks']
    for name, entry in entries.items():
        if set(entry) != {'allocator', 'limits', 'row'} or not all(type(value) is str and value for value in entry.values()):
            raise ValueError('invalid soak attribution: ' + name)
        rows = [line for line in (Path(root) / entry['limits']).read_text().splitlines()
                if line.startswith('| ' + entry['row'] + ' |')]
        if len(rows) != 1 or 'attributed to ' + entry['allocator'] + ' allocator retention' not in rows[0]:
            raise ValueError('soak attribution lacks its known-limits row: ' + name)
    return entries


def soak_verdict(rows, records, name, attributions, rounds=EXTENDED_ROUNDS, window=SOAK_WINDOW, *, report_only=False):
    """Judge one soak over its judged window, rounds 41 to 60 at release size.

    A soak passes on RSS. Only a listed soak may fail RSS, every other resource still
    passing, and then only when three series pass the same floor over the same window:
    the allocator's memory in use, the runtime's live object heap, and RSS less the
    storage the allocator has mapped, so the growth must sit in storage the allocator
    keeps. The listing alone passes nothing. Below release size, report_only reports
    memory without judging it, as growth() does.
    """
    samples = [row for row in rows if row.get('completed_round') is not None]
    if [row['completed_round'] for row in samples] != list(range(1, rounds + 1)):
        raise ValueError(f'soak did not complete {rounds} rounds: ' + name)
    counts = [records] * rounds
    report = growth(samples, counts, memory_window=window, memory_report_only=report_only)
    memory = report['metrics']['memory_bytes']
    figures = {key: memory.get(key) for key in SOAK_FIGURES}
    if report['passed']:
        return dict(figures, verdict='passed')
    attribution = attributions.get(name)
    others = all(check['passed'] for key, check in report['metrics'].items() if key != 'memory_bytes')
    if attribution is None or not others or report.get('verdict') == 'unverifiable':
        raise ValueError('soak failed its growth criterion: ' + name)
    for key, label in (('allocator_in_use_bytes', 'allocator memory in use'), ('runtime_heap_bytes', 'runtime heap'),
                       ('allocator_mapped_bytes', 'allocator mapped storage'), ('memory_bytes', 'RSS')):
        if any(type(row.get(key)) is not int for row in samples):
            raise ValueError(f'attributed soak lacks {label} samples: ' + name)
    outside = [dict(row, outside_allocator_bytes=row['memory_bytes'] - row['allocator_mapped_bytes']) for row in samples]
    series = {}
    for key, label in (('allocator_in_use_bytes', 'allocator memory in use'), ('runtime_heap_bytes', 'runtime heap'),
                       ('outside_allocator_bytes', 'RSS outside allocator storage')):
        series[key] = growth([dict(row, memory_bytes=row[key]) for row in outside],
                             counts, memory_window=window)['metrics']['memory_bytes']
        if not series[key]['passed']:
            raise ValueError(f'attributed soak grew {label}: ' + name)
    return dict(figures, verdict='attributed', allocator=attribution['allocator'], limits_row=attribution['row'],
                **{key.removesuffix('_bytes'): {figure: series[key][figure] for figure in SOAK_FIGURES} for key in series})
