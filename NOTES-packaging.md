# Packaging rehearsal, 2026-09-21

The brief's item 5, run as the languages lane: build each surface's real installable package, install it in a clean container from the package file, and run one slide sample from the marketing deck (`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`) exactly as drawn. Version 0.0.1 everywhere; the first release is 0.1.0 and is not this. Linux first; the macOS attempts by cross-compilation follow in their own section, recorded honestly. Every container is disposable, named `pkg211-*`, removed with `docker rm -f -v`; every image pull and removal is recorded here.

The cache default, stated once for all six packages: the ruled default folder is `$XDG_CACHE_HOME/thinkthen` on Linux and `~/Library/Caches/thinkthen` on macOS. The stand-in reads `THINKTHEN_CACHE` into the settings and writes nothing to disk today — no cache code path writes files yet — so the rehearsal could not observe a folder written, for any surface. The statement holds for the real engine that lands behind the same settings.

## Python

Build, from `libraries/python`:

```
$ maturin build --release
📦 Built wheel for abi3 Python ≥ 3.10 to libraries/python/target/wheels/thinkthen-0.0.1-cp310-abi3-manylinux_2_39_x86_64.whl
```

Clean container: `docker pull python:3.12-slim` (Debian glibc 2.41, Python 3.12.14). Install from the wheel file, not from the source tree and not `maturin develop`:

```
$ docker run --rm --name pkg211-python \
    -v /tmp/pkg211/python:/work \
    -v libraries/python/target/wheels:/wheels:ro \
    python:3.12-slim bash -c \
    "pip install --quiet /wheels/thinkthen-0.0.1-cp310-abi3-manylinux_2_39_x86_64.whl pandas \
     && cd /work && ENGINE_NULL=1 python slide_sample.py"
decide          -> True   (comment: True)
band decide     -> False   (comment: None)
filter          -> []   (comment: none drawn)
annotate        -> columns ['body', 'team', 'urgency', 'wants_refund']
  three new columns: ['team', 'urgency', 'wants_refund']
  first row: {'body': 'I renewed once this morning, but my card shows two charges.\nPlease refund the duplicate.', 'team': None, 'urgency': 1.7, 'wants_refund': True}

MISMATCH  band decide: the comment says None, the run gave False; against the three-bucket stub this evidence carries no keyword, so its probability falls below the band
findings are reported, not hidden; the slide changes, not the sample
```

The `band decide` mismatch is the sample's own recorded finding, unchanged by packaging: the stand-in's keyword rule puts that evidence under the band, and the comment is the real backend's answer. `decide` and `annotate` reproduce their comments against the stand-in.

Installed-package evidence, second clean container:

```
$ pip show thinkthen | grep -E '^(Name|Version|Location)'
Name: thinkthen
Version: 0.0.1
Location: /usr/local/lib/python3.12/site-packages
$ python -c 'import thinkthen; print(thinkthen.__file__)'
/usr/local/lib/python3.12/site-packages/thinkthen/__init__.py
```

Registry use: the wheel came from the local file; `pandas` came from PyPI inside the container. Nothing was published. Pull recorded: `python:3.12-slim`. Containers: `--rm`, none left.
