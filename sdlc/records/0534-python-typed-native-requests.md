# Python typed native requests

Python now converts host arguments into Rust Request, Input, Question, Options and feed values. The native extension uses shared admission and reader rules; the host no longer assembles the request grammar or sends whole requests through JSON. Generated result readers keep their existing behavior.

Fresh code review found that the first conversion lost `details` on six named calls and admitted Boolean values as numeric controls. The correction retains details on all ten calls and rejects Boolean values before numeric conversion. File readers also validate their ReaderOptions when constructed. A fresh reviewer accepted these corrections. The Python production code shrank by 34 lines; its counted handwritten Python ceiling fell from 3033 to 3025. The Rust adapter grew from 3169 to 3606 lines. Review accepted that conversion cost because Rust owns the request rules and the host grammar was deleted.

The installed run passed 35 behavior tests, mypy and 120 shared cases across Python, pandas and Polars. The final correction passed 19 focused tests covering details, named arguments, numeric bounds and reader admission. Policy, source ceilings and commit-history checks passed. Routine lint completed in stages after correcting ticket summaries and the Python ceiling. A resumed lint command mistakenly enabled Cargo offline mode and failed a local Git dependency plant; the corrected invocation passed the remaining checks, including documentation and inventory with 1957 declarations and four refused plants.

The unchanged core routine suite is reused from the 0539 closure. Rebasing this Python branch onto that landed core changed no Python code. Final release gates belong to 0542 and 0543.

The retained local evidence is in `target/0534-python-typed-native-requests/` in the main checkout: installed consumer output, final focused tests, policy output and closure lint. These build outputs are not committed.

## What the build taught us

Conversion must preserve every public argument before shared admission. Strict host integer extraction prevents Boolean values from silently becoming request controls. Native conversion can add adapter code while deleting the second request grammar; the source ceiling must state that cost accurately.
