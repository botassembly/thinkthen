# The C surface

The door every language that cannot bind Rust directly loads: one header
from `contract/include/thinkthen.h`, `libthinkthen.so` and
`libthinkthen.a` built here, and every exported symbol owned by this
surface. The engine exports none. The stand-in implements the contract
today; the real engine replaces it with one changed dependency.

The acceptance sample is the C slide in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`,
and it runs as drawn: `./check.sh` builds the door, compiles the sample
with a plain cc, and runs it on the null backend and against the stub on
port 8216, beside the door's test suites and the surface's slice of
`conformance/conformance.json`.

```c
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_answer a;
int rc = thinkthen_decide(tt, "Does the customer ask for a refund?",
    text, text_len, &a);
thinkthen_decide_many(tt, q, texts, lens, COUNT, out);
thinkthen_engine_free(tt);
```

Three findings for the contract are recorded in `NOTES.md`: the header's
doc promises the question-file grammar where the slide passes a bare
string (the door accepts both), the JSON door's failure code is not
retrievable through the current header, and the header carries no cancel
token, deadline, or poll callback. The ruled bulk spelling is
`thinkthen_decide_many`.
