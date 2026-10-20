import { MAC_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'swift', name: 'Swift', deckHeading: null,
    versions: { runtime: 'Swift 6.0 or later', platforms: `Linux x86-64; ${MAC_PLATFORMS}` },
    backends: 'code',
    lang: 'swift', tab: 'Swift',
    blurb: "Await the ten functions from Swift with async throws.",
    call: 'Create a `Client` and call `try await client.decide(.text(question), input: InputText(text: text))`. Read the returned value.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "`CallResult.facts` counts the call; `CallResult.results` retains its complete records.",
    errors: "`SessionFailure.call` preserves typed failure settlement and completed packets.",
    settings: "`Client` takes generated engine settings and loads its packaged native target.",
    install: [
      ['thinkthen-swift-VERSION-TARGET.tar.gz', 'The package source, from each release.'],
    ],
    particular: ["Close the client after use; generated results remain owned.", "Rust owns files, images, cache, replay and admission."],
  };
