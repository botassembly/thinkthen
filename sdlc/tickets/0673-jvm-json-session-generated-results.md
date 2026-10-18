# 0673: Move JVM onto run-call and generated results

Status: OPEN.

Milestone: 0.2.1

Phase: 3

Depends on: 0688

## Outcome

- Java, Kotlin and Scala call the single run-call entry point and read the final answer, probability, pick and resolution that Rust puts in the packet.
- Results use the one result representation 0638 chose, with generated types. Input uses the path 0635 chose.
- The JVM session loop in `OwnedSession.java` and its private closed, reader and producer error text are deleted. Calls run on one worker thread attached to the JVM and cancel through the handle.
- `NativeResults.java` and its generator template are deleted, along with every layout reader and every probability or answer computation in JVM hand code.
- `Json.java` is replaced by generated serialization if 0635 chose JSON input, or deleted if it chose typed input.
- Old public names are removed, with no compatibility shims.
- Its files leave the 0637 answer-rule check's list of exceptions.

### Proof

- The installed-package checks (`session_installed.py`, `usage_installed.py`, `package_check.py`) pass on the M5 through pm job against the packed jar for Java, Kotlin and Scala consumers.
- Each consumer runs all ten named calls against a recorded backend, and its packet dump matches the 0642 oracle.
- The 0637 answer-rule check passes over the JVM sources, and a grep finds no native struct offsets.

## Evidence

The JVM binding is the only session binding that decodes errors, plans and packets by reading native C memory layouts directly (`libraries/jvm/session/thinkthen/Engine.java:36,58`, `OwnedSession.java:54`, through the generated `NativeResults.java`). C# and the binding guide use the owned JSON session and generated typed results (`libraries/BINDING-AUTHOR.md:9,13`). The JVM therefore needs its own generator path, and every change to a native layout breaks it. Commits that chase layouts include cef6f6f6c, 7fa174151 and 1b88c2d2e. The JVM side also patched the probability type four times on 10-14 and 10-15 (55686dac0, d5c7838bc, d1369ebf1, 211b2159b) because it shapes answers and probabilities itself. `Json.java` is a hand-written JSON encoder.
