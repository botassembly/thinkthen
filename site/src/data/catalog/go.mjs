
export default {
    slug: 'go', name: 'Go', deckHeading: null,
    versions: { runtime: 'Go 1.22 or later', platforms: 'Linux x86-64' },
    backends: 'code',
    lang: 'go', tab: 'Go',
    blurb: "Call the ten functions from Go, with a context for deadlines and cancellation.",
    call: 'Create a client with `thinkthen.NewClient` and call `client.Decide(ctx, question, input, nil)`.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "A completed call retains its full records and final facts.",
    errors: "`SessionError` retains failed calls; admission returns typed `Error` values.",
    settings: "`NewClient(EngineSettings)` forwards native settings to Rust.",
    install: [
      ['go get github.com/botassembly/thinkthen/libraries/go', 'Needs a C compiler.'],
      ['thinkthen-go-VERSION-TARGET.tar.gz', 'The module source, from each release.'],
    ],
    particular: ["The native-bearing package requires Go and a C compiler.", "A context deadline reaches native execution; producers must return promptly on cancellation."],
  };
