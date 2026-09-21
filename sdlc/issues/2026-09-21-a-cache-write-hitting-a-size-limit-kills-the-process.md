# A cache write hitting a size limit kills the process

Status: Open

When the entry write fails on a file-size limit, the process dies on SIGXFSZ with no thinkthen message, and a partial temporary file stays behind. The replay path after the death stays clean, so the damage is contained, but the user sees a raw signal and a core dump notice.

## Reproduction

A true full disk needs root and was not run: `mount -t tmpfs -o size=2m` answers "must be superuser." The available proxy without root is a file-size limit. A 1.7 KB evidence text under `ulimit -f 1`, against a local stand-in, 2026-09-21:

    $ ( ulimit -f 1; thinkthen decide 'Does this ask for a refund?' \
        --url http://127.0.0.1:8806 --record fullcache < big.txt )
    File size limit exceeded
    exit 153 (SIGXFSZ), no thinkthen message, "timeout: the monitored command dumped core"

    $ ls fullcache
    .929786.0.b96d1fd0....json   # a 1024-byte partial temp file

The final entry name never appeared. A replay after the death fails cleanly at exit 5 with "holds no entry named," so the temp-name-plus-hard-link design contains the damage. SIGXFSZ is the RLIMIT_FSIZE mechanism, not a faithful ENOSPC. The reachable write-error path with a real OS error (EACCES) is handled with a clear exit 5. The hard-link refusal case was not run, because no filesystem that refuses hard links is mountable without root.

## Expected

A caught write error with the recording-folder sentence and exit 5, the same handling the permission error gets.

## How bad it is for a user

Minor. The mechanism is rare, the temp file is inert, and replay stays honest.

Found by experiment 218, wave 1, area 8.
