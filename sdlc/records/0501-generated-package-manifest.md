# 0501: Generate one package inventory

The npm and JVM builders, installed checks, assembly and registry consumers derive shipping files from their package definitions. Native targets come from the existing release matrix. JVM members come from compiler output. No runtime download or consumer-side native build was added.

Fresh review accepted a1dc03edae7ba64cbe1b3d1ff9f6f1f800dd1457 with no blocking findings. Windows production and stable JVM native loading remain with their owning migrations.

## Checks

An offline fresh npm installation passed inventory checks, CommonJS and ESM imports, and the existing public type consumer. Java, Kotlin and Scala compiled against the packaged jars; Java plan and Kotlin carrier execution passed. Exact JVM members, stale/private-file mutations, ABI checks and mutations, actual jar assembly, managed assembly tests, registry tests, policy and syntax checks passed.

Independent consumers rejected missing complete.js and Requests.class even when the package definitions agreed with those omissions. Full JVM qualification stopped at the separate legacy recognize-details schema defect. Full npm conformance stopped because the cached backend lacks case 41. Neither result is claimed as full surface qualification. These prerequisites remain with 0513 and the installed family checks.

Ticket-owned evidence was produced in target/0501-inventory/ in the lane. It contains installed packages, compiler output and check logs. Those scratch artifacts may be removed after landing; the maintained source and independent checks remain in the repository.

## What the build taught us

Matching an inventory proves package contents, but it cannot prove that the inventory includes everything a caller needs. Keep the independent installed consumers. Compiler-produced jar members also include directory entries; assembly must compare product files rather than treating directories as missing classes.
