# 0438: Ruby platform fallback

A matching-version plain Ruby gem prints supported-runtime requirements at install and raises LoadError at load. The four native gems retain their platforms, Ruby bounds and Darwin floor. Release packing includes the fallback and checksum beside the native gems.

Fresh code review accepted. Local gem consumers cover historical-placeholder avoidance, native preference and unsupported combinations; actual native import passed on Linux. Other-platform selection uses simulated metadata. Existing package inventory, release bundling and missing-artifact checks passed. Full tests and lint run on the landing commit. Nothing was published.

## What the build taught us

A current fallback must be selectable on unsupported runtimes. Its diagnostic must reject use explicitly while compatible hosts continue to select their native gem.
