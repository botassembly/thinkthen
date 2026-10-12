"""One production-mapping denominator for native and installed collections."""
import datetime
import json
from pathlib import Path

UNIT_COLLECTIONS = ('cli-source', 'source-unit', 'c-source-unit')

PREFIXES = ('crates/thinkthen/src/', 'libraries/c/src/')


def source_path(root, value):
    path = Path(value)
    if path.is_relative_to('/build/source'):
        path = root / path.relative_to('/build/source')
    elif path.is_relative_to('/build/home'):
        path = Path.home() / path.relative_to('/build/home')
    if not path.is_absolute():
        path = root / path
    try:
        relative = path.relative_to(root).as_posix()
        if relative.startswith(PREFIXES):
            if not path.is_file():
                raise RuntimeError(f'coverage source is missing: {path}')
            return relative
    except ValueError:
        pass
    # Wheels build a copied Rust core. Count it only when the copy is identical.
    for prefix in PREFIXES:
        marker = '/' + prefix
        if marker not in path.as_posix():
            continue
        relative = prefix + path.as_posix().rsplit(marker, 1)[1]
        canonical = root / relative
        if not canonical.is_file() or not path.is_file():
            raise RuntimeError(f'coverage source copy is missing: {path}; canonical {canonical}')
        if path.read_bytes() != canonical.read_bytes():
            raise RuntimeError(f'coverage source copy differs from {relative}: {path}')
        return relative
    return None


def lines(root, lcov):
    result = {}
    current = None
    for entry in lcov.splitlines():
        if entry.startswith('SF:'):
            current = source_path(root, entry[3:])
            if current:
                result.setdefault(current, {})
        elif current and entry.startswith('DA:'):
            number, count, *_ = entry[3:].split(',')
            number, count = int(number), int(count)
            result[current][number] = max(result[current].get(number, 0), count)
    return result


def report(root, run, revision, production, collections, elapsed, *, missing=(), missing_reasons=None, continuous=False):
    mapped = lines(root, production)
    denominator = {}
    for path, counts in mapped.items():
        source = (root / path).read_text().splitlines()
        denominator[path] = {number for number in counts if source[number - 1].strip()}
    hits = {name: lines(root, payload) for name, payload in collections.items()}
    limitations = {name: {
        'finite_fsize': [path.name for path in sorted((run / name).glob('limited-fsize-*'))],
        'no_normal_exit': [path.name for path in sorted((run / name).glob('process-*'))
                           if not (path / 'normal-exit').exists()]}
        for name in collections}
    # Signals, _exit and failures before constructors can bypass LLVM flush.
    # Even an empty inventory cannot prove all outside paths were measured.
    zero_key = 'unobserved'
    unit_key = 'observed_unit_only'
    file_totals = {}
    line_sets = {}
    for path, numbers in sorted(denominator.items()):
        retained = line_sets[path] = {
            'mapped_nonblank': sorted(numbers),
            **{name: sorted(number for number in numbers if files.get(path, {}).get(number, 0) > 0)
               for name, files in hits.items()}}
        reached = {name: set(retained[name]) for name in collections}
        outside = set().union(*(value for name, value in reached.items()
                                    if name not in UNIT_COLLECTIONS))
        units = set().union(*(value for name, value in reached.items() if name in UNIT_COLLECTIONS))
        file_totals[path] = {key: len(value) for key, value in retained.items()}
        file_totals[path].update({unit_key: len(units - outside),
                                 zero_key: len(set(retained['mapped_nonblank']) - outside - units)})
    totals = {key: sum(values[key] for values in file_totals.values())
              for key in ('mapped_nonblank', *collections, unit_key, zero_key)}
    partial = not any(name.startswith('installed-') for name in collections) or bool(missing)
    scope = 'partial coverage; installed surfaces unmeasured' if partial else 'native and installed library coverage'
    payload = {'source': revision, 'scope': scope, 'unmeasured_collections': list(missing),
               'unmeasured_reasons': dict(missing_reasons or {}),
               'denominator': 'nonblank lines mapped by all retained production native objects',
               'profile_mode': 'continuous' if continuous else 'normal-exit',
               'totals': totals, 'files': file_totals, 'line_sets': line_sets,
               'missing_child_profiles': limitations}
    (run / 'totals.json').write_text(json.dumps(payload, indent=2, sort_keys=True) + '\n')
    rows = [f'# Coverage at {revision}', '', f'Elapsed: {elapsed:.1f} seconds.', '',
            scope + '.', '',
            'The denominator is the nonblank source lines with LLVM coverage mappings in all retained production native objects, including installed feature builds. Each collection uses that same denominator. Test-only mappings do not enter it. Comments, declarations without mappings, disabled features and unavailable platform code are not instrumentable in this build. Zero-hit production files remain in the totals.', '',
            '| Collection | Reached | Mapped nonblank | Percent |', '| --- | ---: | ---: | ---: |']
    total = totals['mapped_nonblank']
    for name in collections:
        rows.append(f'| {name} | {totals[name]} | {total} | {100 * totals[name] / total if total else 0:.2f}% |')
    rows += ['', 'Finite RLIMIT_FSIZE children retain their product file-size limit and assertions. Only the compiler runtime writes to /dev/null. Owned limited-fsize-PID directories identify that cause. Process-PID directories without normal-exit identify children that bypassed normal exit; SIGINT, SIGTERM, other signals and _exit can do this. The inventory does not identify which signal caused an exit and cannot observe exits before its constructor. Finite file-size paths are unmeasured. Zero observed hits do not establish that a path is unreached; observed unit-only hits do not establish that outside tests never reach those paths.', '']
    rows += [('Linux continuous profiles retain counters during execution, including after _exit and signals. Each process writes its own profile. Missing normal-exit observations do not imply missing counters in this mode.' if continuous else
              'Profiles flush at normal exit. Processes without a normal-exit observation may have unmeasured paths.'), '']
    for name, observations in limitations.items():
        rows.append(f'{name}: {len(observations["finite_fsize"])} finite file-size observations; {len(observations["no_normal_exit"])} processes without normal-exit observations. Causes can overlap.')
    rows += ['', '`rust-integration` measures public Rust API and CLI subprocess tests together. `cli-source` runs the existing `cli::` source-unit harness; `source-unit` runs the remaining root source units; `c-source-unit` runs only the C workspace library source-unit harness. Unit-only comparisons use the union of these source-unit collections against Rust integration and all installed collections, never treating direct CLI internals as outside coverage. Each raw collection stays separate. Integration targets come from Cargo metadata with their default feature requirements. No ignored release/stress tests run.', '',
             'The JSON measurement contains the per-file counts, exact denominator and per-collection reached line-number sets, and missing-profile observations. Build output is temporary and is not durable evidence.', '']
    for label, key in [('Most lines observed only in source-unit profiles', unit_key),
                       ('Most lines with no observed hits; missing child paths are unmeasured', zero_key)]:
        rows += [f'## {label}', '', '| File | Lines |', '| --- | ---: |']
        for path, values in sorted(file_totals.items(), key=lambda item: (-item[1][key], item[0]))[:30]:
            rows.append(f'| {path} | {values[key]} |')
        rows.append('')
    for prefix in PREFIXES:
        rows += [f'## {prefix}', '',
                 '| Module | Mapped nonblank | ' + ' | '.join(collections) + ' |',
                 '| --- | ---: | ' + ' | '.join('---:' for _ in collections) + ' |']
        groups = {}
        for path, values in file_totals.items():
            if not path.startswith(prefix):
                continue
            module = path.removeprefix(prefix).split('/')[0]
            group = groups.setdefault(module, {key: 0 for key in values})
            for key, value in values.items():
                group[key] += value
        for module, values in sorted(groups.items()):
            rows.append(f'| {module} | {values["mapped_nonblank"]} | ' +
                        ' | '.join(str(values[name]) for name in collections) + ' |')
        rows.append('')
    if missing:
        rows += ['Unmeasured collections: ' + ', '.join(missing) + '.', '']
        for name, reason in (missing_reasons or {}).items():
            rows += [f'{name}: {reason}', '']
    suffix = 'native' if not any(name.startswith('installed-') for name in collections) else 'installed'
    summary = root / 'sdlc/records/coverage' / f'{datetime.date.today()}-{revision[:9]}-{suffix}.md'
    summary.parent.mkdir(parents=True, exist_ok=True)
    summary.write_text('\n'.join(rows))
    summary.with_suffix('.json').write_text(json.dumps(payload, indent=2, sort_keys=True) + '\n')
    print(summary)
