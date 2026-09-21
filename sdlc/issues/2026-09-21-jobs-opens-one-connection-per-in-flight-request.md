# `--jobs N` opens one connection per in-flight request

Status: Open

`--jobs N` bounds the requests in flight. Each in-flight request holds its own connection for the life of the request. At `--jobs 32` a busy run opens up to 32 simultaneous connections to the backend; at `--jobs 4` it holds about 7 file descriptors in total.

## Reproduction

A sampler on `/proc/PID` during record runs against a loopback stand-in, 2026-09-21:

    $ thinkthen filter 'Does this ask for a refund?' --jsonl --field /body --jobs 32 ...
    (30-35 open fds sampled while 32 requests were in flight)
    $ thinkthen filter '...' --jobs 4 ...
    (7 fds sampled)

`specification/records.md` says `jobs` bounds how many requests are in flight. It says nothing about connections, and a backend or a proxy that caps connections per client will refuse or queue a run that opens 32 of them. A user meeting a connection cap has no page that names this behavior.

## What is needed

One line in the `--jobs` help and in `records.md`: the number of connections a run opens is the number of requests in flight. Connection reuse across the batch would be the code answer if it is cheap; the sentence is the minimum.

## How bad it is for a user

Surprise. Nothing breaks until a proxy counts connections, and then the failure names neither the cause nor the lever.

Found by experiment 218, wave 1, area 5.
