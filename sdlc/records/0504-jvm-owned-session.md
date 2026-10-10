# 0504: Retire the JVM compatibility API

The work starts at `1848676089546d8d4a9de3880835abbf79eec128`. The code is committed as `a8666053a` and `1165fccc2` on `ticket/0504-close-jvm-migration`. The pending SQLite branch remains at `099f0c54a`.

Java, Kotlin and Scala retain one typed family of ten functions, generated inputs and results, typed failures, cancellation and owned cleanup. The preview `Door`, handwritten `Complete` readers and layouts, old Kotlin and Scala facades, map call overloads and compatibility builders and consumers are removed after their replacements pass installed checks. The shared JSON parser remains. Its distinct parser regressions now run through the stable installed Java consumer. The README gives the old-to-new mapping. Rust product code, the C product and the frozen 0.1 C exports are unchanged.

Each Kotlin and Scala classifier now includes the shared compiled Java classes and the product inventory. Each language needs one SDK dependency, with the matching native classifier supplied by the POM. The installed consumers compile and run Kotlin and Scala without the base Java SDK JAR. The recursive Scala conversion still preserves native Scala collections and numeric originals before passing generated input carriers. JDK version checks use the shared isolated child environment.

## Source removed

Counts use nonblank source lines relative to the starting revision. Generated source and generator templates are unchanged; their added and removed counts are zero. The table includes handwritten product code, fixtures and package scripts.

| Source | Removed | Added | Net removed |
| --- | ---: | ---: | ---: |
| Java | 1,818 | 10 | 1,808 |
| Kotlin | 195 | 1 | 194 |
| Scala | 204 | 8 | 196 |
| Python | 915 | 28 | 887 |
| Shell | 103 | 0 | 103 |
| Total | 3,235 | 47 | 3,188 |

The Java ceiling drops from 5,809 to 4,001, Kotlin from 1,852 to 1,658, Scala from 1,868 to 1,672 and Python from 1,544 to 664. The prior Python tree actually measured 1,551 lines, seven above its declared ceiling. Every final ceiling equals its measured source total.

## Evidence

The baseline stable installed session and usage checks pass before retirement. The baseline shared subset also passes for all three languages. The final package is built from `1165fccc2` as version 0.2.0 for Linux x86-64 with verified OpenJDK 22.0.2, Kotlin 2.4.20 and Scala 3.9.0. The local JDK archive matches the official SHA-256 from the [OpenJDK archive](https://jdk.java.net/archive/). The reused native library and conformance backend have unchanged source inputs between the preserved SQLite revision and this branch's starting revision.

Final checks pass:

- `build.sh` assembles the stable JARs, bundled Linux native classifier and versioned local Maven artifacts.
- `package_check.py --session` checks exact compiled members, rejects retired classes and planted stale and private members, and verifies each embedded inventory. The retained session ABI check compiles and links 19 native descriptors; its relevant descriptor sources are unchanged by the final classifier packaging change.
- `session_installed.py --maven` runs the installed Java ten functions, parser regressions, typed failures, owned results, zero-send refusals and exact surface attribution. Java futures, Kotlin coroutines and Scala futures cancel and clean up while the provider remains held and unrelated calls progress. Each facade runs with its own SDK JAR and native classifier.
- `usage_installed.py` passes written, failed and disabled persistence states for each language, with retained observations and exact request counts.
- `public_types.py` passes 44 selected shared cases per installed consumer, 132 cells in total. The selectors take the public IDs from `conformance/routine-ids.txt`, the ten `files-*` cases and `images-decide`, `images-choose` and `images-score`. The private defect injection is excluded. Kotlin and Scala run without the base Java SDK JAR.
- JVM generation, all four source ratchets, the JVM child-environment guard, offline policy, binding registry lint and `git diff --check` pass. Policy reports existing TypeScript and Zig source-size warnings.

The coordinator reuses the routine Rust evidence at `c46692e1110b6f7fb423746e197ad64ae8cd924e`: 1,992 distinct workspace cases, documentation tests, the external consumer and supporting script checks pass. Relevant Rust and native-backend inputs remain unchanged. Intact root lint passes at `6caecd028`, including workspace Clippy, documentation and inventory. Fresh code review accepted `c1adfd30d41ec5c2add2f3ba2b4980b6efdda328`. Integration keeps the reviewed Python ceiling of 664 and the shared isolated toolchain helper; it changes no reviewed product code.

## Proof limits

These checks establish local Linux x86-64 installed behavior. They do not establish Windows or Apple execution, registry resolution, every native classifier, full installed parity, release qualification or publication. Those belong to 0530 and the candidate platform checks. No paid provider call, load run or release campaign runs here.

## What the build taught us

A facade classifier must carry both its shared compiled classes and the inventory its native loader reads. Compiling against the base JAR had hidden that second SDK dependency. The existing installed consumer now exposes the defect by omitting that JAR. Retiring compatibility code also exposes the useful parser checks; preserving those checks at the installed boundary keeps their behavior without the old harness.
