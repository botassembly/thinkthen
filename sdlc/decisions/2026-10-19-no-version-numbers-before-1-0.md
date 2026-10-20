# No version numbers in ThinkThen's formats before 1.0

Ian, 2026-10-19: "I don't really want version numbers. There's no versioning for anything at this point. It's all just functions." And: "If I change something in the future, then someone has to fix their code. Until we get to 1.0. And then, at that point, I'll not break anything."

- ThinkThen's inputs, outputs and stored formats carry no version number or version marker before 1.0. Question files, question sets, profiles, configuration, result packets and type names drop theirs.
- A format changes in place. Callers update their code with the release that changes it, and the release notes say what changed.
- This extends [no compatibility before 1.0](2026-10-12-no-compatibility-before-1-0.md): no code accepts an old shape, and no version field selects between shapes.
- Package release numbers, tool pins, minimum host versions and model names are not format versions and stay.
- A check that a language wrapper loaded the native library from the same build may stay, as a build identity, not a format version.
- At 1.0, Ian decides how ThinkThen keeps compatibility from then on.

The removals land in 0.2.1, before the core freeze, so every language picks them up once.

Ian, later the same day: the question author's `wording_version` goes too. The SDK asks the question directly; versions of a question's wording belong to the proxy's question optimization, not the core SDK.
