# The C door's churn probe still crashes in a new thread's start

Found: 2026-09-23, review 5 fixer lane (standin). Severity: high.

## What was observed

The churn probe drives the C door with 32 C threads over 70 engines, each
with its own timeout, against a refused port (`NT=32 ITERS=20000`). With
the settings table behind one lock (commit 7e4d4f1), 1 of 83 runs ended
in SIGSEGV. The pre-fix tree crashed 2 of 54 runs. ThreadSanitizer is
clean on the Rust churn probe after the fix, so the table races are gone
and this crash has another cause.

The kernel line for the crash:

    churn[950764]: segfault at 726264001 ip 00007263231043a2 sp 00007262319d4df0 error 6 in libthinkthen.so[3033a2,726322efa000+245000]

The address maps to `std::sys::thread::unix::Thread::new::thread_start`
plus 0x12. The instruction there is `lock incq (%r12)`, the first write
through the new thread's start record. An earlier pre-fix crash
(09:59:34, the round-five reviewer library) sits at the same place:
`thread_start` plus 0x12. The stack pointer of every churn crash ends in
`df0`, the first frame on a fresh thread stack.

The only Rust threads on this path come from ureq's default resolver.
It spawns one detached thread a request whenever a timeout is set, for
an IP literal too (`ureq-3.4.2/src/unversioned/resolver.rs:147`).

The start record's first word read back as `0x726264001`. That value has
the shape of a glibc tcache free-list word: a heap address shifted right
by 12 bits. So something freed the start record's memory before the new
thread read it. The standard library frees that record only when
`pthread_create` fails. A stray free of memory that was later handed to
the start record fits the evidence.

## What is open

The source of the stray free. Next probes, in order:

1. Build the C library with `-Zsanitizer=address` (nightly,
   `-Zbuild-std`), link `churn.c` with clang's matching runtime, and run
   it in batches until a report names the second free.
2. Give the stand-in's agent a resolver that resolves an IP literal on
   the calling thread. This removes one detached thread a request. It is
   worth doing for cost alone, but it is not a fix until probe 1 names
   the free.

The probe sources are `churn.c` and `churnbt.c` (the fault handler
variant) from the round-five review notes.
