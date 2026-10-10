# 0527: Rust Polars named collection calls

The existing `filter_series`, `rank_series` and `find_series` calls now use a private native Request feed. The adapter converts text cells into native records and omits null cells. Shared Request admission owns question compatibility and execution; the adapter keeps no independent input grammar, cache key or result reader.

The public signatures and native output types stay intact. Filter returns a named String Series in input order. Rank returns the original nullable column positions, texts and probabilities, with duplicates preserved. Find returns every real candidate and the optional synthetic none, its full probability vector and the selected flag. The lazy rank/find wrappers continue to collect the complete logical set once. Call facts and terminal errors come from native execution.

The existing outside-in collection cases protect nullable duplicate positions, stable ordering, whole-set find, and native recognition/relation behavior. The existing zero-send edge-case table also checks incompatible filter questions, non-text rank/find columns and cancelled filter/find calls. No large-input or full parity run belongs to this slice.

Focused public Polars collection tests pass: four cases in 0.25 seconds against isolated local fake backends. The test executable calls the crate's public methods directly. Installed package parity remains owed to the owning ticket; this focused run does not substitute for it. Offline Polars library checking, strict library/test Clippy and the repository policy check pass. The handwritten production diff adds 158 lines and removes 78, including the 65-line private Request bridge; no generator or template changes are involved. The measured Rust ceiling rises from 182830 to 182966, including the existing edge-case table's additions and assertions that duplicate rank occurrences retain their text while provider state stays identical across named calls.

## What the build taught us

The typed Request result already carries compact native occurrence ordinals and the complete find candidate list. Mapping those ordinals through actual present-cell positions removes the old private indexed-original carrier without serializing results or changing the caller API. Rank now shares identical pending judgments through the native scheduler: the nullable duplicate fixture asks twice for three present occurrences and still returns all three rows. The old carrier prevented that sharing. Structured relation entities cannot be represented by a text-only adapter and remain on their existing path.

## Remaining outcomes

The other named Series/frame methods, typed complete and pull-based column methods, native columns beyond text, and their installed shared-case consumers still need adoption. The current consumer's R-private projector and Python input decoder must move to a shared native consumer boundary. Ordinary Rust callers, rustdoc and package call mappings need the same owning acceptance checks. Compatibility APIs remain reachable until installed replacement parity justifies retirement. This slice does not certify full Rust or Rust Polars parity.
