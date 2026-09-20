# Recording overwrite changes a run on replay

Status: Open

Found 2026-09-20 in the full-project review at `2c32524` and confirmed after ticket 0025.

Two identical request bodies have one digest and one file. The recorder currently replaces that file after every successful answer. A local server answered the first identical record false and the second true. The live recorded output was false then true. Replay read the one surviving file twice and answered true then true.

The folder therefore cannot both replace an entry with the newest response and promise the same answers on replay. Concurrent workers and separate processes can also race on the same final path. ADR 0020 makes the first complete response immutable and turns a different later response into a safe local failure.
