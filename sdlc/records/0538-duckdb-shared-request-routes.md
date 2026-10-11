# DuckDB shared request routes

DuckDB executes scalar, listed, nested, image, file, find, relate, plan and portable judgments through the shared Request session. The adapter retains SQL results, errors and eager record quotas. Packed try-details retains its approved direct engine call and names the after-0.2 idea for row-by-row recovery in its code comment. Four small native accessors expose the existing Details serialization without adding a format.

The approved annotate behavior rejects an invalid row before any backend send. A missing /body returns the existing bad-row error, and valid neighbours are no longer sent and cached first. The DuckDB README explains this change. Installed regressions expect zero sends on invalid input and retain packed batch counts on valid input. No public admission helper or second direct judgment call was added.

## Review and proof

Fresh code review found that raw session batch parsing changed the established grammar and error. The correction shares the existing engine batch parser. Installed checks cover invalid decimal, sign, padding and overflow forms with the exact error and zero sends, and valid max, 1 and 01 packing. The reviewer accepted the correction and the native, bridge, C++ and Python ceiling changes: +24, +200, +4 and +14 nonblank lines respectively. Integration with the R correction puts the root Rust ceiling at 189491.

Installed routine checks passed on DuckDB 1.5.4 and 1.5.5, including native shared cases, matching-version loading and opposite-version refusal. Earlier failures led to repairs for recognition accumulation, rank diagnostics, eager quotas, package fixture host sources and the documented default toolchain path. Every build and check process was awaited.

Closure lint passed in stages. The first attempt found C++ and Python ceilings that needed the reviewed updates; a temporary resumption script then needed its REPO variable. The retained original and corrected logs show those failures and the remaining passing lint stages. Integrated policy, registry and public inventory checks cover the rebased result.

The routine workspace attempt passed 1807 cases, failed one scratch-folder creation and left 187 cases unrun. Its native-store fixture reused a process-ID folder left by an earlier process. An owned temporary directory allowed the exact 188 failed or unrun cases to pass, completing coverage of all 1995 routine cases. Supporting doctests, installed Rust consumer checks, fixtures, transforms and live checks also passed. This is staged coverage; it does not claim an uninterrupted routine run. The fixture isolation defect belongs to the final 0542 correction and uninterrupted gate.

Original installed, build, review-correction and closure logs are retained under `target/0538-duckdb-shared-requests/` in the main checkout. The folder contains original failures, corrected checks and the temporary resumption inputs. It is local build evidence and is not pushed.

## What the build taught us

Shared request admission can deliberately change backend side effects while preserving a public error. That behavior needs an explicit ruling and an installed zero-send assertion. Reuse the established settings parser rather than duplicating its grammar in an adapter. Test scratch folders must belong to the run; a process ID alone does not isolate them across repeated runs.
