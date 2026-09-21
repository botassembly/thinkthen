# The C surface

Lands in Phase B. The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
as drawn against the stand-in when this folder holds the door:

```c
#include <thinkthen.h>

thinkthen_engine *tt = thinkthen_engine_new();
thinkthen_answer answer;
int rc = thinkthen_decide(tt, "Does the customer ask for a refund?",
    text, text_len, &answer);
thinkthen_decide_many(tt, question, texts, lengths, count, out);
thinkthen_engine_free(tt);
```

The header is `contract/include/thinkthen.h`, the ruled spelling of the bulk
door is `thinkthen_decide_many`, and this surface owns every exported
symbol: the engine exports none.
