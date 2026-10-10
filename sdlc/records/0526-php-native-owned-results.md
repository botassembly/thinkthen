# 0526: PHP calls own native session results

## Caller behavior

This slice builds on `e2382c091`. `ThinkThen\Client` provides the ten named synchronous calls with ordinary PHP inputs. `Operation` owns native polling, bounded feed, cancellation and deterministic cleanup. The client keeps weak references to operations and closes them before freeing its engine, including when the client is destroyed while an operation remains referenced. The native worker keeps its own engine ownership. No host admission, cache identity, probability reading or route policy is added.

Generated PHP classes expose each native field through `Presence`, with absence independent of null and false. Objects and arrays retain their distinct PHP forms. Unknown extensions stay accessible. Every native packet, including observations, is retained. Results survive client closure. Native failures retain their typed failure, terminal, completed results and packets; immediate refusals invent no facts.

PHP's signed integer range cannot hold every native unsigned count. The PHP representation uses exact decimal strings for larger integers, following `JSON_BIGINT_AS_STRING`; `nativeJson()` on each packet retains the unchanged native JSON, including numeric types. This is a host representation choice, not a native schema change.

## Source and package

The PHP target consumes the existing prepared Rust result graph. Its class/member inventories and request version are generated. FFI declarations are stripped directly from the generated C header; no layouts are hand-copied. The exact generated FFI header path is exempted from the handwritten file-size limit. It contains 3,926 nonblank lines and 208,085 bytes. The generated PHP result declarations contain 905 nonblank lines and 83,636 bytes and stay together because they represent one generated graph, not handwritten logic. Generated constants contain 17 lines and 569 bytes; the schema-derived conversion graph contains 5,906 JSON lines and 116,171 bytes.

PHP source grows from 1,759 to 3,136 nonblank lines, including generated declarations and focused consumers. Of the increase, 922 lines are generated PHP declarations/constants. Python fixtures grow from 1,175 to 1,256 after replacing their manual source-package file list with source discovery and adding local archive assembly plus the installed consumer. The target template has 54 nonblank Python lines. Rust source is unchanged.

The local archive includes package source and one supplied native library. An actual Composer consumer installs that archive with plugins, scripts and network disabled, then loads `vendor/autoload.php`. This fixture carries the bounded debug native build; it does not qualify a production distribution.

## Checks

- C shared library rebuilt with locked offline dependencies, two jobs and the bounded memory scope.
- PHP syntax checks pass for every new handwritten and generated PHP file.
- Shared generation freshness passes with the consolidated PHP target registration from `f07fcce75`. That registration is owned by the TypeScript slice and is not duplicated here.
- Installed Composer checks pass for all ten named calls and typed results retained after client closure; absent/null/false, an unknown extension and an unsigned count beyond PHP's range; physical file lines and bounded feed order; retained backend failure facts; typed local failure for a missing native library, counted zero-send invalid input and pre-cancellation; client destruction with an open operation while the provider reply remains held, followed by progress on another engine.
- Both PHP source ceilings match their measured totals. Policy and whitespace checks are recorded against the committed branch before review.
- The existing installed compatibility consumer passes both routes, with three counted arrivals each, after its package copy discovers the new source files.

The final local archive is `target/0526/package.tar.gz`, SHA-256 `6b4eb5d7f69f971160fcd8d95bb2dc9a4c1d21b75ec5654e7149413c2ef33e90`. It carries the unchanged 59,292,496-byte debug native library, SHA-256 `ca587e92f9898621346d917731dc039a61e29bda3076706b0ed04a5c29607b7d`.

Composer 2.10.3 was downloaded into ticket scratch from its official endpoint and verified against the official SHA-256, `7a2d379d5b8ffdaa028580ef26494c36d2feef4b178d3dd1473a4dbc5e17c8d6`. All product consumers use the shared clean child environment and owned temporary homes. No paid calls, full parity, large-input suite or release action ran.

## What the build taught us

Fresh review found that the schema-directed map converter returned PHP arrays even though the generated public methods promise `stdClass`. Empty annotation maps therefore became `[]`. The converter now keeps objects while converting their members recursively. The existing installed presence group failed against the prior archive, then passed against the repaired archive, checking populated and empty maps, typed answer members and nested tag arrays. All five installed groups pass without rebuilding Rust. The repair adds 13 nonblank fixture lines and changes two decoder lines; Python and generated declarations are unchanged.

PHP's FFI parser can load the canonical header after removing comments, preprocessor directives and C++ linkage guards; retaining a copied declaration inventory is unnecessary. The shared graph also needs its optional single-type arrays normalized during conversion, or nested native source records remain untyped PHP objects. An installed nested-source assertion catches that failure.

A weak operation registry prevents an open batch from keeping its client alive. The held-provider case verifies destruction closes the actual operation before a reply is released. Packet JSON gives callers a lossless route around PHP's integer limit without inventing wider numeric values.

The existing synthetic backend reserves certain short record names for its three-worker ordering fixture. This focused API check uses distinct names, so it exercises synchronous calls without accidentally starting that unrelated concurrency fixture.

## Remaining owning outcomes

Full shared installed parity, broader file/image and recognize coverage, final platform packaging under 0530 and compatibility API/reader retirement remain. The old public APIs and readers are retained in this slice. This slice does not claim the whole ticket complete.

## PHP attribution adoption

This bounded follow-up builds on `1c3ec5ba4`. The operation constructor calls the additive native `thinkthen_session_new_with_surface` export with the fixed `php` token. The canonical generator refreshes only the PHP FFI header, adding six nonblank declaration lines and 395 bytes. The result classes, conversion graph and constants remain unchanged; native layouts remain unchanged. The existing generated-header policy exception needs no change.

The installed archive case first failed against the previous PHP constructor: the loopback backend received `thinkthen/0.2.0 (c)`. The repaired archive sends exactly one request with `thinkthen/0.2.0 (php)` and retains typed terminal send facts and the native call identity after client closure. Facts have no surface field; attribution is proved by the actual received User-Agent. All six installed Composer groups pass with a five-second limit per process, including the existing named calls, presence, backend failure, zero-send rejection and open-operation destruction groups. Composer installs the actual archive with plugins, scripts and network disabled.

The follow-up archive is `target/0526-surface/after.tar.gz`, SHA-256 `16a67f45ca2ee523724c3fdb2c1f64a69a0ea67f263f6866dfb4614e22224206`. It uses the existing 59,315,656-byte debug library, SHA-256 `1d1248852a9184b1f77e71052e153be28f8abf4075315d855c5105631a7134f7`. No Rust rebuild ran. Composer 2.10.3 was downloaded from its official endpoint and matched the official SHA-256 already cited above. PHP generation freshness, both measured source ceilings, affected PHP syntax and whitespace checks pass. The ceilings each grow by six lines for the focused installed regression and received-header capture.

The regression exposes a structural gap: changing the native default fixes direct C attribution but requires host constructors to adopt the explicit wrapper export. Header freshness alone cannot prove the host selects its own token. The installed request checks that selection through the real archive. No paid calls, full parity, routine suite, stress cases, large inputs, model runs, platform builds or release actions ran.
