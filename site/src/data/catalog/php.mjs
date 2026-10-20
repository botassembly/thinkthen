
export default {
    slug: 'php', name: 'PHP', deckHeading: null,
    versions: { runtime: 'PHP 8.3 or later', platforms: 'Linux x86-64' },
    backends: 'code',
    lang: 'php', tab: 'PHP',
    blurb: "Call the ten functions from PHP 8.3 through FFI.",
    call: 'Create a `ThinkThen\\Client` and call `$client->decide($question, $text)`.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "Generated result objects retain presence, completed answers and terminal facts.",
    errors: "Typed native failures retain their kind, retryability and final settlement.",
    settings: "`Client` forwards native settings to Rust and loads its packaged native library.",
    install: [
      ['composer require botassembly/thinkthen', 'Needs `ext-ffi`. Run PHP with `-d ffi.enable=1`.'],
      ['thinkthen-php-VERSION-TARGET.tar.gz', 'The package source, from each release.'],
    ],
    particular: ["Enable PHP FFI and release the client after use.", "Use `Client::item` and `Client::files` for explicit input selectors."],
  };
