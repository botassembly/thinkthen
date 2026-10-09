# Thin, first-class bindings in 0.2

## Ruling

Ian, 2026-10-09: "Now is the time to fix all this up because we can't be dealing with this long term. I want to make all the right decisions and make 0.2 as strong as possible."

His two goals, in his words: "The least amount of code, maximizing pushing as much of the logic down to the Rust layer" and "having a first-class client experience, whether it's things like nulls, the different types, etc. Going to and from your normal language should work as expected. An agent working in Dart or an agent working in C# shouldn't have a subpar experience just because the code happens to be Rust-based underneath." He hopes 0.2 is the final shape of the ten functions across every surface.

He chose the full target shape (option A) over thin-now-first-class-later (B) and generated layouts with hand-kept readers (C). The PM applied his instruction to make all the right decisions to the two open choices: 0.2 removes the 0.1 JSON-string calls so each language keeps one public API, and Objective-C moves to Apple Foundation. Ian can overturn either.

## Reason

Three read-only audits at main cd3950f06 found no surface first-class and no surface thin. Rules restated outside Rust, hand-copied result schemas and C layouts, two to five APIs per language, synchronous calls in async languages, and packages without their native library recur across surfaces. Hand-copied readers in Python, Ruby, R and TypeScript lacked facts fields added by 0461 and 0468. The analysis and per-surface scores are in Ian's note `notes/thinkthen-0-2-thin-first-class-bindings.md`.

## The rule

Rust owns every rule. Each language owns only its idiom.

- Core owns the request grammar, limits, defaults and validation. No surface restates them.
- Core owns the result types. Each language's typed results are generated, with explicit presence where missing and null differ.
- Direct-to-Rust languages expose Rust-owned typed results with the language's expected object behavior.
- C-interface languages call the JSON session interface (0503) and add only async, cancellation, cleanup and naming idiom.
- C, Zig, Ada and COBOL read fixed layouts generated from Rust.
- Every package carries its prebuilt native library.
- One public API per language.

## Replaces

The 0502 choice of generated layouts with hand-kept readers for the C-interface languages, except for C, Zig, Ada and COBOL. The `libraries/BINDING-AUTHOR.md` rule that results stay plain host values. The 0.2 scope stays frozen to this work, bugs and TCGA blockers.
