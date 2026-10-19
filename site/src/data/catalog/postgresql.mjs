import { UNIX_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'postgresql', backends: 'sql', name: 'PostgreSQL', deckHeading: 'PostgreSQL',
    versions: { runtime: 'PostgreSQL 16, 17 and 18', platforms: UNIX_PLATFORMS },
    lang: 'sql', tab: 'SQL',
    blurb: 'One extension. Ask questions in any query.',
    unsureWord: '`NULL`',
    facts: '`thinkthen_usage()` gives the totals for the process.',
    errors: 'A failed plain call raises its named error. A `_details` row retains a recoverable typed failure separately from its value.',
    settings: '`SET thinkthen.record` writes a recording to a folder. `SET thinkthen.replay` answers from that recording with no connection.',
    install: [
      ['thinkthen-postgresql16-VERSION-TARGET.tar.gz', 'The extension, from each release. A server administrator copies the file in `lib/` to `pg_config --pkglibdir`. They copy the files in `extension/` to the `extension` folder under `pg_config --sharedir`.'],
      ['thinkthen-postgresql17-VERSION-TARGET.tar.gz', 'The extension, from each release. A server administrator copies the file in `lib/` to `pg_config --pkglibdir`. They copy the files in `extension/` to the `extension` folder under `pg_config --sharedir`.'],
      ['thinkthen-postgresql18-VERSION-TARGET.tar.gz', 'The extension, from each release. A server administrator copies the file in `lib/` to `pg_config --pkglibdir`. They copy the files in `extension/` to the `extension` folder under `pg_config --sharedir`.'],
      ['CREATE EXTENSION thinkthen;', null],
    ],
    particular: [
      'A question file carries a band. The not-sure rows come back NULL, and a person reads them.',
      'pg_cancel_backend and statement_timeout stop a call. A request already sent still completes and is billed.',
    ],
  };
