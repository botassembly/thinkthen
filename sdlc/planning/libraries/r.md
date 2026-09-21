# The R surface: goals and anti-goals

Shared rules live in [README.md](README.md). This page holds only what is particular to R.

## What really good looks like

An R user works a whole column at a time. A great R package takes a column and gives back one of the same length and order, in R's own type, inside `dplyr::mutate` or `dplyr::filter`. It says `NA` when unsure, because `is.na` is how R users find those rows. R is where Ian's speed rule is easiest to keep: one call carries the column down, and the engine runs its requests at once inside Rust.

```r
library(thinkthen)

triaged <- tickets |> mutate(
  team   = tt_choose("Which team owns this?", body, c("billing", "shipping", "other")),
  refund = tt_decide("The customer asks for a refund.", body, threshold = c(0.1, 0.9))
)
triaged |> filter(is.na(refund)) |> send_to_a_person()
```

## Goals

- One `tt_decide()` over n rows crosses the barrier once and runs n requests at the `jobs` width. R keeps the vectorized `decide`, which is its habit (ADR 0017 pick 8); the engine's batch spine carries it (section 8).
- Each verb returns R's own type: logical, `NA` under a band, a character vector from `tt_choose`, numeric from `tt_score` — the specification's position from 0 to K−1, with the nearest level's name in `tt_details` (ADR 0017 pick 6) — a list column from `tt_tag`. Order and names match the input, and `NA` evidence in returns `NA` with no request. `NA` evidence handed to `filter` is a usage error, as 205 measured; the two verbs differ on purpose and the page says so.
- Ctrl-C during a long column returns control within one poll interval. The pattern is one plain worker thread, a channel, and a 100 ms `R_CheckUserInterrupt` poll on the main thread; the small receiver leak per interrupt is named in the shim's notes (205).
- `Imports:` names nothing beyond base R, and the tests still run the verbs inside `dplyr`.
- `install.packages("thinkthen")` on Windows and macOS installs a binary.

## Anti-goals

- No verb called per row, such as `vapply(body, tt_decide, logical(1))`. Each call pays a crossing and a round trip.
- No `NULL` and no `FALSE` for unsure. R has `NA`, and `if (NA)` is already an error.
- No S4 class and no R6 client. A constructor in the first example reads like a vendor SDK.
- No call into R's C API from a worker thread. R is single threaded, and a `SEXP` touched off it corrupts memory.
- No source-only release and no empty placeholder on CRAN. Windows users have no toolchain, and CRAN reviews by hand.

## Where this language wastes time

- **A crossing per element.** `.Call` converts each `CHARSXP` down and allocates up, and `mutate` copies the column it assigns. Fix: one crossing down, and one answer vector allocated in Rust at its final type. The character vector is already the widest container R has. A data frame is a list of vectors, so a column is that same vector and `VECTOR_ELT` hands it over with no copy.
- **String re-encoding.** A character vector holds pointers into R's global `CHARSXP` cache, so equal strings already share storage and nothing is copied to reach Rust. extendr's `Strings` dereferences to a slice of `Rstr`, and reading one as `&str` views the `CHARSXP` bytes in place. That read assumes UTF-8 and checks no encoding mark, so the shim reads the mark itself. `translateCharUTF8` copies only when it re-encodes, and it returns the existing pointer for a UTF-8 or ASCII string.
- **An Arrow column turned into a character vector first.** The `nanoarrow` package carries the Arrow C data and stream interfaces with no dependency on the `arrow` package, and it holds them as external pointers. `arrow-extendr` turns those into `FFI_ArrowArray` and `FFI_ArrowSchema` for Rust, so a string column arrives as one contiguous UTF-8 buffer with offsets. `nanoarrow` would be suggested, and `Imports:` still names nothing.
- **Work the shim must not do.** The engine asks an equal pair of question and evidence once inside a batch, and a cached answer costs nothing. No `unique()` in R before the call. `jobs` is one number for the process, R runs one thread, and two calls in one session share that width. A per-row call is serial, and one `tt_decide()` over the column is the bulk form.
- **The interrupt poll.** Only the main thread may read R's interrupt flag. Fix: the engine works on its own threads while the main thread waits on a channel with a short timeout. Rust never calls into R while waiting.

## How little code

The binding tool is `extendr`: the `extendr-api` crate with the `rextendr` package. It generates the R wrappers, the `.Call` registration, and the `NAMESPACE`, and handles `SEXP` protection.

The shim converts the column to string slices, maps arguments to engine options, builds the answer vector, raises a failure through `stop()` as an R condition class — `thinkthen_usage`, `thinkthen_backend`, `thinkthen_local`, `thinkthen_cancelled`, `thinkthen_deadline`, `thinkthen_defect`, the six of ADR 0017 section 3, so a runner reads the kind with `inherits` instead of grepping a message prefix (205) — polls for an interrupt, and draws progress. It never holds threshold math, JSON, retries, rate limit waits, the recording format, the key, or scheduling.

`src/Makevars` builds the crate sources vendored as one opaque tarball under `src/rust`, so the build works offline on CRAN's machines; a directory of sources rots and the tarball states the glibc bound on Linux binaries. R-universe needs no review and publishes from GitHub with `SystemRequirements: Cargo (Rust)`, so it is the first home and CRAN second. The measured costs belong beside the plans: 89 s to install from source, 8.8 MB installed (205). Prebuilt binaries cover Windows and macOS. Linux coverage is unchecked, and so is Bioconductor.

## Tests only this surface needs

- An interrupt during a long column returns control and leaves no thread alive.
- Vectors marked UTF-8, marked latin1, and native encoded give the same request bytes.
- `tt_choose` gives one value per row in label order, even when every answer is `NA`.
- A `targets` or knitr rebuild under replay makes no network call.
- A bench counts rows a second through one `tt_decide()` over a long column against the stub, beside the engine's own number from pure Rust. A gap is a defect in the shim.

## Open questions for the ADR

1. Where does `details` live? One column per call is the whole R grammar. A spliced data frame and a `tt_details()` verb are the candidates.
2. How is replay scoped with no dependency? A `tt_replaying(dir, expr)` wrapper, an `options()` entry, and an inherited environment variable nest differently.
3. Answered by measurement (205, round two): the Arrow door does not ship. The character vector already crosses with no copy, arrow's ALTREP hands its own column over with no pull step, and the per-row materialization tax (0.3–0.5 µs) is 0.03 percent of a wire call, while `arrow` costs a user 58.5 MB and 258–285 ms to load, 65 times the import budget. `Imports:` stays empty; `Suggests: nanoarrow` is the shape if a caller ever asks, loaded at call time.
4. Does `tt_choose` return a character vector or a factor? The agreed grammar passes the answer to `switch()`, and `switch()` refuses a factor.
5. `tt_choose`, `tt_score`, and `tt_tag` are designed here but unproven on a stand-in: the 205 engine resolved only the decide family. The real engine's conformance cases prove them, and until then they are unchecked, not promised.
6. Progress for a long column hangs on the poll loop. 205 did not build it, staying under the line ceiling; it stays open.
