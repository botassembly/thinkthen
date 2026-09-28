// Read the examples under examples/. Every command and sample on a page
// comes from a file there, and scripts/smoke.mjs runs each script against
// its saved output. Nothing on a page is typed twice.
//
// examples/<page>/<name>.sh is a script. <name>.out holds what it printed,
// and <name>.exit its exit code when that is not 0. examples/<page>/files/
// holds the files the scripts read. A library sample sits beside the
// scripts as <surface>.<ext>, and a first-call sample as
// examples/install/<surface>/first-call.<ext>.

const raw = import.meta.glob(
  ['/examples/**/*', '!/examples/beatles/bench/**'],
  { query: '?raw', import: 'default', eager: true },
);

const text = (p) => raw[`/examples/${p}`];

// The question files a Beatles Bench page reads, from the bench copy. Only
// the JSON files at the top of each bench example folder load.
const benchFiles = import.meta.glob(
  '/examples/beatles/bench/examples/*/*.json',
  { query: '?raw', import: 'default', eager: true },
);

// One file from the bench folder a Beatles page runs in.
export function benchFile(folder, name) {
  const found = benchFiles[`/examples/beatles/bench/${folder}/${name}`];
  if (found === undefined) throw new Error(`samples: the bench has no ${folder}/${name}`);
  return { name, text: found.replace(/\n+$/, '') };
}

// The scripts of one page, in name order, each with its output and exit.
// A page with no script fails the build unless `optional` is set.
export function runs(page, optional = false) {
  const prefix = `/examples/${page}/`;
  const names = Object.keys(raw)
    .filter((k) => k.startsWith(prefix) && k.endsWith('.sh') && !k.slice(prefix.length).includes('/'))
    .map((k) => k.slice(prefix.length, -3))
    .sort();
  if (!names.length && !optional) throw new Error(`samples: examples/${page} holds no script`);
  return names.map((name) => {
    const output = text(`${page}/${name}.out`);
    if (output === undefined) throw new Error(`samples: examples/${page}/${name}.out is missing. Run node scripts/smoke.mjs --update ${page}/`);
    const exit = text(`${page}/${name}.exit`);
    return {
      name,
      command: text(`${page}/${name}.sh`).replace(/\n+$/, ''),
      output: output.replace(/\n+$/, ''),
      exit: exit === undefined ? 0 : Number(exit.trim()),
    };
  });
}

// One script of a page, by name.
export function run(page, name) {
  const found = runs(page).find((r) => r.name === name);
  if (!found) throw new Error(`samples: examples/${page} has no ${name}.sh`);
  return found;
}

// The files a page's scripts read, with their paths inside files/.
export function files(page) {
  const prefix = `/examples/${page}/files/`;
  return Object.keys(raw)
    .filter((k) => k.startsWith(prefix) && !/^(recording|proposed)\//.test(k.slice(prefix.length)))
    .sort()
    .map((k) => ({ name: k.slice(prefix.length), text: raw[k].replace(/\n+$/, '') }));
}

// Captions live in the page data, keyed by script name. Every script needs
// one, and every caption needs a script.
export function captioned(page, see, optional = false) {
  const list = runs(page, optional);
  for (const r of list) {
    if (!see?.[r.name]) throw new Error(`samples: examples/${page}/${r.name}.sh has no caption`);
  }
  for (const name of Object.keys(see || {})) {
    if (!list.some((r) => r.name === name)) throw new Error(`samples: a caption names ${page}/${name}, and no script has that name`);
  }
  return list.map((r) => ({ ...r, see: see[r.name] }));
}

const EXT = {
  python: 'py', polars: 'py', typescript: 'ts', ruby: 'rb', r: 'R',
  rust: 'rs', c: 'c', duckdb: 'sql', sqlite: 'sql', postgresql: 'sql',
};

// The first-call sample for a surface, with the output a database printed.
export function firstCall(surface) {
  const ext = EXT[surface];
  const code = text(`install/${surface}/first-call.${ext}`);
  if (code === undefined) throw new Error(`samples: examples/install/${surface}/first-call.${ext} is missing`);
  return { code: code.replace(/\n+$/, ''), output: text(`install/${surface}/first-call.${ext}.out`)?.replace(/\n+$/, '') ?? null };
}

// The library sample for a function on a surface, with the output a
// database printed. null when no sample exists.
export function sample(fn, surface) {
  const ext = EXT[surface];
  const own = text(`functions/${fn}/${surface}.${ext}`);
  if (own === undefined) return null;
  return { code: own.replace(/\n+$/, ''), output: text(`functions/${fn}/${surface}.${ext}.out`)?.replace(/\n+$/, '') ?? null };
}
