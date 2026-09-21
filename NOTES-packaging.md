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

## TypeScript

Build, from `libraries/typescript` (napi CLI from the folder's own dev-dependencies):

```
$ npm run build
> napi build --release --platform --cargo-cwd ./addon --js loader.cjs --dts loader.d.ts .
$ npm pack
thinkthen-0.0.1.tgz
$ tar tzf thinkthen-0.0.1.tgz
package/loader.cjs
package/index.js
package/package.json
package/README.md
package/index.mjs
package/index.linux-x64-gnu.node
package/index.d.ts
```

Clean container: `docker pull node:22-slim` (Node v22.23.2). Install from the tarball, then run the deck's TypeScript sample exactly as drawn:

```
$ docker run --rm --name pkg211-typescript \
    -v /tmp/pkg211/typescript:/work -v libraries/typescript:/pkg:ro node:22-slim bash -c \
    "mkdir -p /app && cd /app && npm init -y >/dev/null && npm install --silent /pkg/thinkthen-0.0.1.tgz \
     && cp /work/sample.mjs . && ENGINE_NULL=1 node sample.mjs"
ThinkThenError: a choose reply carried no choice
    at invoke (/app/node_modules/thinkthen/index.js:104:11)
    at async Module.choose (/app/node_modules/thinkthen/index.js:144:11)
  kind: 'defect',
  retryable: false
```

**Finding, the deck's TypeScript sample cannot run as drawn.** The one-shape ruling of 2026-09-21 (`sdlc/issues/2026-09-21-one-shape-for-nine-surfaces-as-the-slides-show-it.md:32`) says "TypeScript takes one options object last: `tt.choose(question, text, { options })`, `tt.tag(question, text, { labels })`, `tt.rank(question, records, { top, signal })`". The binding implements the last object as call options only (`signal`, deadline) and builds the question from the first argument alone; a bare string first argument becomes a `decide` question, so `choose` and `tag` raise `defect` and `rank` ignores `top`. Host probe, null backend and wire stub both:

```
choose {options}-last  -> ERROR defect | a choose reply carried no choice
choose question-object -> "the refund team"
tag {labels}-last      -> ERROR defect | a tag reply carried no labels
tag question-object    -> []
rank {top,signal}-last -> 3 rows returned for { top: 2 }   (top ignored)
rank question-object   -> 3 rows
```

The transport works: `signal` in the last object reached the engine (the lane's cancel test used it). The gap is the merge of question options (`options`, `labels`, `top`) from the ruled last object into the question. This is a TypeScript surface defect, not a packaging defect; the packaging lane records it and does not fix it. The lane that landed the surface ran the deck section as it stood then (decide/band/filter/annotate); the current section (choose/tag/rank) is new, and this run is its first.

**The installed package answers, through the shape the binding implements** (question object first, call options last), in the same container:

```
$ ENGINE_NULL=1 node sample-supported.mjs
choose -> "the refund team"
tag -> []
rank[0] -> {"index":0,"record":"I want a refund for order 9","probability":0.97}
```

`tag` returning `[]` is the stand-in's rule (no option text carries a keyword); the deck's `["billing", "shipping", "urgent"]` is the real backend's answer. The package installs from the tarball, loads its prebuilt `.node`, and answers all three verbs. Registry use: the tarball came from the local file; nothing was published. Pull recorded: `node:22-slim`. Containers: `--rm`, none left.

## Ruby

Build, from `libraries/ruby` (no Ruby on this host; `build.sh` runs inside a locally built Debian trixie image, `ruby:3.4-trixie` plus `libclang-dev`, mounting the host's Rust toolchain read-only; the image is never pushed and is removed below):

```
$ ./build.sh
built: thinkthen-0.0.1.gem and lib/thinkthen/thinkthen.so
$ ls -la thinkthen-0.0.1.gem
-rw-r--r-- 1 root root 1718272 thinkthen-0.0.1.gem   (sha256 b05dd4d5b40d2cc8…)
```

Clean container, no Rust toolchain present, installing the gem from the local file into a clean `ruby:3.4-trixie` account:

```
$ docker run --rm --name pkg211-ruby \
    -v /tmp/pkg211/ruby:/work -v libraries/ruby:/pkg:ro ruby:3.4-trixie bash -c \
    "which cargo rustc gem ruby; gem install --local --quiet /pkg/thinkthen-0.0.1.gem \
     && gem list thinkthen && cd /tmp && ENGINE_NULL=1 ruby /work/slide_sample.rb"
/usr/local/bin/ruby
Successfully installed thinkthen-0.0.1
1 gem installed
thinkthen (0.0.1)
finding: score returned 1.7, the slide comment says 2.0; nearest level is 'Immediate.'
3 of 8 are complaints
I want a refund for order 9
The refund never arrived
maybe escalate this one
Where is my order?
Hello team
slide sample green
```

No `cargo` or `rustc` was found in the container; the gem carries the compiled extension and installs from the local file alone. The slide sample is the deck's Ruby section run as drawn: filter keeps 3 of 8, rank returns the deck's five, score returns 1.7 with "Immediate." nearest — the score-comment finding is the sample's own, already filed for the slide owner in the marketing repo. Registry use: none; the gem came from the local file. Pulls recorded this section: none new (`ruby:3.4-trixie` was already local from the surface's own work). The builder image `thinkthen-ruby-builder:local` was rebuilt by `build.sh` and removed after the gem was built: `docker rmi thinkthen-ruby-builder:local`.


