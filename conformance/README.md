# Shared conformance cases

The active corpus is `typed/manifest.json`. Every SDK and database driver reads its case filenames and runs both declared backend wire formats through its public API. The old root files used the removed request envelope and have been removed. Their supported scenarios live in the typed corpus and `construction/options.json`. The sole root JSON file, `stress.json`, defines the shared request-count and resource workload.

Each typed case contains a current `request`, optional `client` overrides, `replies`, `expected` and an exact paid `requests` count. A top-level `cancelled: true` means cancelling a public token before opening the call. It does not mean starting work and then cancelling its stream. `expected` is an ordered row array or `{"error":"stable.code"}` for a call or admission error. Client construction belongs inside the error check too.

Compare the complete JSON shape and scalar types. Preserve absent fields, IDs, order, answers, probabilities, detail and execution names. Validate source backend and model against the selected client. Only then remove source `backend`, `model` and `question_identity` for the shared expectation, retaining any other source fields. Remove unstable `message` fields only from coded errors. No projection onto the expected keys is permitted. TypeScript uses its documented camelCase names and UTF-16 offsets; other drivers use their documented host offset unit.

## Fixture protocol

Build the unpublished `loopback` test executable once with the shared native test inputs. Start `loopback CASE.json` with standard input open. Its first output line is its free loopback port. Use `http://127.0.0.1:PORT/v1` and a synthetic key. `count` returns the actual paid HTTP request count, including retries. `shutdown` ends the fixture. Close pipes and wait for a successful exit. All control reads and child waits have deadlines.

Reply rules contain `probability` or `distribution`. Optional `contains` matches instruction text; optional `criterion` matches a wire criterion name. The first matching rule wins. A `status` and `body` rule supplies a whole-request response. Status zero deliberately closes the connection without HTTP bytes and proves transport failure; it is not an HTTP status. Fixtures carry no real credentials.

## Owning boundaries

`public-contract/usage-admission.json` supplies malformed dynamic inputs for question, toolbox and settings boundaries. Drivers invoke their public boundary, compare the exact stable code and diagnostic, and check secret redaction. `construction/options.json` checks public settings, named execution and invalid function options. `construction/stub.json` supplies complete deterministic stub rows. Other files under `public-contract/`, `native/` and `toolbox/` belong to their named Rust, ABI, saved-question or database boundary tests.

A case may declare `not_applicable`, an object from the existing canonical surface ID to a nonempty reason. This is limited to an input the public boundary cannot express. Drivers print the reason as skipped, and never count it as a passing execution. An unmarked skip fails. For eager R and SQL APIs, the absence of a persistent pre-call token requires held-call interruption on every function instead. R's native character type cannot contain NUL, so those cases retain a conversion-boundary refusal rather than a fabricated string. SQL's required arguments and scalar types may reject an input before JSON admission; its own refusal and secrecy checks still run. Valid but invalid-option values never receive these exemptions.

The construction example owns every generated semantic field. Surface applicability is reviewed metadata in the checked fixture files. The schema check compares all generated fields exactly and validates applicability surface IDs and reasons separately.
