# 0268 independent gitless checkpoint code review

A fresh independent High reviewer accepted code candidate `98672eab` on 2026-09-29. The review covers the generic legacy/gitless packaging correction and controlled archived Go/C++ inputs. It does not complete ticket0268 or qualify a container, installed consumer, workflow runner or release.

## Findings and corrections

The first review of `5493e6d0` found that comparing only archived members allowed extra source/configuration inputs to affect the build. It also found the Docker stand-in performing the source-identity assertion that the fixture claimed to prove. The coordinator then found an unsupported-host regression in the synthetic C fixture: routine macOS lint would require a real dylib path and tool operation.

Correction `ef091fcb` verifies the complete extracted file, directory and link inventory and rejects extra paths before the C build. Fixture native outputs live outside the source tree, so there is no broad generated-output exemption. The Docker stand-in captures transport data only; the fixture independently checks the selected SHA, archive path and embedded commit. Extra Cargo configuration, missing/type-changed inputs and changed link targets have focused refusal proof. The fixture skips unsupported hosts successfully; production checks remain strict.

## Verdict: CODE ACCEPT

The same High reviewer inspected the immutable correction and record at `98672eab` and found both requested corrections closed. The review used source inspection and a clean diff, without running a write-effect fixture or any build. After merging, the coordinator independently ran `workflows --self-test`:27/27 cases passed. Plain workflow policy, shell syntax, Python parsing and diff checks passed. These focused checks use synthetic build side effects; no SQL, DataFrame, native, container, backend or Actions run occurred.

The actual Go/C++ workflow package list, expected-family gates, runner tool setup, resolved-SHA wiring, manylinux C/ABI proof, installed consumers and Actions result remain explicit ticket criteria. Ian's SQL/DataFrame hold still prevents the aggregate execution. This acceptance changes no original issue count or support claim. The next independent workflow wiring must reuse the accepted source-provenance boundary and undergo review before landing.
