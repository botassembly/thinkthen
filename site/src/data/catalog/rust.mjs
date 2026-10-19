import { NATIVE_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'rust', name: 'Rust', deckHeading: 'Rust',
    versions: { runtime: 'Rust 1.95.0 or later; Polars 0.55.2 or later, below 0.56, for the polars feature', platforms: NATIVE_PLATFORMS },
    backends: 'builder',
    lang: 'rust', tab: 'Rust',
    blurb: 'Call the engine directly. The compiler makes you handle not sure.',
    unsureWord: '`None` from `call.value()` on `decide` or `choose`; `call.probability()` is also `None`',
    facts: '`call.results()` retains complete records; `call.facts()` counts this call.',
    errors: 'Every call returns a `Result`. Its `Error` names one of six kinds. A started call that fails keeps its facts in `Error::facts()`.',
    settings: '`EngineBuilder` sets each setting. `record` writes a recording to a folder. `replay` answers from that recording and sends nothing, even on a miss.',
    install: [['cargo add thinkthen', null]],
    fragment: 'Put this code inside `fn main() -> Result<(), Box<dyn std::error::Error>>` and end it with `Ok(())`. `main` returns a `Result`, so `?` compiles.',
    particular: [
      'Calls block. No async runtime comes with it.',
    ],
  };
