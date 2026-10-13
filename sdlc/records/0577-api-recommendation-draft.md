# Draft DuckDB API recommendation

Keep the existing C++ extension for 0.2. Qualify DuckDB 1.5.6 and the 2.0 preview separately. Do not add 1.4 LTS to this release unless a caller needs it. Investigate the stable C extension API as a separate, bounded follow-up before committing to a port. Ian can overturn this recommendation.

## Engineering cost

The current implementation has 36 C++ source/header files. Its work reaches beyond scalar registration: it registers SQL macros and table functions, reads connection settings, opens files through DuckDB, shares statement ownership and checks interruption. A C port must replace those host-facing pieces while retaining the Rust bridge and every SQL signature. The five work streams are registration/results, connection settings, authorized file handles, statement lifecycle/cancellation, and build/footer packaging. The existing SQL behavior, permission, prepared execution, cancellation, cache and co-load checks must run against the port. Registration alone would not prove equivalent behavior.

DuckDB 1.5.6's C extension header exposes config-option registration, function bind/init callbacks and file-system handles in its 1.5.6 API band. That removes several earlier obstacles. The current C++ statement owner also uses ClientContextState query callbacks and ClientContext interruption state; the C header does not establish an equivalent whole-statement cancellation/lifetime contract. A prototype must preserve that behavior before choosing the port. Dropping it or opening files outside DuckDB would change the product contract.

## Build and maintenance cost

| Choice | Release build work | Compatibility checks | Recurring maintenance |
| --- | --- | --- | --- |
| Keep C++ for 1.5.6 and 2.0 | Eight artifacts across two Linux and two Mac targets | Eight SQL/load runs plus the existing host behavior suite on each supported host version | Rebuild each supported DuckDB version and fix any changed C++ interfaces |
| Port to stable C API 1.5.6 | Four platform artifacts if one binary actually loads in both hosts | Eight SQL/load runs using the same four artifacts, plus permission/lifecycle/cancellation parity | Maintain the C adapter and test each new DuckDB release; Rust and native-library packaging remain |
| Add 1.4 LTS to C++ | Four additional artifacts, twelve total | Four additional SQL/load runs and behavior qualification on 1.4 | Maintain another C++ compatibility branch or conditional interface path through LTS updates |

The C API targets its own semantic API version rather than the host version. DuckDB's loader accepts an older minor/patch within the API major. The inspected 2.0 preview header still advertises API 1.5.6, so a stable C artifact targeting that API is a plausible shared binary for these two hosts. This is source evidence, not a successful load test. A 1.5.6 API target cannot also promise 1.4 loading: 1.4 predates the new settings/file/function facilities used above, and the loader rejects newer API versions. Supporting all three through stable C would require an older API floor with a separate solution for those capabilities.

The incremental 1.4 costs buy no named consumer in this ticket. They also do not help the two required community builds. Keep the 0.2 change bounded, then authorize a C prototype only if its cancellation and file-permission behavior can match the existing extension. The maintenance saving is four fewer builds for these two host versions; it does not remove the eight runtime compatibility checks.

## Primary sources

- [DuckDB 1.5.6 C extension header](https://github.com/duckdb/duckdb/blob/069cc9f9b5be802405797faecc284961b07c70ef/src/include/duckdb_extension.h)
- [DuckDB loader API compatibility rule](https://github.com/duckdb/duckdb/blob/069cc9f9b5be802405797faecc284961b07c70ef/src/main/extension.cpp)
- [2.0 preview header inspected at d4355d01c59362d57036ad377735dd5340276a5b](https://github.com/duckdb/duckdb/blob/d4355d01c59362d57036ad377735dd5340276a5b/src/include/duckdb_extension.h)
- [C extension template](https://github.com/duckdb/extension-template-c)
