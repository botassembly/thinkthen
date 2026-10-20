# 0738: Layered check image evidence

M5 job 1792510701041-20887 passed on source 638e052bf. A scratch Zig 0.15.1 preparation reused all 42 earlier build steps. Restoring Zig 0.15.2 and editing the mounted resolver left the image tag unchanged; lint passed with three resolver execution markers. The resolver fixtures passed with three native macOS cases skipped in the Linux container, and hosted workflow fixtures passed.

The resulting image was thinkthen-check:arm64-663d85f170a9e4515cec, sha256:755d2886972b7c125457d1ea224f5bec6cc5f6b58d6850297a70f9e3bc09252a. One tagged arm64 ThinkThen image remained.

The real proof exposed two defects. The Dockerfile omitted Ninja's owning fragment, causing missing-tool refusal before pruning. Selected-fragment coverage now checks every required owner. Removing an older image also deleted classic-builder parent metadata and destroyed cache reuse; removal now preserves those parents with --no-prune. Failed preparation and invocation receipts remain under jobs 1792509020190-33852, 1792509883250-99019 and 1792509860485-91248.

Earlier authorized cleanup job 1792508132971-9695 removed six unused arm64 tags while protecting running images. Host available space increased by 27,722,117,120 bytes, or 25.82 GiB. Docker's rounded aggregate image usage fell from 111.8 GB to 80.24 GB; the host and Docker totals measure different storage layers and were collected while other containers were running.
