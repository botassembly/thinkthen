// Retained Rust site examples build in a normal Cargo project.
// Other language package guides own their native-bearing install commands.
// The binding replay builds the C, C++, Ada and COBOL function samples
// beside the unpacked C archive, thinkthen-c/, and the binding's folder.
const program = sample => sample.replace(/\.[^.]+$/, '');
const C_LIB = ['-L thinkthen-c/lib -lthinkthen', '-Wl,-rpath,"$PWD/thinkthen-c/lib"'];
// The Go, PHP, Dart and C# lines build a function page sample beside the
// unpacked release archive, as scripts/smoke-bindings.mjs lays it out.
const GO_MODULE = 'github.com/botassembly/thinkthen/libraries/go';
export const BUILD_LINES = {
  c: ({ sample }) => ({
    beside: 'the unpacked archive thinkthen-c/',
    lines: [[
      'cc -std=c11 -I thinkthen-c/include',
      `${sample} -o ${program(sample)}`,
      ...C_LIB,
    ]],
    run: [`./${program(sample)}`],
  }),
  // The C++ headers include <thinkthen/thinkthen.h> from the C archive.
  cpp: ({ sample }) => ({
    archive: 'thinkthen-cpp',
    lines: [
      ['cp thinkthen-c/include/thinkthen.h', 'thinkthen-cpp/include/thinkthen/'],
      ['c++ -std=c++17', '-I thinkthen-cpp/include', `${sample} -o ${program(sample)}`, ...C_LIB],
    ],
    run: [`./${program(sample)}`],
  }),
  // The Swift and Zig packages carry their own native library, so their
  // lines name no C archive. Package.swift builds main.swift as FirstCall;
  // build.zig builds first-call.zig as first-call.
  swift: () => ({
    archive: 'thinkthen-swift',
    beside: 'Package.swift and the unpacked package thinkthen-swift/',
    lines: [['swift build']],
    run: ['.build/debug/FirstCall'],
  }),
  zig: ({ sample }) => ({
    archive: 'thinkthen-zig',
    beside: 'build.zig, build.zig.zon and the unpacked package thinkthen-zig/',
    lines: [['zig build']],
    run: [`./zig-out/bin/${program(sample)}`],
  }),
  rust: ({ sample }) => ({
    beside: 'Cargo.toml',
    lines: [['mkdir -p src/bin'], [`cp ${sample} src/bin/`]],
    run: [`cargo run --quiet --bin ${program(sample)}`],
  }),
  ada: ({ sample }) => ({
    archive: 'thinkthen-ada',
    lines: [[
      'gnatmake -q -gnat2022 -gnata',
      '-Ithinkthen-ada/src',
      `${sample} -o ${program(sample)}`,
      '-largs',
      ...C_LIB,
    ]],
    run: [`./${program(sample)}`],
  }),
  cobol: ({ sample }) => ({
    archive: 'thinkthen-cobol',
    lines: [[
      'cobc -x -free',
      '-fstatic-call -fno-gen-c-decl-static-call',
      '-I thinkthen-cobol/copybooks -I thinkthen-c/include',
      '-A "-I $PWD/thinkthen-c/include',
      '-include $PWD/thinkthen-cobol/src/tt_session.h',
      '-Wno-incompatible-pointer-types"',
      `-o ${program(sample)} ${sample}`,
      'thinkthen-cobol/src/tt_session.c',
      'thinkthen-cobol/src/tt_requests_generated.c',
      '-L thinkthen-c/lib -lthinkthen',
      '-Q "-Wl,-rpath,$PWD/thinkthen-c/lib"',
    ]],
    run: [`./${program(sample)}`],
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
