import { UNIX_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'sqlite', backends: 'sql', name: 'SQLite', deckHeading: 'SQLite',
    versions: { runtime: 'SQLite 3.50.0 or later', platforms: UNIX_PLATFORMS },
    lang: 'sql', tab: 'SQL',
    blurb: 'Judge a whole table in one call, then join the answers back by key.',
    unsureWord: '`NULL`',
    facts: '`thinkthen_usage()` gives the totals for the process.',
    errors: 'A failure is an error whose text starts `thinkthen <kind>: `. The message follows, and the text ends with `(retryable: yes)` or `(retryable: no)`.',
    settings: '`thinkthen_configure` takes the settings as JSON. `"record"` writes a recording to a folder. `"replay"` answers from that recording with no connection.',
    install: [
      ['thinkthen-sqlite-VERSION-TARGET.tar.gz', 'The extension, from each release. Rename `libthinkthen0.so` to `thinkthen.so`, or `libthinkthen0.dylib` to `thinkthen.dylib` on a Mac.'],
      ['.load ./thinkthen', null],
    ],
    particular: [
      'Normal loading registers every ThinkThen function for use in triggers, views, CHECK constraints and column defaults.',
    ],
  };
