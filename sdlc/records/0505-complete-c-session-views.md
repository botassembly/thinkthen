# Complete C session views

C callers borrow the complete generated packet graph from their existing independent session result. All nested strings, arrays, maps and union arms share that result’s lifetime after engine and session destruction. No second owner or release function exists. Partial token usage preserves reported zero and missing counts separately; optional result members also retain explicit null. Unknown members retain their raw JSON at every object depth, while arbitrary JSON retains member order and exact native numeric tokens. Frozen compatibility declarations and exports keep their contracts.

The build exposed a generator naming collision between a named schema type and an inline field. Inline field declarations now use a distinct namespace, and conflicting names refuse generation. The existing generated-reader null fixture also clarified that optional result conversion must remain tolerant rather than perform semantic admission.

This slice adds 786 handwritten nonblank source lines and removes 6, including generator and contract-test changes. The generated Rust graph adds 12704 nonblank lines. Core Rust source does not change. C source ceilings require explicit reviewer acceptance before landing.

Review found that the temporary JSON projection owned each nested subtree again. Conversion now borrows raw slices from the serialized packet and keeps only published strings, extension JSON and numeric tokens in result-owned storage. The small nested fixture drops its input before reading the final graph, preserving the existing lifetime contract without a memory benchmark.
