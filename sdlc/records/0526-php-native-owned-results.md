# 0526: PHP calls own native session results

## Caller behavior

This slice builds on `e2382c091`. `ThinkThen\Client` provides the ten named synchronous calls with ordinary PHP inputs. `Operation` owns native polling, bounded feed, cancellation and deterministic cleanup. The client keeps weak references to operations and closes them before freeing its engine, including when the client is destroyed while an operation remains referenced. The native worker keeps its own engine ownership. No host admission, cache identity, probability reading or route policy is added.

Generated PHP classes expose each native field through `Presence`, with absence independent of null and false. Objects and arrays retain their distinct PHP forms. Unknown extensions stay accessible. Every native packet, including observations, is retained. Results survive client closure. Native failures retain their typed failure, terminal, completed results and packets; immediate refusals invent no facts.

PHP's signed integer range cannot hold every native unsigned count. The PHP representation uses exact decimal strings for larger integers, following `JSON_BIGINT_AS_STRING`; `nativeJson()` on each packet retains the unchanged native JSON, including numeric types. This is a host representation choice, not a native schema change.

## Source and package

The PHP target consumes the existing prepared Rust result graph. Its class/member inventories and request version are generated. FFI declarations are stripped directly from the generated C header; no layouts are hand-copied. The exact generated FFI header path is exempted from the handwritten file-size limit. Its 3,920 nonblank declarations remain recorded here. The generated PHP result declarations have 905 nonblank lines and stay together because they represent one generated graph, not handwritten logic.

PHP source grows from 1,759 to 3,111 nonblank lines, including generated declarations and focused consumers. Of the increase, 922 lines are generated PHP declarations/constants. Python fixtures grow from 1,175 to 1,250 after replacing their manual source-package file list with source discovery and adding local archive assembly plus the installed consumer. The target template has 53 nonblank Python lines. Rust source is unchanged.

The local archive includes package source and one supplied native library. An actual Composer consumer installs that archive with plugins, scripts and network disabled, then loads `vendor/autoload.php`. This fixture carries the bounded debug native build; it does not qualify a production distribution.

## Checks

- C shared library rebuilt with locked offline dependencies, two jobs and the bounded memory scope.
- PHP syntax checks pass for every new handwritten and generated PHP file.
- Shared generation freshness passes with the consolidated PHP target registration from `f07fcce75`. That registration is owned by the TypeScript slice and is not duplicated here.
- Installed Composer checks pass for all ten named calls and typed results retained after client closure; absent/null/false, an unknown extension and an unsigned count beyond PHP's range; physical file lines and bounded feed order; retained backend failure facts; counted zero-send invalid input and pre-cancellation; client destruction with an open operation while the provider reply remains held, followed by progress on another engine.
- Both PHP source ceilings match their measured totals. Policy and whitespace checks are recorded against the committed branch before review.

Composer 2.10.3 was downloaded into ticket scratch from its official endpoint and verified against the official SHA-256, `7a2d379d5b8ffdaa028580ef26494c36d2feef4b178d3dd1473a4dbc5e17c8d6`. All product consumers use the shared clean child environment and owned temporary homes. No paid calls, full parity, large-input suite or release action ran.

## What the build taught us

PHP's FFI parser can load the canonical header after removing comments, preprocessor directives and C++ linkage guards; retaining a copied declaration inventory is unnecessary. The shared graph also needs its optional single-type arrays normalized during conversion, or nested native source records remain untyped PHP objects. An installed nested-source assertion catches that failure.

A weak operation registry prevents an open batch from keeping its client alive. The held-provider case verifies destruction closes the actual operation before a reply is released. Packet JSON gives callers a lossless route around PHP's integer limit without inventing wider numeric values.

The existing synthetic backend reserves certain short record names for its three-worker ordering fixture. This focused API check uses distinct names, so it exercises synchronous calls without accidentally starting that unrelated concurrency fixture.

## Remaining owning outcomes

Full shared installed parity, broader file/image and recognize coverage, final platform packaging under 0530 and compatibility API/reader retirement remain. The old public APIs and readers are retained in this slice. The public C session currently uses the native default Rust surface; PHP preserves that report. PHP-specific attribution remains a shared C-session outcome under 0503/0526. This slice does not claim the whole ticket complete.
