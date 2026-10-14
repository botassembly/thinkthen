# 0587: Test command-bearing Python and npm packages

Ticket: 0587
Date: 2026-10-14

Review: accept. A fresh read-only reviewer checked archive hashes and contents, lineage, recorded command and MCP responses, installed-library replay checks and size accounting. The reviewer accepted the explicitly directional result and its release, platform, runtime-floor, registry and mixed-revision limits. No findings required changes.

Both local prototypes work on Linux x86-64. Offline uvx and npx print the real command's version and receive an MCP initialize response. The installed Python and Node libraries still answer true from their own existing recordings. Adding a separate native command costs about 5.7 MB compressed and 15.1 MB installed per Linux package in these historical builds. This answers packaging feasibility; it does not qualify the current release.

## Inputs and identity

The assignment checkout is 0e46dc2daf53bba7ad40165e67068948607f7bec. No product code changed. Its native artifacts were absent, so this bounded experiment reused existing identified artifacts instead of competing with 0.2 compilation. Its ticket text had coordinator edits during inspection; source-status.txt records those edits. This experiment wrote only its owned folder.

The command and Python wheel came from the retained build manifest at revision 11eca8b69a86e22de889170bc8789410adf30689. The command SHA-256 is 6db2628480a8113b132b2209971826bb51c7cd16013ed7c4417ac2028ce3acea. The baseline wheel SHA-256 is 98902d1065233b8858dda91b7fc8c31dcfa3ded1feff0b6bec176f1fe9f7ed15. Both hashes were checked against that manifest before packaging. The prototype wheel SHA-256 is 30abd5f89b792bf10035eb55d7cb000132862b6f2426c8bd90bb36aaa795c4df.

The Node baseline is the retained installed-check archive from experiment 302, revision 4c0ef210fd9193c40c6277efc59c170b385f3c43, package version 0.0.1. Its SHA-256 is 50fb57bcb2d92aee505c60ca6630900c11626dfe91a17509676b23aac9425499, matching its earlier report. The npm prototype SHA-256 is cb358dfe3793b2234236398085e54763daa855a0f6a710315bdaf202657eea70. The Node library and bundled command therefore have different historical source revisions. They operate independently; this demonstrates packaging and preservation of that older library, not matching-release parity. No current baseline package-size claim follows from these numbers.

Inputs are the project's MIT code, local build artifacts, and its existing synthetic report recordings. Fixtures were copied from Git at each library's own revision. No model service was called.

## Prototype and checks

The wheel adds thinkthen/bin/thinkthen, a tiny Python exec launcher and a console_scripts entry, then rebuilds RECORD with valid hashes and preserves the executable bit. The npm archive adds the actual native command and a package.json bin entry. The Node facade and addon remain unchanged. No fake command, Rust build, publication or installed-service change occurred.

All execution ran on Beelink in disposable `docker run --rm --label pm.repo=thinkthen --label pm.ticket=0587 --cpus 1 --memory 1g` Ubuntu 24.04 containers. Cached image ID: 69f919497cad. Free Python and certificates were installed only inside each container layer; host uv/uvx and Node were mounted read-only. Consumer caches were new directories under container /tmp. No host caches changed. Tool versions: Python 3.12.3, Node 22.22.3, npm 10.9.8, uv 0.8.0, glibc 2.39. Network was available for apt; uvx/npx and installs used offline flags. This is not a kernel-enforced no-network proof. No backend credentials were passed to the containers.

`probes.json` records exact argument arrays and raw outputs. Four command runs exit 0: uvx version, Python recording smoke, npx version and Node recording smoke. Each version is `thinkthen 0.2.0`. Both installed-library results equal true. Both stdio initialize probes receive JSON-RPC id 1 with serverInfo.name thinkthen, version 0.2.0 and protocolVersion 2025-11-25. The probes send client version 2024-11-05 and check response identity; they do not certify protocol negotiation semantics or a later tools/call. The server is deliberately terminated after its response. The first pipe-and-EOF probe produced no initialize output; holding stdin open resolves that test-harness issue.

The central commands are:

```
uvx --offline --python /usr/bin/python3 --from /exp/prototype/thinkthen-0.2.0-cp310-abi3-manylinux_2_34_x86_64.whl thinkthen --version
uvx --offline --python /usr/bin/python3 --from /exp/prototype/thinkthen-0.2.0-cp310-abi3-manylinux_2_34_x86_64.whl thinkthen mcp
npx --offline --yes --package /exp/prototype/thinkthen-0.0.1.tgz thinkthen --version
npx --offline --yes --package /exp/prototype/thinkthen-0.0.1.tgz thinkthen mcp
```

## Size results

Bytes are measured, not estimates. Installed logical size includes the package and its metadata; Python's generated command script appears separately. Disk allocation is available in installed-sizes.json and varies by filesystem.

| Package | Baseline compressed | Prototype compressed | Baseline installed logical | Prototype installed logical |
| --- | ---: | ---: | ---: | ---: |
| Python | 5,500,188 | 11,154,037 | 14,336,106 | 29,455,423 |
| npm | 2,493,330 | 8,167,364 | 6,326,183 | 21,445,028 |

Python also adds a 312-byte generated console launcher outside site-packages. Compressed growth is 5,653,849 Python bytes and 5,674,034 npm bytes. Installed package growth is 15,119,317 Python bytes plus that launcher, and 15,118,845 npm bytes. The native CLI itself is 15,118,752 bytes. These packages duplicate engine code because they contain an extension/addon and a separate command. No effort to share one engine image was necessary to answer this risk.

## Platform approach and remaining work

Proceed with command-bearing packages if this size increase is acceptable. Keep the command version and library source revision identical in actual release assembly. Python should carry one executable for each wheel's platform and architecture, preserving its wheel ABI and operating-system floor. Its console launcher must select thinkthen.exe on Windows and preserve process exit status and stdio. npm should select the matching bundled command for each supported platform/architecture; a small JavaScript launcher can handle platform selection and Windows execution. Preserve the existing addon loader and package target matrix. Consumer installation should fetch no extra executable and compile no Rust.

Build the command against the same or lower runtime floor as its containing package. These historical Python bytes require glibc 2.34, so the cached glibc 2.28 manylinux image cannot run them. Ubuntu success does not prove the planned glibc 2.28 baseline. Mac x86-64/arm64, Linux arm64, Windows x86-64, signal propagation, actual publishing and final all-platform npm sizes remain for the publishing milestone. Reuse its real release assets rather than rebuild just for this experiment. No registry name-resolution claim follows from local `--from`/`--package` invocation; `uvx thinkthen` and `npx thinkthen` by public name require an authorized published package.

## Evidence and stop

plan.md records the risk and stop rule. measure.py and measure-npm.py build the local prototypes; the latter gzip header timestamp changes upon rerun. run-python.sh, run-all.sh, probes.py, smoke.py, smoke-node.js, run-sizes.sh and installed-sizes.py hold the executed procedure. python-container.log, all-container.log, sizes-container.log, probes.json, sizes.json, npm-sizes.json and installed-sizes.json hold the raw outcomes and measurements. The prototype and baseline archives remain for the open ticket. All disposable containers exited and removed themselves; no processes or external changes remain. No evidence was deleted.

The named feasibility and size risk is answered for historical Linux prototypes. Stop here; rebuilding current native artifacts would add release competition without changing this directional packaging finding.

## Evidence retention

The experiment folder is `experiments/483-thinkthen-packages-0587`. The coordinator reviewed its raw results before recording this finding. After acceptance and landing, owned generated inputs, databases, packages and scratch can be removed. The observed results and stated limits remain in this record.
