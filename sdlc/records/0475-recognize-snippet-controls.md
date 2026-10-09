# Recognition snippet width

This change starts from `7d8f7b4281c89b094e1dbe13bbb93295c9d6b0d3` under the accepted reversible unknown-route and strict encoded-byte assumption. It does not settle the model-window criterion in 0461.

The native declaration adds `RecognizeBuilder::snippet_pieces(u32) -> Self`, `Recognize::with_snippet_pieces(u32) -> Result<Self, Error>` and `RecognitionReading::snippet_pieces() -> u32`. The builder validates platform representation during `build`. Shared `RequestOptions::snippet_pieces` is `Option<u32>`; saved declarations use `recognize.snippet_pieces`; CLI uses `--snippet-pieces`. Zero is admitted. Omission resolves to six and retains existing serialized bytes. Explicit six normalizes to that same reading. Wider values clip through saturating arithmetic and remain subject to existing actual encoded-byte admission. Batch grouping is unchanged.

The pre-change compiled command refused `--snippet-pieces 0` as an unknown argument with exit 2. The kept outside-in tests exercise clipping and generated request identity through counted loopback requests. The existing request schema helper also retained a 20-kind maximum after size-based recognition admission landed; removing that stale schema constraint brings declaration admission into agreement with the native parser.
