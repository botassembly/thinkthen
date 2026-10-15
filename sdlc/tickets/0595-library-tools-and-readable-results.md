# 0595 — library-tools-and-readable-results

Status: OPEN.

Milestone: 0.3

Draft: Ian filed this on 2026-10-15 for the 0.3 PM to finish. The open decisions below need Ian's answers before a developer starts.

## Outcome

A Python caller makes a reusable tool once, with its function, question and options, and calls it on text in one line. The result reads like its answer in `assert`, `if` and `==`, and still carries its probability, rows and facts. The README shows the ten functions with no engine object, one call per line and its assertion beside it.

## Evidence

- Ian, 2026-10-15, reading the 0.2 Python example (`libraries/python/README-PUBLIC.md` on `ticket/0543-final`), whose example runs inside `with tt.Engine() as engine:`, loops over tables of expected answers and reads `.value` on every call:
  - "Why do we need an engine object?"
  - The examples should sit closer to the text: `from thinkthen import decide`, then `assert decide(...)` or `assert not decide(...)`.
  - "Why can't the object itself support evaluation of true, false, and none via an object reference? Same thing with equality of a string like shipping. Obviously, you would still have the object itself, and you could get at the confidence itself." He asked whether Rust or the thin Python wrapper could provide these methods.
  - "What happened to currying? Is currying still a thing where you can basically create a question with options and resolve it that way?"
  - "Wouldn't that be a tool, though? Are we going away from the word question and more toward tools?" Then: that is a 0.3 question, because toolboxes arrive in 0.3. He asked that "tools replacing question as library options" be fully recorded here.
  - The same day, for the 0.2 examples: assertions sit right beside the example and are explicit, such as `is True`, `is False`, `is None` or `== "shipping"`, in the fewest lines, under 60 characters each where possible.
- The engine is optional today. `libraries/python/thinkthen/__init__.py` defines module-level `decide`, `choose` and the other eight over one lazily created process engine that reads its settings from the environment. `Engine(...)` is needed to set `cache`, `timeout` and the other engine settings in code, and for `plan`, `iterate` and `asyncio`, which have no module-level form. `model` is also a per-call option.
- `Result` in `libraries/python/thinkthen/_calls.py` is a frozen dataclass whose fields are `results`, `terminal`, `function` and `scalar`, with `value`, `probability` and `facts` as properties. It defines no `__bool__`, `__eq__` or numeric methods, and sets `eq=False`, so `assert tt.decide(q, text)` always passes because any object is true.
- `Result` had this once. From 2026-10-09 its `__bool__` returned `bool(self.value)` and its `__eq__` compared `value`. Ticket 0581's PM decision of 2026-10-14 deleted both (commit 2303e8d21, "Return one Python call result with aligned probabilities") and said guides teach `result.value`. This ticket would overturn that decision. In 0.1.2 the native `Call.__bool__` raised a `TypeError` instead.
- Python cannot make `result is True` work. `is` tests object identity and no class can change it. An object result can support `assert r`, `assert not r`, `r == True`, `r == "shipping"`, `r > 0.7` and `float(r)`. It can never satisfy `r is True` or `r is None`.
- Currying existed in 0.1 and was dropped without a ruling on it. ADR 0107 section 1 built an immutable `Judge` when a verb was called without its input: `complaint = tt.decide("Is this a complaint?", threshold="0.3:0.7")`, then `complaint(text)`, `complaint(list)` or `complaint(column)`, one packed request per list. Commit 02178641c (2026-10-10, "Retire duplicate Python APIs and own named native sessions") deleted it with the other old judging routes; its message cites no ruling. The no-compatibility ruling of 2026-10-12 later marked ADR 0107 superseded for the 0.1 interfaces, on `ticket/0543-final` only; on main ADR 0107 still reads accepted. Today only `functools.partial(tt.decide, q)` remains.
- `sdlc/planning/libraries/README.md`, "A saved question is a value" (2026-09-20), designed `tt.question(decide=..., threshold=...)` as frozen data built from the question file's keys and callable on evidence, and ruled out automatic currying: a forgotten argument would hand `if` a function, which the README notes is true in JavaScript and Ruby; it is true in Python too. `tt.question` shipped in 0.1.0 through 0.1.2 as a frozen question value passed in the question slot, as in `tt.decide(refund, text)`; it was not callable on evidence. Commit 02178641c removed it too.
- ADR 0107 also made `Call.__bool__` raise on purpose. With truthy results, Python's built-in `filter(judge, xs)` sends one request per item where `tt.filter(q, xs)` packs them. Answers stay right; cost and time grow.
- Ruling 2026-10-14 (`sdlc/decisions/2026-10-14-three-releases-after-0-2.md`): 0.3 brings the toolbox and `run`; libraries and MCP get only the ten functions and `run`; the toolbox itself stays in Rust and the command line.

## Open decisions for Ian and the 0.3 PM

1. The word. Ian's direction is that a library's reusable configured call is a tool from the 0.3 toolbox, not a question. Decide the spelling: a tool value built in code, a tool loaded by name for `run`, or both as one type. The question text stays the question inside it.
2. How a tool is made: an explicit constructor (the PM recommends this, because a forgotten text then cannot become a silently true function), or 0.1's call-without-input judge.
3. Assertion style. Ian wants explicit assertions. With object results, `is True` and `is None` cannot work. Options: keep `.value is True` in examples; make `r == True` and a named `r.unsure` the explicit forms; or return plain values from tools and keep the result object one call away.
4. Not sure. The PM recommends that testing a not-sure decide result as true or false raises a usage error naming the explicit check, so `assert not r` never passes on a not-sure answer. A result over many records raises the same way.
5. Which languages follow Python. Ruby, C#, Kotlin, Swift, Scala, Dart, PHP and TypeScript each allow some of this; Java, Go and C cannot and keep `.value`.

## Scope

- Changes: `libraries/python/thinkthen/__init__.py`, `libraries/python/thinkthen/__init__.pyi`, `libraries/python/thinkthen/_calls.py`, `libraries/python/thinkthen/_calls.pyi`, `libraries/python/README.md`, `libraries/python/README-PUBLIC.md`, `libraries/python/examples.json` and the Python tests. The coordinator narrows this when the decisions land.
- The methods live in the thin Python layer, where `Result` is defined; the native code does not change.
- Keeps: every request byte for byte; `.value`, `.probability`, `.results` and `.facts` unchanged; a failed call raises and never reads as an answer; `tt.filter`, `tt.rank` and `tt.annotate` remain the packed paths.

## Proof

Routine suite, Python checks under replay: `assert refund(text)` and `assert not refund(other)` pass on recorded yes and no answers; `team(text) == "shipping"` passes; a not-sure result and a many-record result each raise the named usage error on a truth test; request bodies equal today's for the same calls.

## Defers

- Other languages follow in their own tickets after the Python shape is accepted.
- The toolbox, its storage and `run` belong to the 0.3 toolbox tickets; this ticket only shapes how a library holds and calls one.
- Uneven result shapes across the ten functions are ticket 0596.
