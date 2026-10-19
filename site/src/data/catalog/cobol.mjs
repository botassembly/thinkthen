
export default {
    slug: 'cobol', name: 'COBOL', deckHeading: null,
    versions: { runtime: 'GnuCOBOL 4', platforms: 'Linux x86-64' },
    backends: 'code',
    lang: 'cobol', tab: 'COBOL',
    blurb: "Call the ten functions from GnuCOBOL through copybooks and called programs.",
    call: 'Fill the request records and call the named program, such as `TT_DECIDE`. Read the value from its returned result record.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "The returned answer record has `v-value`, `v-probability`, `v-results` and `v-facts`, with presence indicators for nullable answers.",
    errors: "`tt-n-cobol-AnswerStatus` preserves native errors and refuses representation overflow. Free its nonnull `v-owner` with `TT_ANSWER_FREE`, including after failure.",
    settings: "The generated engine declarations forward settings to Rust.",
    install: [
      ['thinkthen-cobol-VERSION-TARGET.tar.gz', 'The copybooks, the bridge and the native library, from each release.'],
    ],
    particular: ["Generated declarations require a matching compiler-measured platform layout.", "The package carries its native library."],
  };
