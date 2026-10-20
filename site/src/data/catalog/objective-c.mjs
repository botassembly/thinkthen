import { MAC_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'objective-c', name: 'Objective-C', deckHeading: null,
    versions: { runtime: 'Objective-C with ARC; Swift tools 6.0 or later', platforms: MAC_PLATFORMS },
    backends: 'code',
    lang: 'objective-c', tab: 'Objective-C',
    blurb: "Call the ten functions from Objective-C on Apple platforms, with blocks and ARC.",
    call: 'Create a `TTFoundationClient` and call `decide:input:options:feed:completion:error:`. The completion block receives the answer.',
    unsureWord: 'a null answer with null probability; a resolved authored JSON null keeps its probability',
    facts: "`TTCall` retains generated packets and typed terminal facts with explicit presence.",
    errors: "`NSError` retains native typed failures under `TTFailureDetailsKey`.",
    settings: "`initWithSettings:error:` forwards Foundation settings to Rust.",
    install: [
      ['thinkthen-objective-c-VERSION-TARGET.tar.gz', 'The Swift package `ThinkThenFoundation`, from each release. A SwiftPM project on an Apple platform depends on it and imports `ThinkThenFoundation.h`.'],
    ],
    particular: ["The package uses Apple Foundation and ARC."],
  };
