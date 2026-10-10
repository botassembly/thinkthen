// Retained Rust site examples build in a normal Cargo project.
// Other language package guides own their native-bearing install commands.
const program = sample => sample.replace(/\.[^.]+$/, '');
export const BUILD_LINES = {
  rust: ({ sample }) => ({
    beside: 'Cargo.toml',
    lines: [['mkdir -p src/bin'], [`cp ${sample} src/bin/`]],
    run: [`cargo run --quiet --bin ${program(sample)}`],
  }),
};

export function buildLines(slug, sample, offline = false) {
  const make = BUILD_LINES[slug];
  return make ? make({sample, offline}) : null;
}
