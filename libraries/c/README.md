# The C surface

The door every language that cannot bind Rust directly loads: one header
from `contract/include/thinkthen.h`, `libthinkthen.so` and
`libthinkthen.a` built here, and every exported symbol owned by this
surface. The engine exports none. The engine is built through the
contract's connector; the stand-in provides the connector today and the
real engine replaces it by the one line that names the connector.

The acceptance sample is the C slide in
the product deck's surfaces page,
and it runs as drawn: `./check.sh` builds the door, compiles the sample
with a plain cc, and runs it on the null backend and against the stub on
port 8216, beside the door's test suites and the surface's slice of
`conformance/conformance.json`.

```c
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_cancel_token *stop = thinkthen_cancel_token_new();
thinkthen_answer a;
int rc = thinkthen_decide(tt, "Does the customer ask for a refund?",
    text, text_len, &a);
/* the control spelling takes a budget and the token; plain calls are this
   with THINKTHEN_NO_DEADLINE and NULL */
int rc2 = thinkthen_decide_opts(tt, "Does the customer ask for a refund?",
    text, text_len, 5000, stop, &a);
thinkthen_decide_many(tt, q, texts, lens, COUNT, out);
thinkthen_cancel(stop);  /* from any thread: no new request starts */
thinkthen_cancel_token_free(stop);
thinkthen_engine_free(tt);
```

The options design is `DESIGN.md`: a cancel token fired from any thread is
how a C host hears its own interrupt, every call may carry a budget in
milliseconds, and the failure code is readable through
`thinkthen_error_code`. Facts for the contract are recorded in `NOTES.md`:
the header's doc promises the question-file grammar where the slide passes
a bare string (the door accepts both), and the deck and one design page
name the free function `thinkthen_string_free` where the header and this
library say `thinkthen_free_string` (their owners correct the line; the
door owns one name per symbol). The other findings the lane filed are
closed: the failure code is retrievable, the door carries the token and
the budget, the return-code lines match `code_of`, and the question-file
relation ends are `source`/`target` in the header, the parser, and the
conformance cases. A door-supplied poll callback stays deferred, as
`DESIGN.md` records. The ruled bulk spelling is `thinkthen_decide_many`.

The width (the engine's `width` setting or `ENGINE_WIDTH`) is the number of
requests in flight, and each in-flight request holds its own connection:
1,000 records at width 32 measured 33 pooled connections.
