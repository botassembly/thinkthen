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

Each door call starts three Rust threads, and the crashing thread may be
any of them. The batch path starts a scoped worker (`ttb-worker`) and a
scoped feeder (`ttb-feed`). ureq's default resolver starts one detached
thread a request whenever a timeout is set, for an IP literal too
(`ureq-3.4.2/src/unversioned/resolver.rs:147`). The independent verifier
counted three `pthread_create` calls a door call with an `LD_PRELOAD`
shim.

The start record's first word read back as `0x726264001`. That value has
the shape of a glibc tcache free-list word: a heap address shifted right
by 12 bits. So something freed the start record's memory before the new
thread read it. The standard library frees that record only when
`pthread_create` fails. A stray free of memory that was later handed to
the start record fits the evidence.

## What the verifier ran

The independent verifier did not reproduce the crash in 32 runs of the
same shape (`NT=32 ITERS=20000`, batches of 8, load average 300 to 360):

- The plain release library: 0 of 16 on the fixed tree, 0 of 16 on the
  pre-fix tree.
- AddressSanitizer (nightly, `-Zbuild-std`, the runtime linked into the
  probe, checked first against a double `thinkthen_free_string`): 8 of 8
  clean at `ITERS=5000`, no report.
- ThreadSanitizer on the C churn (`NT=32 ITERS=300`): no report.
- The `pthread_create` shim, which compares each thread's start record at
  create and at the child's start: 16 of 16 clean, about 1.9 million
  creates a run, no failed create, no changed start record.
- The fork probe (16 threads, 1,500 forks): no hang.

## Two design risks, both untested

- A lock holder killed inside the settings-table lock by an asynchronous
  `pthread_cancel`. The lock word keeps the dead thread's pid, and the
  pid matches the live process, so every later caller spins. Untested.
- A fork into a new PID namespace. The child can see the same pid its
  parent held, so it cannot tell the parent's stale lock word from its
  own and may spin. Untested.

## What is open

The source of the stray free, and a way to reproduce the crash. Next
probes, in order:

1. Run the AddressSanitizer build in larger batches at the full
   `ITERS=20000` until a report names the second free.
2. Give the stand-in's agent a resolver that resolves an IP literal on
   the calling thread. This removes one detached thread a request. It is
   worth doing for cost alone, but it is not a fix until probe 1 names
   the free.
3. Probe the two design risks above.

The probe sources are `churn.c` and `churnbt.c` (the fault handler
variant) from the round-five review notes.
