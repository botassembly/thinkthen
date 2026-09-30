# rank returns no probability at the C door

Status: open. Filed 2026-09-30 by ticket 0354. Owner: queue owner.
Kind: bug
Blocks 0.1: no. The specification promises every record in order, and the door gives that.

The C door's `rank` value is the records in order, most likely yes first (`libraries/c/src/call.rs`, the `"rank"` arm). Python, TypeScript, Ruby and R return each record's place and yes probability beside it. So the 13 languages on the C door cannot read a rank probability, as `find` could not before ticket 0354. The likely fix mirrors 0354: the door writes `{"index","record","probability"}` rows from a crate type the schema derives, and the type corpus pins one case.
