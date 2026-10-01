# find returns no probability in the languages on the C interface

Status: closed by ticket 0354. Filed 2026-09-30 from the release QA language grid. Owner: queue owner.
Kind: bug
Blocks 0.1: yes, if the specification promises the probability on every surface.

Release QA reports that `find` returns no probability in the 13 languages built on the C interface, while the command and the other libraries return one. Check `specification/find.md` and the C door's `find` result first. Then decide whether the door drops the field or the ports do not read it, and add the field to the conformance case so every surface proves it.

Resolution: ticket 0354 slice B. The C door dropped the probability; the ports read the door value as host JSON. The door's `find` value is now `{"index","unit","probability"}` or `null`, from a crate type the result schema derives. Type corpus case `18-find-second` pins probability 0.8, so all 13 languages on the C door prove it with no binding code.
