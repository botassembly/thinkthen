"""One production-mapping denominator for native and installed collections."""
import datetime
import json
from pathlib import Path

PREFIXES = ('crates/thinkthen/src/', 'libraries/c/src/')


def source_path(root, value):
    path = Path(value)
    if not path.is_absolute():
        path = root / path
    try:
        relative = path.relative_to(root).as_posix()
        if relative.startswith(PREFIXES):
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
            continue
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


def report(root, run, revision, production, collections, elapsed, *, missing=()):
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
    for path, numbers in sorted(denominator.items()):
        reached = {name: {number for number in numbers if files.get(path, {}).get(number, 0) > 0}
                   for name, files in hits.items()}
        outside = set().union(*(value for name, value in reached.items()
                                    if name not in ('source-unit', 'cli-source')))
        units = reached['source-unit'] | reached['cli-source']
        file_totals[path] = {'mapped_nonblank': len(numbers),
                            **{name: len(value) for name, value in reached.items()},
                            unit_key: len(units - outside),
                            zero_key: len(numbers - outside - units)}
    totals = {key: sum(values[key] for values in file_totals.values())
              for key in ('mapped_nonblank', *collections, unit_key, zero_key)}
    partial = not any(name.startswith('installed-') for name in collections) or bool(missing)
    scope = 'partial coverage; installed surfaces unmeasured' if partial else 'native and installed library coverage'
    payload = {'source': revision, 'scope': scope, 'unmeasured_collections': list(missing),
               'denominator': 'nonblank lines mapped by non-test core/CLI and C shared-library builds',
               'totals': totals, 'files': file_totals, 'missing_child_profiles': limitations}
    (run / 'totals.json').write_text(json.dumps(payload, indent=2, sort_keys=True) + '\n')
    rows = [f'# Coverage at {revision}', '', f'Elapsed: {elapsed:.1f} seconds.', '',
            scope + '.', '',
            'The denominator is the nonblank source lines with LLVM coverage mappings in non-test core/CLI and C builds. Each collection uses that same denominator. Test-only mappings do not enter it. Comments, declarations without mappings, disabled features and unavailable platform code are not instrumentable in this build. Zero-hit production files remain in the totals.', '',
            '| Collection | Reached | Mapped nonblank | Percent |', '| --- | ---: | ---: | ---: |']
    total = totals['mapped_nonblank']
    for name in collections:
        rows.append(f'| {name} | {totals[name]} | {total} | {100 * totals[name] / total if total else 0:.2f}% |')
    rows += ['', 'Finite RLIMIT_FSIZE children retain their product file-size limit and assertions. Only the compiler runtime writes to /dev/null. Owned limited-fsize-PID directories identify that cause. Process-PID directories without normal-exit identify children that bypassed normal exit; SIGINT, SIGTERM, other signals and _exit can do this. The inventory does not identify which signal caused an exit and cannot observe exits before its constructor. These paths are unmeasured. Zero observed hits do not establish that a path is unreached; observed unit-only hits do not establish that outside tests never reach those paths.', '']
    for name, observations in limitations.items():
        rows.append(f'{name}: {len(observations["finite_fsize"])} finite file-size observations; {len(observations["no_normal_exit"])} processes without normal-exit observations. Causes can overlap.')
    rows += ['', '`rust-integration` measures public Rust API and CLI subprocess tests together. `cli-source` runs the existing `cli::` source-unit harness; `source-unit` runs the remaining source units. Unit-only comparisons use the union of those two source-unit collections against Rust integration and all installed collections, never treating direct CLI internals as outside coverage. Each raw collection stays separate. Integration targets come from Cargo metadata with their default feature requirements. No ignored release/stress tests run.', '',
             'The JSON measurement contains the per-file counts and missing-profile observations. Build output is temporary and is not durable evidence.', '']
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
    suffix = 'native' if not any(name.startswith('installed-') for name in collections) else 'installed'
    summary = root / 'sdlc/records/coverage' / f'{datetime.date.today()}-{revision[:9]}-{suffix}.md'
    summary.parent.mkdir(parents=True, exist_ok=True)
    summary.write_text('\n'.join(rows))
    summary.with_suffix('.json').write_text(json.dumps(payload, indent=2, sort_keys=True) + '\n')
    print(summary)
