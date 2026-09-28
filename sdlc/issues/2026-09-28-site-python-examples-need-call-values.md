# Site Python examples need the approved Call value

Status: open, required before public 0.1. Owner: the marketing lead under sdlc/planning/ownership.md. Ticket0214's approved result contract is implemented in candidate79723fd2, integratedc9579b02, and awaits fresh code review. This issue records the required consumer migration before that contract ships; it does not claim the candidate is landed.

Every successful Python asking method and module function returns Call with value, facts and details. question() and usage() retain their existing local contracts. The library's examples have migrated, but these thirteen site files still consume the old bare values on main3b96e720:

- site/examples/functions/{choose,filter,find,rank,recognize,relate,score,tag}/python.py
- site/examples/functions/{decide,question-file,score}/polars.py
- site/examples/install/{python,polars}/first-call.py

For example, filter compares the Call to a list, the Polars installation example calls drop() on the Call, and the Python installation example expects the Call itself to be None. A bare truthiness assertion may pass without inspecting the answer at all. Preserve the existing named-answer convention and use the inner value for each answer assertion or frame operation. Keep null, false, failure and empty values distinct. Facts and details can remain unshown where they do not serve the example.

Search direct consumers and generated copies before declaring completion. Reuse the current library examples and stub as the source of the approved contract, and run the closest existing offline sample checks against the final reviewed Python package. Preserve batch=1 where old recorded request bodies require it; do not regenerate provider or benchmark evidence merely to change result access. Record the package/source revision tested and prove the actual scalar, list, find/rank, recognition/relation and Polars values rather than Call truthiness.

The C JSON wrapper has its own seven-consumer issue, 2026-09-28-site-c-examples-need-json-value-wrapper.md. Both migrations can share one marketing batch, but each retains its distinct result and failure rules. The broader site-samples issue remains an umbrella. No site file was edited and no external message was sent to file this issue.
