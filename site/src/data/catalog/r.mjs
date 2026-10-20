import { UNIX_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'r', name: 'R', deckHeading: 'R',
    versions: { runtime: 'R 4.2 or later', platforms: UNIX_PLATFORMS },
    backends: 'code',
    lang: 'r', tab: 'R',
    blurb: "Call `tt_decide` and nine more functions from R, on text, vectors and data frames.",
    call: 'Call `tt_decide(question, input)` and read the answer from `call$value`. The other nine functions follow the same shape, such as `tt_choose` and `tt_rank`.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "`call$results` retains generated rows and `call$facts` retains native settlement.",
    errors: 'Each failure arrives as an R condition named for its kind, such as `thinkthen_usage`. Each condition carries `retryable`.',
    settings: "`tt_engine(backend =)` configures the native engine; call controls go in `options = list()`.",
    install: [
      ['install.packages("thinkthen", repos = c("https://botassembly.r-universe.dev", "https://cloud.r-project.org"))', 'On Linux this route builds from source: jsonlite 2.0.0+, R development headers, C compiler/linker, make, cargo and rustc 1.95.0+; installation fetches Rust dependencies.'],
      ['install.packages("thinkthen", repos = c("https://botassembly.r-universe.dev/bin/linux/resolute-x86_64/4.6/", "https://cloud.r-project.org"))', 'Ubuntu 26.04 (Resolute), R 4.6, x86-64 only. Use `resolute-arm64` for ARM64. Dependencies or missing binaries can fall back to source; keep source prerequisites then.'],
    ],
    particular: ["Authored lists supply questions, labels and thresholds.", "`tt_batch` streams generated results; `tt_plan` previews supported native requests."],
  };
