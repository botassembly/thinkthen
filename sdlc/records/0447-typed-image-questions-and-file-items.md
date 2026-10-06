# 0447 slice A: Native and command-line image inputs

Decide, choose and score accept ordered JPEG/PNG images, scalar/batch inputs and whole-file located items through the shared reader and scheduler. Other functions and unproved routes refuse images before sending. Existing text methods remain compatible.

Fresh code review found image/text cache collisions, blocked attachment cancellation and missing stored-image validation. The fixes pass distinct-key replay, first-signal termination and corruption regressions; focused High correction review accepted. Native image13, estimate1, reader7, CLI image7, interruption2 and existing store8 checks passed. The landing gate found stale standalone consumer locks; their new registry packages and checksums now match the reviewed root lock, without replacing existing registry versions. Full tests and lint run on the corrected landing commit. Local profiles and C/host/SQL/frame adoption remain open. No paid calls ran.

Local slice B is built from 5974abe80 with closed declarations, exact aliases and the measured SDK envelope. Pure decoded-input checks, policy, format and ratchet pass. An isolated propagation candidate passes focused Clippy and 21 SDK/CLI image cases; its original Imajev partial-usage regression remains explicitly pending native integration. Shared propagation is supplied separately; whole High review and full landing checks remain with root. No model calls or dependencies were added.

## What the build taught us

Image transport needs a distinct identity, the existing cancellable reader and validation of referenced stored data. These are user behavior, not receipt verification. Preserve immutable originals and absent image line positions.
