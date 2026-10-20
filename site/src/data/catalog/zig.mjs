
export default {
    slug: 'zig', name: 'Zig', deckHeading: null,
    versions: { runtime: 'Zig 0.15.2 or later', platforms: 'Linux x86-64' },
    backends: 'code',
    lang: 'zig', tab: 'Zig',
    blurb: "Call the ten functions from Zig through a source module.",
    call: 'Call a named function in `thinkthen.calls` with `calls.Arguments(Input)`. Read the successful result\'s `value` field and release that result with `deinit`.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "The successful result has `value`, `probability`, `results` and `facts`. Its owned records survive native session cleanup.",
    errors: "Handle the `failed` arm of `thinkthen.Result` and release it with `thinkthen.releaseFailure` using the engine allocator. Started failures retain packets and terminal facts.",
    settings: "Generated engine settings forward recording and replay configuration to Rust.",
    install: [
      ['thinkthen-zig-VERSION-TARGET.tar.gz', 'The module source, from each release.'],
    ],
    particular: ["The package carries its generated header and native engine.", "`Request.plan` returns an owned plan; release owners with `deinit` after borrowers finish."],
  };
