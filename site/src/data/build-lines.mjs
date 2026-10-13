// Retained Rust site examples build in a normal Cargo project.
// Other language package guides own their native-bearing install commands.
// The binding replay builds the C, C++, Objective-C, Ada and COBOL function samples
// beside the unpacked C archive, thinkthen-c/, and the binding's folder.
const program = sample => sample.replace(/\.[^.]+$/, '');
const C_LIB = ['-L thinkthen-c/lib -lthinkthen', '-Wl,-rpath,"$PWD/thinkthen-c/lib"'];
const OBJC = 'thinkthen-objective-c/Sources/Foundation';
// The Go, PHP, Dart and C# lines build a function page sample beside the
// unpacked release archive, as scripts/smoke-bindings.mjs lays it out.
const GO_MODULE = 'github.com/botassembly/thinkthen/libraries/go';
// The Java, Kotlin and Scala lines build a function page sample beside the
// unpacked JVM archive, thinkthen-jvm/, which holds the session, Kotlin
// and Scala JARs and the native JAR for this platform. They need JDK 22
// or later, and the Kotlin and Scala lines need KOTLIN_HOME and
// SCALA_HOME for the coroutine and Scala runtime JARs.
const JVM = 'thinkthen-jvm';
const NATIVE = `$(echo ${JVM}/thinkthen-natives-*.jar)`;
const JAVA = 'java -ea --enable-native-access=ALL-UNNAMED';
const KOTLIN_CP = [
  `${JVM}/thinkthen-kotlin.jar`,
  '$KOTLIN_HOME/lib/kotlinx-coroutines-core-jvm.jar',
].join(':');
const KOTLIN_RUN = [KOTLIN_CP, NATIVE, '$KOTLIN_HOME/lib/kotlin-stdlib.jar'].join(':');
const SCALA_CP = `${JVM}/thinkthen-scala.jar`;
const SCALA_RUN = [SCALA_CP, NATIVE, '$SCALA_HOME/lib/scala.jar'].join(':');
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
  // Objective-C builds the Foundation sources from the binding's folder
  // with the sample, on macOS.
  'objective-c': ({ sample }) => ({
    archive: 'thinkthen-objective-c',
    lines: [[
      'clang -fobjc-arc -fblocks',
      `-I ${OBJC} -I thinkthen-c/include`,
      `${OBJC}/*.m ${sample} -o ${program(sample)}`,
      '-framework Foundation',
      ...C_LIB,
    ]],
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
  java: ({ sample }) => ({
    archive: JVM,
    lines: [[
      'javac --release 22',
      `-cp ${JVM}/thinkthen-session.jar`,
      `-d classes ${sample}`,
    ]],
    run: [JAVA, `-cp "classes:${JVM}/thinkthen-session.jar:${NATIVE}"`, program(sample)],
  }),
  kotlin: ({ sample }) => ({
    archive: JVM,
    lines: [[
      'kotlinc -jvm-target 22',
      `-classpath "${KOTLIN_CP}"`,
      `${sample} -d classes`,
    ]],
    run: [JAVA, `-cp "classes:${KOTLIN_RUN}"`, `${program(sample)}Kt`],
  }),
  scala: ({ sample }) => ({
    archive: JVM,
    lines: [
      ['mkdir -p classes'],
      ['scalac', `-classpath ${SCALA_CP}`, `-d classes ${sample}`],
    ],
    run: [JAVA, `-cp "classes:${SCALA_RUN}"`, program(sample).toLowerCase()],
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
