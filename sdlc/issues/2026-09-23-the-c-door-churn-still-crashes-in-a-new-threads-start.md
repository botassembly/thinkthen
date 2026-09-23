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

## Wave 7: a second signature, and the lookup thread removed

Two crash signatures now exist, both in the churn probe
(`NT=32 ITERS=20000`, 70 engines, a refused port).

The detach signature. The wave-7 prober's two cores at 610a133 (2 of
104 runs) fault at `___pthread_detach+32`, called from ureq 3.4.2's
`DefaultResolver::resolve` on a `ttb-worker` thread. That resolver
starts a lookup thread for every request with a timeout, an IP literal
included, and drops its handle (`resolver.rs:147`). In both cores the
faulting read is `cancelhandling` at offset 0x308 of the target
thread's record. The instruction before it, a compare-and-swap on
`joinid` at offset 0x620, completed, and both offsets sit on the same
4 KiB page. The core shows the whole 2 MiB stack region that held the
record unmapped. So the page was unmapped between those two
instructions.

The start-record signature. The baseline here crashed once, at
`thread_start+18` (`lock incq (%r12)`), the same place as the wave-5
crashes above. The start record at `rdi` holds `0x72fb44002`, its own
address shifted right by 12, and then a random word. Those two words are
glibc's free-list link and free-list key, so the start record sat freed
in the thread cache when the new thread read it.

### What was run

- Baseline at 14f12f5, batches of 8: 1 crash in 64 runs (the
  start-record signature). With the prober's 2 in 104 at 610a133, that
  is 3 in 168 across the two signatures.
- The fix (w7/standin 5f36536): every pool resolves through the
  stand-in's `Lookup`, which parses a numeric address on the calling
  thread and joins its lookup thread for a name. `strace -f -c` over 800
  churn requests counted 2,407 thread creations before and 1,606 after.
  The per-request lookup thread is gone, and the worker and feeder that
  each door call starts, and joins, remain.
  `standin/tests/thread_starts.rs` holds this in the gate: 1,000 requests
  started 1,003 threads before and at most 3 after.
- The fix, churn: 0 crashes in 20 runs (16 in batches of 8, 4 in a batch
  of 4). The run stopped there because the box was saturated. At the
  baseline rate, 20 clean runs happen by chance about 70 percent of the
  time, so they prove nothing yet.

### What is proven

- Something unmapped the stack holding the detached thread's record
  while `pthread_detach` was inside it. glibc unmaps a thread stack only
  after the thread's record is freed. A joinable thread's record is
  freed only after a detach or a join, and the compare-and-swap shows
  the thread was still joinable when this detach began. So the detach
  in that same call let the exiting thread free its own record, and the
  stack was unmapped before the detach finished reading it. That is a
  race inside glibc's `pthread_detach` against an exiting thread.
- Our thread-exit cleanup did not free that memory. The unmapped region
  is a whole thread stack, and only glibc unmaps thread stacks.
- The detach signature needs a thread detached while it exits. After the
  fix no request detaches a thread. A name lookup that outlives its
  timeout is still detached, but that thread is blocked in the lookup.

### What is not proven

- That the fix ends the crashes. That needs at least 300 churn runs at
  the fixed library with no crash, on an idle box, with Ian's go-ahead.
- What freed the start record in the start-record signature, and
  whether the lookup thread is necessary for it. A reused stack after
  the detach race is one path that fits. The 300-run proof answers the
  second question: a start-record crash after the fix names another
  cause.

### The manual stress recipe

The probe is `R4-1-churn.c` from the wave-7 prober's notes. Build it
with `cc -O1 -pthread -I contract/include R4-1-churn.c -L<libdir>
-lthinkthen -o churn7`. Run it as `NT=32 ITERS=20000
LD_LIBRARY_PATH=<libdir> ./churn7`, four at a time, with `ulimit -c
unlimited`. Exit 139 is a crash. On this box apport writes the core
under `/var/lib/apport/coredump`. Read it with `gdb -batch -ex "thread
apply 1 bt" -ex 'p $_siginfo._sifields._sigfault.si_addr' churn7
<core>`. A run takes about three minutes at load 300. The probe runs by
hand only: it pegs the machine for the length of a batch.

### What is open

1. The 300-run proof at the fixed library. The lever is Ian's go-ahead
   on an idle box.
2. If a crash follows the fix: its backtrace, and the ASan build at the
   full `ITERS=20000` from the earlier open list.
3. Main has the same per-request lookup thread
   (`crates/thinkthen/src/engine/http.rs:78-90` with ureq's default
   resolver). The steering session files that on main.
4. The two design risks above are still untested.
