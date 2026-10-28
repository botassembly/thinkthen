"""Collect live process samples and synchronous completed-round checkpoints."""
import json
import os
from pathlib import Path
import socket
import tempfile
import threading
import time

from stress_resources import AccessLost, CRITERION, SHORT_GATE_CRITERION, growth, sample


class Monitor:
    def __init__(self, roots, backend_port=None, output=None):
        self.roots = roots
        self.backend_port = backend_port
        self.output = output
        self.scratch = tempfile.TemporaryDirectory(prefix='stress-samples-')
        self.path = str(Path(self.scratch.name) / 'checkpoint')
        self.server = socket.socket(socket.AF_UNIX)
        self.server.bind(self.path)
        self.server.listen()
        self.server.settimeout(0.1)
        self.stop = threading.Event()
        self.rounds = []
        self.groups = {}
        self.ports = {}
        self.required_pids = {}
        self.concurrencies = {}
        self.process_samples = {}
        self.contexts = {}
        self.peak = {}
        self.first = self.last = None
        self.count = 0
        self.error = None
        self.tracked = {}
        self.observation_failures = {}
        self.group = "workload"
        self.pending_failures = []
        self.process_events = []
        self.sequence = 0
        self.started = time.monotonic()
        self.thread = threading.Thread(target=self.collect)
        self.thread.start()

    def record(self, exclude=(), checkpoint=False, group="workload", required_pid=None, context=None):
        details = {}
        self.sequence += 1
        observation = dict(sequence=self.sequence, elapsed_seconds=time.monotonic() - self.started,
                           phase='checkpoint' if checkpoint else ('teardown' if self.stop.is_set() else 'periodic'),
                           group=group if checkpoint else self.group, completed_rounds=len(self.rounds))
        events = []
        if checkpoint:
            self.group = group
            if self.pending_failures:
                failures = self.observation_failures.setdefault(group, [])
                failures.extend(value for value in self.pending_failures if value not in failures)
                self.pending_failures.clear()
        try:
            values = sample(self.roots(), self.backend_port, [os.getpid(), *exclude], required_pid,
                            details, self.tracked, events)
        except AccessLost as error:
            error.failure.update(observation)
            events.append(dict(event='access_denied', **error.failure))
            failures = (self.observation_failures.setdefault(group, []) if checkpoint
                        else self.pending_failures)
            if error.failure not in failures:
                failures.append(error.failure)
            return
        finally:
            for event in events:
                recorded = dict(observation, event_sequence=len(self.process_events) + 1, **event)
                self.process_events.append(recorded)
                if self.output:
                    self.output.write(json.dumps(recorded) + '\n')
                    self.output.flush()
        self.count += 1
        if values['processes']:
            self.first = self.first or values
            self.last = values
        for key, value in values.items():
            if value is not None:
                self.peak[key] = max(self.peak.get(key, 0), value)
        if checkpoint:
            self.rounds.append(values)
            self.groups.setdefault(group, []).append(values)
            self.required_pids.setdefault(group, []).append(required_pid)
            self.process_samples.setdefault(group, []).append(details)
            self.contexts.setdefault(group, []).append(context)
        if self.output:
            self.output.write(json.dumps(dict(elapsed_seconds=time.monotonic() - self.started,
                completed_round=len(self.groups[group]) if checkpoint else None,
                group=group if checkpoint else None, **values)) + '\n')
            self.output.flush()

    def collect(self):
        try:
            while not self.stop.is_set():
                try:
                    connection, _ = self.server.accept()
                except socket.timeout:
                    self.record()
                    continue
                with connection:
                    request = b''
                    while not request.endswith(b'\n'):
                        block = connection.recv(1024)
                        if not block or len(request) > 1024:
                            raise ValueError('invalid resource checkpoint')
                        request += block
                    event = json.loads(request)
                    group = event.get('group', 'workload')
                    if event.get('concurrency') is not None:
                        concurrency = int(event['concurrency'])
                        if concurrency < 1 or self.concurrencies.get(group, concurrency) != concurrency:
                            raise ValueError('concurrency changed during a soak')
                        self.concurrencies[group] = concurrency
                    if event.get('backend_port') is not None:
                        port = int(event['backend_port'])
                        if self.ports.get(group, port) != port:
                            raise ValueError('backend port changed during a soak')
                        self.ports[group] = port
                        self.backend_port = port
                    self.record(exclude=[event['pid']], checkpoint=True, group=group, required_pid=event['required_pid'], context=event.get('context'))
                    connection.sendall(b'sampled\n')
        except BaseException as error:
            self.error = error
        finally:
            self.server.close()

    def finish(self, expected_rounds=None, expected_groups=None, verified_records=None, expected_context=None, *, short_gate=False):
        criterion = SHORT_GATE_CRITERION if short_gate else CRITERION
        self.stop.set()
        self.thread.join()
        self.server.close()
        self.scratch.cleanup()
        if self.error:
            raise self.error
        if self.pending_failures:
            failures = self.observation_failures.setdefault(self.group, [])
            failures.extend(value for value in self.pending_failures if value not in failures)
        report = dict(samples=self.count, first=self.first, last=self.last, peak=self.peak,
                      memory_measure='sum of owned process RSS bytes', backend_port=self.backend_port,
                      process_events=self.process_events)
        if expected_rounds is not None:
            if not self.groups and not self.observation_failures:
                raise ValueError('no completed-round samples were received')
            observed_groups = set(self.groups) | set(self.observation_failures)
            if expected_groups is not None and observed_groups != set(expected_groups):
                if not self.observation_failures:
                    raise ValueError(f'expected checkpoint groups {expected_groups}, got {list(self.groups)}')
                report['missing_checkpoint_groups'] = sorted(set(expected_groups) - observed_groups)
                report['unexpected_checkpoint_groups'] = sorted(observed_groups - set(expected_groups))
            reports = {}
            for group in dict.fromkeys([*self.groups, *self.observation_failures]):
                rounds = self.groups.get(group, [])
                if group in self.observation_failures or (self.observation_failures and len(rounds) != expected_rounds):
                    reports[group] = dict(round_samples=rounds,
                        process_samples=self.process_samples.get(group, []),
                        required_live_pids=self.required_pids.get(group, []),
                        observation_failures=self.observation_failures.get(group, []),
                        expected_rounds=expected_rounds,
                        growth=dict(criterion=criterion, passed=False, verdict='unverifiable'))
                    continue
                if len(rounds) != expected_rounds:
                    raise ValueError(f'{group}: expected {expected_rounds} completed-round samples, got {len(rounds)}')
                contexts = self.contexts[group]
                counts = ([(row or {}).get('verified_records') for row in contexts]
                          if verified_records is None else verified_records.get(group))
                if expected_context is not None and any(row != expected_context for row in contexts):
                    counts = None
                identifier_masks = [row['identifier_width_mask'] for row in contexts
                                    if row and 'identifier_width_mask' in row]
                identifiers = {}
                if identifier_masks:
                    mask = 0
                    for value in identifier_masks:
                        mask |= value
                    widths = [width for width in range(mask.bit_length()) if mask & (1 << width)]
                    changes = [index + 1 for index, value in enumerate(identifier_masks)
                               if value.bit_count() != 1 or (index and value != identifier_masks[index - 1])]
                    identifiers = dict(observed_widths=widths, min_width=min(widths, default=None),
                        max_width=max(widths, default=None), width_changed=len(widths) > 1,
                        round_width_changes=changes,
                        measured_round_width_changes=[value for value in changes if value > len(rounds) // 2])
                reports[group] = dict(identifiers=identifiers, first=rounds[0], last=rounds[-1],
                    peak={key: max(row[key] for row in rounds) for key in rounds[0]},
                    peak_scope='completed-round samples', backend_port=self.ports.get(group, self.backend_port),
                    process_samples=self.process_samples[group], context=self.contexts[group],
                    concurrency=self.concurrencies.get(group), required_live_pids=self.required_pids[group], round_samples=rounds, growth=growth(rounds, counts, short_gate=short_gate))
            report.update(groups=reports)
            if self.rounds:
                report.update(first=self.rounds[0], last=self.rounds[-1])
            report['growth'] = (reports['workload']['growth'] if set(reports) == {'workload'} else
                dict(criterion=criterion, passed=all(r['growth']['passed'] for r in reports.values()),
                     **(dict(verdict='unverifiable') if any(r['growth'].get('verdict') == 'unverifiable'
                         for r in reports.values()) else {})))
        if self.observation_failures:
            report['observation_failures'] = self.observation_failures
            report['growth'] = dict(criterion=criterion, passed=False, verdict='unverifiable')
        return report
