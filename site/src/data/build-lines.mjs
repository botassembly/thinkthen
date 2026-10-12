// Retained Rust site examples build in a normal Cargo project.
// Other language package guides own their native-bearing install commands.
// The binding replay builds the Ada and COBOL function samples beside the
// unpacked C archive, thinkthen-c/, and the binding's own folder.
const program = sample => sample.replace(/\.[^.]+$/, '');
const C_LIB = ['-L thinkthen-c/lib -lthinkthen', '-Wl,-rpath,"$PWD/thinkthen-c/lib"'];
export const BUILD_LINES = {
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
};

export function buildLines(slug, sample, offline = false) {
  const make = BUILD_LINES[slug];
  return make ? make({sample, offline}) : null;
}
