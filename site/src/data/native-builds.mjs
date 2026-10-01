// How a reader builds and runs a first call on a binding over the C
// library. The install page shows these lines, and
// scripts/smoke-bindings.mjs runs the same lines, so the two cannot drift.
//
// The lines run in a folder that holds the sample, the C archive unpacked
// as thinkthen-c/, and the binding's archive unpacked as thinkthen-<slug>/.
// Each line is a list of parts. The runner joins them with spaces and
// runs the line with sh. The page puts each part on its own line.

const C_LIB = ['-L thinkthen-c/lib -lthinkthen', '-Wl,-rpath,"$PWD/thinkthen-c/lib"'];

export const NATIVE_BUILDS = {
  cpp: ({ sample, program }) => [
    ['cp thinkthen-c/include/thinkthen.h', 'thinkthen-cpp/include/thinkthen/'],
    ['c++ -std=c++17', '-I thinkthen-cpp/include', `${sample} -o ${program}`, ...C_LIB],
  ],
  'objective-c': ({ sample, program }) => [
    [
      'gcc -std=gnu11 -x objective-c',
      '-I thinkthen-c/include',
      '-I thinkthen-objective-c/Sources',
      'thinkthen-objective-c/Sources/ThinkThen.m',
      'thinkthen-objective-c/Sources/TTJSON.c',
      `${sample} -o ${program}`,
      ...C_LIB,
      '-lobjc -pthread -lm',
    ],
  ],
  cobol: ({ sample, program }) => [
    [
      'cobc -x -free',
      '-fstatic-call -fno-gen-c-decl-static-call',
      '-I thinkthen-cobol/copybooks',
      '-A "-include $PWD/thinkthen-c/include/thinkthen.h',
      '-Wno-incompatible-pointer-types',
      '-Wno-implicit-function-declaration"',
      `-o ${program} ${sample}`,
      'thinkthen-cobol/src/tt_decide.cob',
      'thinkthen-cobol/src/tt_error.cob',
      '-L thinkthen-c/lib -lthinkthen',
      '-Q "-Wl,-rpath,$PWD/thinkthen-c/lib"',
    ],
  ],
  ada: ({ sample, program }) => [
    [
      'gnatmake -gnat2022 -gnata',
      '-Ithinkthen-ada/src',
      `${sample} -o ${program}`,
      '-largs',
      ...C_LIB,
    ],
  ],
};

// The lines for one sample file, with the command that runs the program.
export function nativeBuild(slug, sample) {
  const make = NATIVE_BUILDS[slug];
  if (!make) return null;
  const program = sample.replace(/\.[^.]+$/, '');
  return { lines: make({ sample, program }), program };
}
