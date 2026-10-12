// Retained Rust site examples build in a normal Cargo project.
// Other language package guides own their native-bearing install commands.
// The Go, PHP, Dart and C# lines build a function page sample beside the
// unpacked release archive, as scripts/smoke-bindings.mjs lays it out.
const program = sample => sample.replace(/\.[^.]+$/, '');
const GO_MODULE = 'github.com/botassembly/thinkthen/libraries/go';
export const BUILD_LINES = {
  rust: ({ sample }) => ({
    beside: 'Cargo.toml',
    lines: [['mkdir -p src/bin'], [`cp ${sample} src/bin/`]],
    run: [`cargo run --quiet --bin ${program(sample)}`],
  }),
  go: ({ sample }) => ({
    archive: 'thinkthen-go',
    lines: [
      [`go mod init ${program(sample)}`],
      ['go mod edit', `-replace=${GO_MODULE}=./thinkthen-go`],
      ['go mod tidy'],
      [`go build -o ${program(sample)} .`],
    ],
    run: [`./${program(sample)}`],
  }),
  php: ({ sample }) => ({
    archive: 'thinkthen-php',
    lines: [],
    run: ['php -d ffi.enable=1', '-d zend.assertions=1', sample],
  }),
  dart: ({ sample, offline }) => ({
    archive: 'thinkthen-dart',
    lines: [[offline ? 'dart pub get --offline' : 'dart pub get']],
    run: ['dart run --enable-asserts', sample],
  }),
  csharp: () => ({
    archive: 'thinkthen-csharp',
    beside: 'first-call.csproj and the unpacked archive thinkthen-csharp/',
    lines: [['dotnet build -o bin', '--source "$PWD/thinkthen-csharp"']],
    run: ['dotnet bin/first-call.dll'],
  }),
};

export function buildLines(slug, sample, offline = false) {
  const make = BUILD_LINES[slug];
  return make ? make({sample, offline}) : null;
}
