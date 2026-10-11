// The public repository. It sits in its own module so that catalog.mjs and
// lib/settings-table.mjs can both import it without importing each other.
export const REPO = 'https://github.com/botassembly/thinkthen';

// Package guides own executable language examples and platform evidence.
export function packageGuide(slug) {
  if (slug === 'shell') return `${REPO}/blob/main/README.md`;
  const folder = ['java', 'kotlin', 'scala'].includes(slug) ? 'jvm'
    : ['pandas', 'polars'].includes(slug) ? 'python' : slug;
  const parent = ['duckdb', 'sqlite', 'postgresql'].includes(slug) ? 'databases' : 'libraries';
  return `${REPO}/blob/main/${parent}/${folder}/README.md`;
}

// These callers run through the package checks; the site keeps no second example.
export function packageExample(slug) {
  const examples = {
    shell: 'demos/01-refund-gate/demo.sh',
    python: 'libraries/python/tests/test_named_calls.py',
    pandas: 'libraries/python/tests/test_named_calls.py',
    polars: 'libraries/python/tests/test_named_calls.py',
    typescript: 'libraries/typescript/tests/owned_session.test.mjs',
    ruby: 'libraries/ruby/tests/test_owned_session.rb',
    r: 'libraries/r/examples/slide_check.R',
    rust: 'libraries/rust/consumer/src/main.rs',
    c: 'libraries/c/examples/session.c',
    cpp: 'libraries/cpp/fixtures/session_consumer.cpp',
    csharp: 'libraries/csharp/examples/Program.cs',
    go: 'libraries/go/examples/decide/main.go',
    java: 'libraries/jvm/tests/SessionConsumer.java',
    kotlin: 'libraries/jvm/tests/SessionKotlin.kt',
    scala: 'libraries/jvm/tests/SessionScala.scala',
    swift: 'libraries/swift/Examples/main.swift',
    zig: 'libraries/zig/examples/decide.zig',
    php: 'libraries/php/fixtures/session_consumer.php',
    dart: 'libraries/dart/checks/session_consumer/bin/main.dart',
    ada: 'libraries/ada/examples/session_demo.adb',
    'objective-c': 'libraries/objective-c/checks/foundation_installed.py',
    cobol: 'libraries/cobol/examples/session.cob',
    duckdb: 'databases/duckdb/README.md',
    sqlite: 'databases/sqlite/README.md',
    postgresql: 'databases/postgresql/README.md',
  };
  if (!examples[slug]) throw new Error(`No package example for ${slug}`);
  return `${REPO}/blob/main/${examples[slug]}`;
}
