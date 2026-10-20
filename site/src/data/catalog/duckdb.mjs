import { UNIX_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'duckdb', backends: 'sql', name: 'DuckDB', deckHeading: 'DuckDB',
    versions: { runtime: ['v1.5.6', 'v2.0-cyanoptera'], platforms: UNIX_PLATFORMS },
    lang: 'sql', tab: 'SQL',
    blurb: 'Ask a question in WHERE, SELECT, or ORDER BY.',
    unsureWord: '`NULL`',
    facts: '`thinkthen_<function>_details` returns typed rows with the value, probability, full record, failure, facts and observations.',
    errors: 'Scalar calls raise an SQL error on failure. Details calls return recoverable failures in the row’s `failure` field. Check that field before using `value` or `probability`. Cancellation, deadline and defect failures raise an SQL error. A not-sure answer has a null probability; an authored JSON null answer can have a numeric probability.',
    settings: '`SET thinkthen_record` writes a recording to a folder. `SET thinkthen_replay` answers from that recording with no connection.',
    install: [
      ['thinkthen-duckdb-VERSION-TARGET.tar.gz', 'Unsigned extensions, from each release. Unpack the archive where DuckDB runs.'],
      ['duckdb -unsigned', 'The `-unsigned` flag lets DuckDB load a local extension file. The query loads that file first.'],
    ],
    particular: ['dbt v2 requires signed extensions; this unsigned archive does not enable ThinkThen in dbt v2.', 'To load by path, choose the member under your DuckDB version and platform, then pass its path to LOAD.', 'DuckDB hands the extension rows in chunks. One call judges a chunk together, and `SET thinkthen_max_requests` caps that call.'],
  };
