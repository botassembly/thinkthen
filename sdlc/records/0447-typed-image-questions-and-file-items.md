# 0447 slice A: Native and command-line image inputs

Decide, choose and score accept ordered JPEG/PNG images, scalar/batch inputs and whole-file located items through the shared reader and scheduler. Other functions and unproved routes refuse images before sending. Existing text methods remain compatible.

Fresh code review found image/text cache collisions, blocked attachment cancellation and missing stored-image validation. The fixes pass distinct-key replay, first-signal termination and corruption regressions; focused High correction review accepted. Native image13, estimate1, reader7, CLI image7, interruption2 and existing store8 checks passed. Full tests and lint run on the landing commit. Local profiles and C/host/SQL/frame adoption remain open. No paid calls ran.

## What the build taught us

Image transport needs a distinct identity, the existing cancellable reader and validation of referenced stored data. These are user behavior, not receipt verification. Preserve immutable originals and absent image line positions.
