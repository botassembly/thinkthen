import { NATIVE_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'c', name: 'C', deckHeading: 'C',
    versions: { runtime: 'C11', platforms: NATIVE_PLATFORMS },
    backends: 'code',
    lang: 'c', tab: 'Rust',
    blurb: "Call the ten functions from C through one header and one library.",
    call: 'Write the request as JSON, start it with `thinkthen_session_new`, and read the answers the session returns.',
    unsureWord: 'a present null generated answer; typed failures remain separate',
    facts: "Generated complete session views retain presence and terminal facts.",
    errors: "Admission diagnostics remain thread-local; terminal packets retain native typed failures.",
    settings: "`thinkthen_engine_new_with` forwards native settings to Rust.",
    install: [['thinkthen.h + libthinkthen', 'Each release ships the header, the shared library, and the static library.']],
    fragment: 'Keep the `#include` lines on top. Put the rest inside `int main(void)` and end it with `return 0;`.',
    particular: ["New callers use generated native sessions.", "The C package owns the matching generated header and native library."],
  };
