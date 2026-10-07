# 0393: Stage the verified npm archive

Slice A pins npm 11.15.0 and stages the same archive that the pack job dry-runs and uploads. Provenance, public access, release guards and GitHub approval stay in place. The workflow parser accepts the staged command and still refuses bare archive paths. One fresh High review accepted the complete change; 113 workflow self-tests and the stable-checkout workflow check passed. Full landing checks are running on the combined MCP/npm commit.

## What the build taught us

Staging uploads the package, while npm availability waits on Ian's subsequent two-factor approval. The release instructions now explain that distinction and the public-install rerun after approval.

## Remaining

The final rehearsal must exercise the staged dry run. Direct publishing must be disabled before the actual authorized release stages the package; Ian then approves it on npm. No registry setting, staging upload or publication ran during implementation.
