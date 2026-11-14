import os from 'node:os';
import path from 'node:path';
import { menuPages, sidebar } from './src/lib/sitemap.mjs';
import errorData from './src/data/errors.json' with { type: 'json' };

const sdkLocalFailures = ['go', 'java', 'kotlin', 'python', 'typescript'].map((language) => `sdk-${language}-failure`);
const localCaptures = [...sdkLocalFailures, 'ref-json-failed', 'version', 'completions', 'man-page', 'error-backend-name', 'error-batch', 'error-choices', 'error-column', 'error-key-missing', 'error-name', 'error-output', 'error-pointer', 'error-question', 'error-record-missing', 'error-sheet', 'error-table', 'error-too-large', 'error-toolbox', 'error-url', 'exit-interrupt', 'exit-local', 'exit-usage', 'ans-field-piped', 'ans-field-piped-missing', 'ans-json-on-table'];
const localNotices = { 'exit-interrupt': 'thinkthen: cancelling; press Ctrl-C again to exit' };
const localCodes = Object.fromEntries(Object.entries(errorData.codes).flatMap(([code, entry]) => [entry.example, entry.also].filter((id) => localCaptures.includes(id) && !localNotices[id]).map((id) => [id, code])));

// Everything specific to the ThinkThen site.
export default {
  name: 'ThinkThen',
  site: 'https://thinkthen.dev',
  // Share Starlight's search ranking with the missing-page search and browser checks.
  pagefind: { ranking: { termFrequency: 0.1, pageLength: 0.1, termSaturation: 2, termSimilarity: 9 } },
  description: 'ThinkThen gives programs ten functions that judge records by meaning, from the shell, thirteen languages and three databases.',
  binary: 'thinkthen',
  binDir: process.env.SITE_BIN_DIR ?? '../target/release',
  // The command that prints the version (and the build commit, when the program reports one).
  versionArgs: ['--version'],
  // The Zig example compiles the standard library on its first run with an empty cache.
  exampleTimeoutMs: 300000,
  // Examples run with the stub backend. The runner removes every key variable.
  exampleEnv: { THINKTHEN_BACKEND: 'stub' },
  // The hash stub reaches this documented refusal; real capture still requires its published pass.
  exampleRoutineResults: { 'howto-ci-pass': { exit: 1, stdout: 'not sure: a reviewer must read the test plan\n', stderr: '' } },
  // These commands finish locally; capture checks their exit and documented error code.
  exampleLocalCaptures: localCaptures,
  exampleLocalCaptureCodes: { ...localCodes, ...Object.fromEntries(sdkLocalFailures.map((id) => [id, 'record.empty'])), 'ref-json-failed': 'record.missing', 'ans-field-piped': 'record.json', 'ans-field-piped-missing': 'record.missing', 'ans-json-on-table': 'usage.option' },
  exampleLocalCaptureNotices: localNotices,
  exampleKeylessCaptures: ['error-key-missing'],
  // These fixtures bypass the recorder or produce machine-specific endpoint diagnostics.
  exampleWithheldCaptures: ['backend-wire', 'error-reply', 'error-status', 'backend-route', 'error-transport', 'error-tls'],
  // The language examples build with the shell's toolchains, offline.
  exampleSetup: ({ shell }) => {
    const packages = path.resolve(shell.SITE_PACKAGES ?? new URL('../target/site-packages', import.meta.url).pathname);
    return {
    CARGO_HOME: shell.CARGO_HOME ?? `${os.homedir()}/.cargo`,
    RUSTUP_HOME: shell.RUSTUP_HOME ?? `${os.homedir()}/.rustup`,
    ...(shell.RUSTUP_TOOLCHAIN ? { RUSTUP_TOOLCHAIN: shell.RUSTUP_TOOLCHAIN } : {}),
    // JDK 22, Swift 6.3 and Zig 0.17.0 for the Java, Kotlin, Swift and Zig examples; see scripts/prepare-packages.sh.
    THINKTHEN_BUILD_TOOLS: shell.THINKTHEN_BUILD_TOOLS ?? `${os.homedir()}/.local/share/thinkthen-build-tools`,
    SITE_PACKAGES: packages,
    GEM_PATH: path.join(packages, 'ruby-gems'),
    ...(shell.RUBYLIB ? { RUBYLIB: shell.RUBYLIB } : {}),
    R_LIBS_USER: path.join(packages, 'r-deps'),
    ...(shell.LD_LIBRARY_PATH ? { LD_LIBRARY_PATH: shell.LD_LIBRARY_PATH } : {}),
    GOTOOLCHAIN: 'local',
    GOPROXY: 'off',
    };
  },
  // The README links to installation guides; shared site install lines live in src/data/site.json.
  readmeInstallLines: false,
  logo: { light: './src/assets/thinkthen-wordmark-light.svg', dark: './src/assets/thinkthen-wordmark-dark.svg', replacesTitle: true },
  favicon: '/thinkthen-mark-dark.svg',
  // The link-preview card; scripts/social-card.mjs renders it to public/social-card.png.
  socialCard: {
    mark: 'src/assets/thinkthen-wordmark-dark.svg',
    showName: false,
    markHeight: 120,
    background: '#0e1311',
    text: '#e6eee8',
    muted: '#8a9a90',
    accent: '#2fd27a',
    bodyFont: 'ui-sans-serif, system-ui, sans-serif',
    monoFont: "ui-monospace, 'Liberation Mono', monospace",
    tagline: 'Functions that judge text by meaning, from the shell, your programming language or your database.',
    command: "thinkthen decide 'Is this a refund request?'",
  },
  social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/botassembly/thinkthen' }],
  // The landing checks (scripts/check-landing.mjs, npm run landing): which generator writes which pages.
  // pattern matches the path under src/content/docs.
  landing: {
    generators: [
      { pattern: '^answers/', generator: 'scripts/gen-answers.mjs', template: 'answers page', source: 'the question list entry in questions/' },
      { pattern: '^reference/', generator: 'scripts/gen-reference.mjs', template: 'reference page', source: "the program's help text or schemas, written by the generator" },
    ],
  },
  // Navigation and agent reading order come from the same finite sitemap.
  spine: menuPages.map(({ slug }) => slug),
  sectionNames: { 'command-line': 'Command line', sdks: 'SDKs', 'how-tos': 'How-tos', reference: 'Reference', blog: 'Blog' },
  sidebar,
};
