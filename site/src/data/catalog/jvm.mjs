import { NATIVE_PLATFORMS } from './platforms.mjs';

export default [
{
    slug: 'java', name: 'Java', deckHeading: null,
    versions: { runtime: 'JDK 22 or later', platforms: NATIVE_PLATFORMS },
    backends: 'code',
    lang: 'java', tab: 'Java',
    blurb: "Call the ten functions from Java 22 or later. Each call returns a future.",
    call: 'Open a `thinkthen.Engine`, call `engine.decide(question, input)`, and `join()` the future it returns.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "Owned packets retain generated results, explicit presence and terminal facts.",
    errors: "Native admission and session failures retain typed errors and completed results.",
    settings: "`Engine` takes generated settings; Rust owns recording and replay.",
    install: [
      ['io.github.botassembly:thinkthen-jvm:VERSION', 'The Maven coordinate. Run with `--enable-native-access=ALL-UNNAMED`.'],
      ['thinkthen-jvm-VERSION-TARGET.tar.gz', 'The session, Kotlin and Scala JARs, from each release.'],
    ],
    particular: ["Use `--enable-native-access=ALL-UNNAMED`; no `--enable-preview` flag or manual library path is required.", "The package loads its bundled native target."],
  },
{
    slug: 'kotlin', name: 'Kotlin', deckHeading: null,
    versions: { runtime: 'JDK 22 or later', platforms: NATIVE_PLATFORMS },
    backends: 'code',
    lang: 'kotlin', tab: 'Kotlin',
    blurb: "Call the ten functions from Kotlin as suspend functions.",
    call: 'Open a `KotlinEngine` and call its `decide` suspend function from a coroutine.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "Generated owned packets retain results, presence and terminal facts.",
    errors: "Typed native failures preserve completed results and actual facts.",
    settings: "`KotlinEngine` uses the generated JVM engine settings.",
    install: [
      ['io.github.botassembly:thinkthen-jvm:VERSION:kotlin', 'The Maven coordinate with the `kotlin` classifier. It needs no separate Java dependency. Add `org.jetbrains.kotlinx:kotlinx-coroutines-core-jvm:1.8.0` beside it. Run with `--enable-native-access=ALL-UNNAMED`.'],
      ['thinkthen-jvm-VERSION-TARGET.tar.gz', 'The session, Kotlin and Scala JARs, from each release.'],
    ],
    particular: ["Stable JDK 22 or later is required, with native access and no `--enable-preview` flag.", "Kotlin cancellation and cleanup follow the shared owned session."],
  },
{
    slug: 'scala', name: 'Scala', deckHeading: null,
    versions: { runtime: 'JDK 22 or later', platforms: NATIVE_PLATFORMS },
    backends: 'code',
    lang: 'scala', tab: 'Scala',
    blurb: "Call the ten functions from Scala. Each call returns a Future you can cancel.",
    call: 'Open a `ScalaEngine` and call `decide`. It returns a `Future` you can cancel.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "Generated owned packets retain results, presence and terminal facts.",
    errors: "Typed native failures preserve completed results and actual facts.",
    settings: "`ScalaEngine` uses the generated JVM engine settings.",
    install: [
      ['io.github.botassembly:thinkthen-jvm:VERSION:scala', 'The Maven coordinate with the `scala` classifier. It needs no separate Java dependency. Keep the normal Scala runtime libraries. Run with `--enable-native-access=ALL-UNNAMED`.'],
      ['thinkthen-jvm-VERSION-TARGET.tar.gz', 'The session, Kotlin and Scala JARs, from each release.'],
    ],
    particular: ["Stable JDK 22 or later is required, with native access and no `--enable-preview` flag.", "Keep the normal Scala runtime libraries on the class path."],
  }
];
