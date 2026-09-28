# 0163 answer drift measured from saved recordings

Measured offline on 2026-09-27. The repository recordings came from `69a58dabe7bf06887c21625a8899cadda9aeee97`; its recording files were unchanged by the cache code slice. The separate Beatles Bench archive came from the `site/examples/beatles/BENCH` pin `af538978aea39c738cc3c4d2a15aaf43c48b3a8a`, read with `git archive` from the local clone without checkout or fetch. No benchmark fixture was copied into this repository, and no provider call ran.

Only `thinkthen.recording/1` JSON entries count. A repeated digest appears in at least two different folders of one set. For each shared question and label pair, the gap is the largest minus the smallest stored probability across that digest's entries. `noul` uses `noul`; `choice` and `score` use values from `probabilities`. Confidence, score, choice, legend, usage and model are excluded. A crossing has a value below 0.5 and another at or above 0.5 for the same pair. The sets stay separate; the repository's probe and other recordings mix model keys, so no question-text joining occurs.

| Set | Entries | Repeated digests | Gap above 0 | Gap above 0.1 | Cross 0.5 | Largest gap |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `probes/01`–`09` | 879 | 0 | 0 | 0 | 0 | 0 |
| Other tracked repository recordings outside `site/` | 2,618 | 3 | 0 | 0 | 0 | 0 |
| Beatles Bench at the pinned commit | 21,584 | 5,111 | 4,075 | 310 | 430 | 0.45 |

This measurement uses the ticket's defined probability-pair rule. Report 09's 263 gaps above 0.1 used an unstated comparison rule and is retained as historical evidence, not substituted for the measured 310 here.

The script run from the repository root was:

```python
import collections, io, json, pathlib, subprocess, tarfile

pin = 'af538978aea39c738cc3c4d2a15aaf43c48b3a8a'
bench = str(pathlib.Path.home() / 'workspace/repos/beatles-bench')
repo = []
for name in subprocess.check_output(['git', 'ls-files', '-z']).decode().split('\0'):
    if name.endswith('.json') and not name.startswith('site/') and pathlib.Path(name).is_file():
        try: data = json.loads(pathlib.Path(name).read_bytes())
        except (ValueError, UnicodeError): continue
        if isinstance(data, dict) and data.get('schema') == 'thinkthen.recording/1': repo.append((name, data))
raw = subprocess.check_output(['git', '-C', bench, 'archive', pin])
archive = tarfile.open(fileobj=io.BytesIO(raw))
bench_entries = []
for member in archive:
    if not member.isfile() or not member.name.endswith('.json'): continue
    try: data = json.load(archive.extractfile(member))
    except (ValueError, UnicodeError): continue
    if isinstance(data, dict) and data.get('schema') == 'thinkthen.recording/1': bench_entries.append((member.name, data))

def measure(entries):
    groups = collections.defaultdict(list)
    for name, data in entries:
        if len(pathlib.Path(name).stem) == 64: groups[pathlib.Path(name).stem].append((name, data))
    total = diff = big = cross = 0
    maximum = 0.0
    for digest, items in groups.items():
        if len({str(pathlib.Path(n).parent) for n, _ in items}) < 2: continue
        total += 1
        vals = collections.defaultdict(list)
        for _, data in items:
            for question, answer in data.get('response', {}).get('answers', {}).items():
                if answer.get('type') == 'noul' and isinstance(answer.get('noul'), (int, float)):
                    vals[(question, 'noul')].append(answer['noul'])
                elif answer.get('type') in ('choice', 'score'):
                    for label, p in answer.get('probabilities', {}).items():
                        if isinstance(p, (int, float)): vals[(question, label)].append(p)
        gaps = [max(v) - min(v) for v in vals.values() if len(v) > 1]
        gap = max(gaps, default=0.0)
        maximum = max(maximum, gap)
        diff += gap > 0
        big += gap > 0.1
        cross += any(min(v) < 0.5 <= max(v) for v in vals.values() if len(v) > 1)
    return len(entries), total, diff, big, cross, maximum

probes = [(n, d) for n, d in repo if n.startswith(tuple(f'probes/{i:02d}-' for i in range(1, 10)))]
rest = [(n, d) for n, d in repo if (n, d) not in probes]
for label, entries in [('probes', probes), ('rest', rest), ('bench', bench_entries)]:
    print(label, measure(entries))
```
