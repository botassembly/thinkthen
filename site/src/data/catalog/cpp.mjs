
export default {
    slug: 'cpp', name: 'C++', deckHeading: null,
    versions: { runtime: 'C++17', platforms: 'Linux x86-64' },
    backends: 'code',
    lang: 'cpp', tab: 'C++',
    blurb: "Call the ten functions from C++17 through a header-only CMake package.",
    call: 'Open a `tt::Client` and call one of its ten named methods, such as `decide`, with a question and the text.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "`call.result()` returns the value, probability, complete records and final call counts.",
    errors: "`tt::Failure` retains the failure kind, retryability, completed packets and available facts.",
    settings: "`tt::Client` takes native settings. Rust owns configuration, recording and replay.",
    install: [
      ['thinkthen-cpp-VERSION-TARGET.tar.gz', 'The header and a CMake package, from each release. Needs a compiler and CMake.'],
    ],
    particular: ["Each named method returns a `ResultCall`. Read its completed result with `result()`; use `collect` or `poll` to read packets.", "Use `tt::inputs` for typed questions and inputs. The result owns its records after the call closes."],
  };
