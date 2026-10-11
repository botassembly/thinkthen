# 0.2 adversarial review, 2026-10-11

Ian asked for an adversarial review of the 0.2 build after 0534 to 0542 closed. Three read-only reviewers covered the release test campaign, the language bindings and the Rust core at 8e75481d5. The PM checked the main claims against the code. Tickets 0549 to 0554 carry the work.

## Release test campaign

The rc.11 campaign cannot pass. The time limit killed Ada at 30:00 after 254 of 255 cases. Nine consumers failed setup within a second each because the scratch home drops the toolchain and cache paths. The Beelink has 16 cores, but the runner uses one consumer at a time.

Each consumer took this long in the rc.11 campaign. The serial preamble before them took 13 minutes.

| Consumer | Time | Per cell |
|---|---|---|
| Java, Kotlin, Scala | 21:34 | 1.7 s |
| Ada | 30:00, killed | 7.1 s |
| rust-polars | over 7:40 | |
| Rust | 6:22 | 1.5 s |
| Others | 0:42 to 4:09 each | 0.2 to 1.0 s |

Ada is the outlier because every case writes a generated caller, deletes its objects and statically links the 70 MB native archive (`libraries/ada/checks/full_case.py:53-59`). A warm compile and link costs about 4.7 s, and the program runs in 6 ms. COBOL builds one bridge and runs all 255 cases in 111 s. The JVM driver starts a new JVM for each case in each of three languages. Rust consumers compile with one job. The preamble reruns the routine suite and a Rust public-case pass that the installed `rust` consumer repeats.

Parallel consumers, one Ada build and one JVM per language would leave the longest consumer as the campaign's length. Restoring the nine missing consumers adds their own time back. Ticket 0549 carries this work.

Of the 258 required cases, 96 are named-input cases and 24 are image-admission cases. Most of them test refusals and byte limits inside the shared Rust core, so each binding only passes them through. Keeping them in cli, rust and c, plus a few in each binding, would cut each other consumer by about 40%. That changes the parity contract, so Ian decides it.

## Bindings

The 0.2 slimming worked for C#, PHP, Ruby, JavaScript and COBOL. Python (1,358 host lines) and R (621 host lines plus 1,909 lines of Rust glue) still shape requests in host code. These are the hand-written source sizes, excluding tests, checks and generated files.

| Binding | Host lines | Binding | Host lines |
|---|---|---|---|
| Python | 1,358 | Ada | 442 |
| JVM | 772 | C# | 440 |
| Go | 764 | Objective-C | 382 |
| Dart | 732 | Zig | 344 |
| Swift | 666 | Ruby | 295 |
| R | 621 | JavaScript | 191 |
| PHP | 499 | COBOL | 55 |
| C++ | 461 | | |

Defects, carried by 0550:

- **B1.** Five bindings discard the caller's producer exception and report a generic invalid-input error.
- **B2.** Rust, Ruby, JavaScript, Python and the C session pick error messages by matching on message text. The copies disagree.
- **B3.** Python and Ruby keep a second aggregate-row rule for recognize. JavaScript and PHP use one rule. The rows agree today because Rust sends one aggregate packet per call except for recognize.
- **B4.** C++ throws on a closed push, which is a normal stop signal.
- **B5.** C# immediate failures drop the native retry flag and facts.
- **B6.** PHP's client destructor closes the operations it handed out. PHP also sends records for find and relate where every other binding sends units and entities.
- **B7.** `libraries/typescript/_requests.js` is dead code that still holds admission rules.
- **B8.** The usage counter silently drops counts when its lock is poisoned. `finish_usage` and `finish_usage_status` have identical bodies.

Architecture, carried by 0551 and 0553:

- **B9.** The mapping from function names to calls and input kinds is copied in about 13 places, and PHP's copy disagrees with the rest. JVM and C++ write the request envelope by hand, and the JVM keeps a hand-written `Json.java`.
- **B10.** Python and R still build questions, repeat label checks and do column bookkeeping in host code. R and JavaScript pass errors as packed text.

The C ABI offers only non-blocking reads (`thinkthen_session_try_read`). Every binding therefore sleeps and polls every 1 to 10 ms. A timed or blocking read in Rust would delete those loops. It adds one public C export, so Ian decides it.

## Rust core

The reviewer found no FFI memory-safety defect. The null, length and array checks, handle lifetimes, poisoned session locks and cache-write failures are sound. The framing decoders and the DuckDB routes from the closure review are now shared.

- **A1.** `RequestCall` has ten variants that all carry the same arguments. That shape causes the copied mapping in B9 and a ten-way match in `Request::new`. Carried by 0551.
- **A2.** The C library still runs three request paths besides the session: `thinkthen_call` through `call/legacy.rs` and the engine's per-function methods, and the typed exports in `door.rs`. The `{"usage":true}` branch is frozen 0.1 behavior and stays. "door" and "legacy" remain on 103 non-test lines, including the public `DoorReply`. Carried by 0552.
- **A3.** The public Rust `Engine` methods `decide`, `choose`, `score`, `tag` and `details` go straight to `ask_all` and skip `Request` admission. `public/` has 710 `pub fn`. Command-line reader branches and preparation remain in the library. Carried by 0554.

Product code in `crates/` grew from 41,939 lines on 2026-10-01 to 80,544 at HEAD. Test code grew from 61,925 to 104,023. Binding Rust grew from 26,582 to 38,208, so the core growth is not code moved out of the bindings. The last eight ratchet commits raised the ceiling, and `crates/` plus `conformance/` now sits exactly at it.

## Process lessons

- 0534, 0535, 0537 and 0542 closed with named items still in the code: the door names and the Python and R host logic. Each review accepted a summary of the change, not a search for each named item. A closing reviewer should search the code for every item in the ticket's Changes line.
- A banned word or pattern belongs in `policy.py`, not only in a ticket. 0552 adds "door" to it.
- The ceiling rose eight times in a row during a release meant to remove code. A ratchet raise should name the deletion that will pay it back.
- No summary shows each consumer's duration, so a slow consumer went unnoticed until it hit the time limit. 0549 adds the durations.

## Decisions for Ian

1. Slim the required parity cases for the bindings, as described above.
2. Add a timed blocking read to the C session API before 0.2 publishes.
