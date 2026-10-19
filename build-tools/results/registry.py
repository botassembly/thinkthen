"""The outputs owned by the schema generators, grouped by producer invocation."""
from dataclasses import dataclass
import json
from pathlib import Path


@dataclass(frozen=True)
class Output:
    target: str
    mode: str
    path: str
    producer: str = 'results'


OUTPUTS = tuple(
    Output(source.stem, **entry)
    for source in sorted(Path(__file__).with_suffix('').glob('*.json'))
    for entry in json.loads(source.read_text())
)


def generated_paths():
    return {output.path for output in OUTPUTS}


def attributes(source):
    """Derive generated-file labels without replacing unrelated Git attributes."""
    retained = []
    for line in source.splitlines():
        fields = line.split()
        if not fields or fields[0].startswith('#') or 'linguist-generated' not in fields[1:]:
            retained.append(line)
            continue
        fields = [field for field in fields if field != 'linguist-generated']
        if len(fields) > 1:
            retained.append(' '.join(fields))
    return '\n'.join([*retained, *(path + ' linguist-generated'
                                 for path in sorted(generated_paths()))]) + '\n'


def invocations():
    return tuple(dict.fromkeys((output.producer, output.target, output.mode)
                              for output in OUTPUTS))


def paths(target, mode):
    return tuple(output.path for output in OUTPUTS
                 if output.target == target and output.mode == mode)


def output_path(root, target, name):
    candidates = [output.path for output in OUTPUTS
                  if output.target == target and output.path.endswith('/' + name)]
    if len(candidates) != 1:
        raise ValueError(f'unknown or ambiguous generated output: {target}/{name}')
    return root / candidates[0]
