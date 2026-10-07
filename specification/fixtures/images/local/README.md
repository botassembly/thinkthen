# Measured local image declarations

Use the existing profile file or inline JSON door. Each example declares the setup in [backends.md](../../../backends.md#local-image-declarations), including exact runtime/projector/context requirements. An operator may replace `model_alias` only to match their explicitly chosen server alias; IDs are exact and closed. All three use the measured named `llamacpp` System One wire route. The Imajev ID describes its MLX runtime and does not enable the separate named `mlx` text route.

- `clef-profile.json`: measured Clef alias `clef-local-0036`.
- `clef-flash-profile.json`: measured Flash alias `flash-local-0036`.
- `imajev-profile.json`: measured author MLX alias `imajev-2b`.

`setups.json` transcribes the measured setup identities from completed thinkthen-exp 0036 at `b4b1d00b7ec28e57acc878c2bd3d23ae6402ed1a`, `inputs/runtime-{clef,flash,imajev}.json` and `inputs/source-pins.json`. No host paths, credentials or headers are carried here. Profiles do not inspect or attest these identities at runtime.

The four body-only fixtures retain request and response objects from native 0036 recordings, without headers or transport location. `0036-clef-receipt.json` and `0036-clef-grouped.json` contain CORD receipt originals. CORD v2 is NAVER/Clova's CC BY 4.0 dataset at revision `7f0115a4b758a71d6473b8d085751692da2fef98`: [release](https://huggingface.co/datasets/naver-clova-ix/cord-v2/tree/7f0115a4b758a71d6473b8d085751692da2fef98), [license](https://creativecommons.org/licenses/by/4.0/). The pair and partial fixtures contain prepared VisA images from Amazon Science, revision `2a692ab575001cbde74d402d897a7286086c6199`, [CC BY 4.0 license](https://github.com/amazon-science/spot-diff/blob/2a692ab575001cbde74d402d897a7286086c6199/LICENSE-DATASET). Preparation in 0036 explicitly resized those experiment inputs; ThinkThen sends the saved prepared bytes unchanged. These fixtures make no accuracy claim.

`0036-imajev-partial.json` retains the original author response with input_tokens 887 and absent output_tokens, including `unknown_probability` and `abstained`. It does not copy the later wrapper's derived zero. Native integration must retain partial observation without inventing a missing count.

The synthetic PNG edge files are deterministic 1024×1 and 1025×1 red pixels. The byte files add a valid ancillary `tEXt` chunk to hit exactly 1,048,576 and 1,048,577 bytes. Together with the independent shared red/blue PNG and JPEG fixtures they test decoded originals at the optional SDK envelope boundaries. They are not vendor limits or local image-token estimates.

These examples become publicly executable only after shared propagation and partial usage integration, fresh High review and landing checks. No live compatibility session ran in this builder.
