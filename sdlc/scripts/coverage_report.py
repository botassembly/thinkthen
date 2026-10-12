"""One production-mapping denominator for separate native collections."""
import datetime
import json
from pathlib import Path

PREFIXES = ('crates/thinkthen/src/', 'libraries/c/src/')


def lines(root, lcov):
    result = {}
    current = None
    for entry in lcov.splitlines():
        if entry.startswith('SF:'):
            path = Path(entry[3:])
            try:
                relative = path.relative_to(root).as_posix()
            except ValueError:
                current = None
                continue
            current = relative if relative.startswith(PREFIXES) else None
            if current:
                result.setdefault(current, {})
        elif current and entry.startswith('DA:'):
            number, count, *_ = entry[3:].split(',')
            number, count = int(number), int(count)
            result[current][number] = max(result[current].get(number, 0), count)
    return result


def report(root, run, revision, production, collections, elapsed):
    mapped = lines(root, production)
    denominator = {}
    for path, counts in mapped.items():
        source = (root / path).read_text().splitlines()
        denominator[path] = {number for number in counts if source[number - 1].strip()}
    hits = {name: lines(root, payload) for name, payload in collections.items()}
    limitations = {name: [path.name for path in sorted((run / name).glob('limited-fsize-*'))]
                   for name in collections}
    limited = any(limitations.values())
    zero_key = 'unobserved' if limited else 'unreached'
    unit_key = 'observed_unit_only' if limited else 'unit_only'
    file_totals = {}
    for path, numbers in sorted(denominator.items()):
        reached = {name: {number for number in numbers if files.get(path, {}).get(number, 0) > 0}
                   for name, files in hits.items()}
        outside = reached['rust-integration'] | reached['cli-source']
        file_totals[path] = {'mapped_nonblank': len(numbers),
                            **{name: len(value) for name, value in reached.items()},
                            unit_key: len(reached['source-unit'] - outside),
                            zero_key: len(numbers - outside - reached['source-unit'])}
    totals = {key: sum(values[key] for values in file_totals.values())
              for key in ('mapped_nonblank', *collections, unit_key, zero_key)}
    payload = {'source': revision, 'scope': 'partial native collection; installed surfaces unmeasured',
               'denominator': 'nonblank lines mapped by non-test core/CLI and C shared-library builds',
               'totals': totals, 'files': file_totals, 'missing_child_profiles': limitations}
    (run / 'totals.json').write_text(json.dumps(payload, indent=2, sort_keys=True) + '\n')
    rows = [f'# Native coverage at {revision}', '', f'Elapsed: {elapsed:.1f} seconds.', '',
            'This is a partial native measurement. Installed language collections have not run.', '',
            'The denominator is the nonblank source lines with LLVM coverage mappings in non-test core/CLI and C builds. Each collection uses that same denominator. Test-only mappings do not enter it. Comments, declarations without mappings, disabled features and unavailable platform code are not instrumentable in this build. Zero-hit production files remain in the totals.', '',
            '| Collection | Reached | Mapped nonblank | Percent |', '| --- | ---: | ---: | ---: |']
    total = totals['mapped_nonblank']
    for name in collections:
        rows.append(f'| {name} | {totals[name]} | {total} | {100 * totals[name] / total:.2f}% |')
    if limited:
        rows += ['', 'Finite RLIMIT_FSIZE children retain their product file-size limit and assertions. Only the compiler runtime writes to /dev/null. Each missing child profile has an owned limited-fsize-PID directory in its collection. Their executed paths are unmeasured. Zero observed hits do not establish that a path is unreached; observed unit-only hits do not establish that outside tests never reach those paths.', '']
        for name, observations in limitations.items():
            rows.append(f'{name}: {len(observations)} child profiles unavailable because of finite RLIMIT_FSIZE.')
    rows += ['', '`cli-source` runs the existing `cli::` source harness; `source-unit` excludes that harness. `rust-integration` runs every integration target discovered by Cargo metadata, with its default feature requirements. No ignored release/stress tests run.', '',
             f'Raw profiles, profile rejection diagnostics, collection exports and per-file counts: `{run.relative_to(root) if run.is_relative_to(root) else run.name}`.', '']
    for label, key in [('Most lines observed only in source-unit profiles' if limited else 'Most lines reached only by source-unit tests', unit_key),
                       ('Most lines with no observed hits; missing child paths are unmeasured' if limited else 'Most lines reached by no measured test', zero_key)]:
        rows += [f'## {label}', '', '| File | Lines |', '| --- | ---: |']
        for path, values in sorted(file_totals.items(), key=lambda item: (-item[1][key], item[0]))[:30]:
            rows.append(f'| {path} | {values[key]} |')
        rows.append('')
    for prefix in PREFIXES:
        rows += [f'## {prefix}', '', '| Module | Mapped nonblank | Reached by integration | Reached by CLI | Reached by units |',
                 '| --- | ---: | ---: | ---: | ---: |']
        groups = {}
        for path, values in file_totals.items():
            if not path.startswith(prefix):
                continue
            module = path.removeprefix(prefix).split('/')[0]
            group = groups.setdefault(module, {key: 0 for key in values})
            for key, value in values.items():
                group[key] += value
        for module, values in sorted(groups.items()):
            rows.append(f'| {module} | {values["mapped_nonblank"]} | {values["rust-integration"]} | {values["cli-source"]} | {values["source-unit"]} |')
        rows.append('')
    summary = root / 'sdlc/records/coverage' / f'{datetime.date.today()}-{revision[:9]}-native.md'
    summary.parent.mkdir(parents=True, exist_ok=True)
    summary.write_text('\n'.join(rows))
    print(summary)
