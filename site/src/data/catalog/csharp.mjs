import { NATIVE_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'csharp', name: 'C#', deckHeading: null,
    versions: { runtime: '.NET 8 or later', platforms: NATIVE_PLATFORMS },
    backends: 'code',
    lang: 'csharp', tab: 'C#',
    blurb: "Await the ten functions from .NET 8, with cancellation tokens.",
    call: 'Open an `Engine` with `Engine.Open` and `await engine.DecideAsync(question, input)`.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "Generated `ThinkThen.Results` preserve presence, completed answers and terminal facts.",
    errors: "Typed failures retain native kind, retryability, final facts and completed prefixes.",
    settings: "`Engine.Open(InputEngineSettings)` forwards generated settings to Rust.",
    install: [
      ['dotnet add package Botassembly.ThinkThen', 'The NuGet package.'],
      ['thinkthen-csharp-VERSION-TARGET.tar.gz', 'The package file, from each release. Its folder serves as a local feed.'],
    ],
    particular: ["The package loads its bundled native target automatically.", "Use CancellationToken and scoped cleanup."],
  };
