// How a reader builds and runs a first call on a binding that needs a
// build step. The install page shows these lines, and
// scripts/smoke-bindings.mjs runs the same lines, so the two cannot drift.
//
// The lines run in a folder that holds the sample, the C archive unpacked
// as thinkthen-c/, and the binding's archive unpacked beside it under the
// name `archive` gives. `beside` names what the folder holds when that
// differs: C needs only its own archive, and Rust needs only Cargo.toml. Each line is a list of parts. The runner joins
// them with spaces and runs the line with sh. The page puts each part on
// its own line. The runner passes `offline`, so a package tool reads only
// its local cache and the run sends no request.

const C_LIB = ['-L thinkthen-c/lib -lthinkthen', '-Wl,-rpath,"$PWD/thinkthen-c/lib"'];
const JAVA = [
  'java --enable-preview -ea',
  '--enable-native-access=ALL-UNNAMED',
  '-Dthinkthen.library="$PWD/thinkthen-c/lib/libthinkthen.so"',
];
const DOOR = 'thinkthen-jvm/thinkthen-door.jar';
const program = (sample) => sample.replace(/\.[^.]+$/, '');

export const BUILD_LINES = {
  c: ({ sample }) => ({
    beside: 'the unpacked archive thinkthen-c/',
    lines: [[
      'cc -std=c11 -I thinkthen-c/include',
      `${sample} -o ${program(sample)}`,
      '$(pkg-config --cflags --libs json-c)',
      ...C_LIB,
    ]],
    run: [`./${program(sample)}`],
  }),
  rust: () => ({
    beside: 'Cargo.toml',
    lines: [],
    run: ['cargo run --quiet'],
  }),
  cpp: ({ sample }) => ({
    archive: 'thinkthen-cpp',
    lines: [
      ['cp thinkthen-c/include/thinkthen.h', 'thinkthen-cpp/include/thinkthen/'],
      ['c++ -std=c++17', '-I thinkthen-cpp/include', `${sample} -o ${program(sample)}`, ...C_LIB],
    ],
    run: [`./${program(sample)}`],
  }),
  'objective-c': ({ sample }) => ({
    archive: 'thinkthen-objective-c',
    lines: [[
      'gcc -std=gnu11 -x objective-c',
      '-I thinkthen-c/include',
      '-I thinkthen-objective-c/Sources',
      'thinkthen-objective-c/Sources/ThinkThen.m',
      'thinkthen-objective-c/Sources/TTJSON.c',
      `${sample} -o ${program(sample)}`,
      ...C_LIB,
      '-lobjc -pthread -lm',
    ]],
    run: [`./${program(sample)}`],
  }),
  cobol: ({ sample }) => ({
    archive: 'thinkthen-cobol',
    lines: [[
      'cobc -x -free',
      '-fstatic-call -fno-gen-c-decl-static-call',
      '-I thinkthen-cobol/copybooks',
      '-A "-include $PWD/thinkthen-c/include/thinkthen.h',
      '-Wno-incompatible-pointer-types',
      '-Wno-implicit-function-declaration"',
      `-o ${program(sample)} ${sample}`,
      'thinkthen-cobol/src/tt_decide.cob',
      'thinkthen-cobol/src/tt_error.cob',
      '-L thinkthen-c/lib -lthinkthen',
      '-Q "-Wl,-rpath,$PWD/thinkthen-c/lib"',
    ]],
    run: [`./${program(sample)}`],
  }),
  ada: ({ sample }) => ({
    archive: 'thinkthen-ada',
    lines: [[
      'gnatmake -gnat2022 -gnata',
      '-Ithinkthen-ada/src',
      `${sample} -o ${program(sample)}`,
      '-largs',
      ...C_LIB,
    ]],
    run: [`./${program(sample)}`],
  }),
  java: ({ sample }) => ({
    archive: 'thinkthen-jvm',
    lines: [['javac --enable-preview --release 21', `-cp ${DOOR}`, sample]],
    run: [...JAVA, `-cp ${DOOR}:.`, program(sample)],
  }),
  kotlin: ({ sample }) => ({
    archive: 'thinkthen-jvm',
    lines: [[
      'kotlinc -jvm-target 21',
      `-cp ${DOOR}:thinkthen-jvm/thinkthen-kotlin.jar`,
      `${sample} -include-runtime -d ${program(sample)}.jar`,
    ]],
    run: [
      ...JAVA,
      `-cp ${DOOR}:thinkthen-jvm/thinkthen-kotlin.jar:${program(sample)}.jar`,
      `${program(sample)}Kt`,
    ],
  }),
  scala: ({ sample }) => ({
    archive: 'thinkthen-jvm',
    lines: [[
      'scalac',
      `-cp ${DOOR}:thinkthen-jvm/thinkthen-scala.jar`,
      `-d ${program(sample)}.jar ${sample}`,
    ]],
    run: [
      ...JAVA,
      `-cp ${DOOR}:thinkthen-jvm/thinkthen-scala.jar:${program(sample)}.jar:"$SCALA_HOME/lib/scala.jar"`,
      'firstCall',
    ],
  }),
  go: ({ sample }) => ({
    archive: 'thinkthen-go',
    lines: [
      ['go mod init first-call'],
      ['go mod edit', '-replace=github.com/botassembly/thinkthen/libraries/go=./thinkthen-go'],
      ['go mod tidy'],
      ['PKG_CONFIG_PATH="$PWD/thinkthen-c/lib/pkgconfig"', `go build -o ${program(sample)} .`],
    ],
    run: ['LD_LIBRARY_PATH="$PWD/thinkthen-c/lib"', `./${program(sample)}`],
  }),
  swift: () => ({
    archive: 'thinkthen-swift',
    lines: [[
      'swift build',
      '-Xlinker -L -Xlinker "$PWD/thinkthen-c/lib"',
      '-Xlinker -rpath -Xlinker "$PWD/thinkthen-c/lib"',
    ]],
    run: ['.build/debug/FirstCall'],
  }),
  zig: () => ({
    archive: 'thinkthen-zig',
    lines: [['zig build', '-Dnative="$PWD/thinkthen-c"']],
    run: ['./zig-out/bin/first-call'],
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
    lines: [['dotnet build -o bin', '--source "$PWD/thinkthen-csharp"']],
    run: ['LD_LIBRARY_PATH="$PWD/thinkthen-c/lib"', 'dotnet bin/first-call.dll'],
  }),
};

// The build and run lines for one sample file.
export function buildLines(slug, sample, offline = false) {
  const make = BUILD_LINES[slug];
  return make ? make({ sample, offline }) : null;
}
