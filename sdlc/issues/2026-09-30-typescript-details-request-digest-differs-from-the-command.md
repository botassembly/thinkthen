# The TypeScript details request digest differs from the command's

Status: open, owned by ticket 0304 slice 3a. Found by the TypeScript binding check during ticket 0335 slice 1, on a branch from main `dfa7324c7`. That slice changed no library or TypeScript source, so main carries the same failure.

## The problem

`libraries/typescript/tests/shapes.test.mjs` `details equals the command --details document for the same question and text` fails. The library's `meta.requests` holds a different request digest than the command's `--details` document for the same question and text. Every other field matches. The library and the command send different request bytes, or digest them differently.

## Next step

Compare the request body each path sends for `decide 'Does it ask for a refund?'` on the loopback backend. The cause is known: ticket 0304 slice 2 changed the command's request identity, and the TypeScript library still sends the old bytes. The 0304 slice 3a branch fixes it and passes `shapes.test.mjs`. Close this issue when 3a lands.
