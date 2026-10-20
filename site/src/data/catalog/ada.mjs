
export default {
    slug: 'ada', name: 'Ada', deckHeading: null,
    versions: { runtime: 'Ada 2022; GNAT 13.3', platforms: 'Linux x86-64' },
    backends: 'code',
    lang: 'ada', tab: 'Ada',
    blurb: "Call the ten functions from Ada 2022 through a GNAT project.",
    call: 'Build the question and input and call a named function in `Thinkthen.Inputs.Calls`, such as `Decide`. Read the returned result\'s `Value` field.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "The returned result has `Value`, `Probability`, `Results` and `Facts`. Its owned records survive session finalization.",
    errors: "Typed native failures preserve kind, retryability and owned messages.",
    settings: "`Configure` forwards engine settings to Rust.",
    install: [
      ['thinkthen-ada-VERSION-TARGET.tar.gz', 'The package and its GNAT project, from each release. Needs gprbuild.'],
    ],
    particular: ["Release owned sessions after their borrowers finish.", "Direct ABI planning remains; no named typed Ada Plan result is claimed."],
  };
