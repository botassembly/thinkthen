# The site gives commands, libraries, and databases a status

Status: Open. Ian's ruling, 2026-09-24, filed by the marketing session.

## The ruling

Ian, 2026-09-24: "We're not doing the marketing and not publishing the website until everything's done." Every command, library, and database extension ships together, and Polars too. No copy says one thing is done and another is not. That rules out preview, planned, beta, "at launch", "ships first", "not run yet", "not yet shipped", "after" a release, and any staged rollout.

A plain version number is fine. Ian: "We can say the words 0.1. I'm not afraid of saying that that's the version when we publish. It's just that I hate all this hedging." "ThinkThen 0.1" can stay. A badge that gives Bash "ships first" and every other surface "comes with 0.1" cannot, because it tells one surface from another.

## Lines to change

Read on thinkthen main `544ffe44` by a read-only grep of `site/`. Each line gives a surface or a function a status, or it builds one. The fix drops the status and states the page as a finished product. Where a line exists only to stage something, it goes.

### `site/README.md`

- `site/README.md:43`: `` | `planned` | Nothing is written for this cell yet. | The page says so in place of code. | ``
- `site/README.md:92`: `…e reads Ruby, falling back to Bash where nothing is written yet. Plain JavaScript, no framework on the client.…`
- `site/README.md:94`: `## What is not here yet`

### `site/src/pages/install.astro`

- `site/src/pages/install.astro:17`: `…nkThen, the command that answers questions about text. Bash ships first. Every library below lists the line it will ship under.</p…`
- `site/src/pages/install.astro:26`: `<tr><td>{channel}</td><td><code>{line}</code></td><td><span class="status planned">comes with 0.1</span></td></tr>`
- `site/src/pages/install.astro:35`: `<p>Each page shows how the calls will read.</p>`
- `site/src/pages/install.astro:42`: `` <td><span class="status planned">{s.release ? `comes with ${s.release}` : 'planned'}</span></td> ``

### `site/src/pages/index.astro`

- `site/src/pages/index.astro:21`: `const plannedCaption = (slug) => surfaceOf(slug).name + ' comes with 0.1. The calls will read like this.';`
- `site/src/pages/index.astro:45`: `` …stallLine line={surfaceOf(t.surface).install[0][0]} status="planned" label={surfaceOf(t.surface).release ? `comes with ${surfaceOf… ``
- `site/src/pages/index.astro:55`: `<b>{fn.name}{fn.status === 'preview' && <> <span class="status preview">preview</span></>}</b>`

### `site/src/pages/[surface]/index.astro`

- `site/src/pages/[surface]/index.astro:15`: `const shipped = surface.status === 'ships first';`
- `site/src/pages/[surface]/index.astro:18`: `const absent = (s) => s.data.status === 'planned' || s.data.status === 'preview';`
- `site/src/pages/[surface]/index.astro:25`: `` ? `${surface.name} comes with ${surface.release}.` ``
- `site/src/pages/[surface]/index.astro:26`: `` : `${surface.name} is planned.`; ``
- `site/src/pages/[surface]/index.astro:33`: `? 'The first call below ran on the branch build, and its output is under it. Bash ships first.'`
- `site/src/pages/[surface]/index.astro:34`: `` : `The code below shows how the calls will read.${surface.release ? ' None of it has run yet. Bash ships first.' : ''}`}</div> ``
- `site/src/pages/[surface]/index.astro:41`: `` <tr><td><code>{line}</code></td><td>{note || (shipped ? '' : surface.release ? `Comes with ${surface.release}.` : 'Planned.')}</td></tr> ``
- `site/src/pages/[surface]/index.astro:51`: `` <Code code={sample.code} caption={printed ? `${when} The calls read like this.` : `${when} The calls will read like this.`} /> ``
- `site/src/pages/[surface]/index.astro:52`: `` …: printed.output, exit: null }} caption={`Recorded from the branch build. ${printed.see}`} word={false} />}… ``
- `site/src/pages/[surface]/index.astro:60`: `{carried.length === 0 && <p>No {surface.name} sample yet.</p>}`
- `site/src/pages/[surface]/index.astro:73`: `<h3>No sample yet</h3>`
- `site/src/pages/[surface]/index.astro:78`: `? 'Each is a preview. Its Bash sample lands here once it is recorded.'`
- `site/src/pages/[surface]/index.astro:79`: `: 'Each works as it does in Bash. Its sample lands here when the library ships.'}`

### `site/src/pages/functions/index.astro`

- `site/src/pages/functions/index.astro:18`: `` <td><a href={`/functions/${fn.name}/`}><code>{fn.name}</code></a>{fn.status === 'preview' && <> <span class="status preview">preview</span></>}</td> ``

### `site/src/data/examples.mjs`

- `site/src/data/examples.mjs:15`: `const STATUSES = new Set(['run', 'drawn', 'planned', 'preview']);`

### `site/src/styles/site.css`

- `site/src/styles/site.css:144`: `.status.preview { color: var(--think); }`
- `site/src/styles/site.css:145`: `.status.planned { color: var(--broken); }`

### `site/src/articles/code-that-understands.md`

- `site/src/articles/code-that-understands.md:173`: `## What ships and what has not shipped`
- `site/src/articles/code-that-understands.md:175`: `` …tag`, `score`, `filter`, `rank`, `find`, and `annotate` are shipped. `recognize` and `relate` are built and not yet shipped. Relat… ``
- `site/src/articles/code-that-understands.md:177`: `… Python, TypeScript, Ruby, R, Rust, and C are drawn and not shipped. C opens the door to every other language. A Polars column is …`

### `site/src/pages/tutorial.astro`

- `site/src/pages/tutorial.astro:39`: `<InstallLine line={shell.install[0][0]} status="planned" label="comes with 0.1" note={shell.install[0][1]} caption="Homebrew tap" />`
- `site/src/pages/tutorial.astro:40`: `<InstallLine line={shell.install[1][0]} status="planned" label="comes with 0.1" note={shell.install[1][1]} caption="Download script" />`

### `site/src/pages/surfaces.astro`

- `site/src/pages/surfaces.astro:7`: `…from Bash, six languages, Polars, and three databases. Bash ships first. Python, TypeScript, Ruby, R, Rust, C, DuckDB, SQLite, and…`
- `site/src/pages/surfaces.astro:16`: `` <td><span class={`status ${statusClass(s.status)}`}>{s.release === '0.1' ? 'comes with 0.1' : s.status}</span></td> ``
- `site/src/pages/surfaces.astro:21`: `<h2>Not here yet</h2>`

### `site/src/data/catalog.mjs`

- `site/src/data/catalog.mjs:220`: `requests: 'The specification does not yet carry recognize, so the request count is not settled.',`
- `site/src/data/catalog.mjs:221`: `args: 'Not settled. The command has no recognize yet.',`
- `site/src/data/catalog.mjs:222`: `status: 'preview',`
- `site/src/data/catalog.mjs:225`: `…ute it, and it claims nothing about chance. Relations are a preview too.',…`
- `site/src/data/catalog.mjs:235`: `requests: 'The specification does not yet carry relate, so the request count is not settled.',`
- `site/src/data/catalog.mjs:236`: `args: 'Not settled. The command has no relate yet.',`
- `site/src/data/catalog.mjs:237`: `status: 'preview',`
- `site/src/data/catalog.mjs:240`: `unsure: 'A relation has a direction, or it is marked as reading the same both ways. The number on an edge is a probability. relate is a preview.',`
- `site/src/data/catalog.mjs:295`: `// One status vocabulary for every badge: planned, comes with 0.1, ships`
- `site/src/data/catalog.mjs:296`: `// first, not run yet, preview. An example cell may still say "drawn" in data.`
- `site/src/data/catalog.mjs:297`: `// A surface's release names the version it comes with; Polars has none yet.`
- `site/src/data/catalog.mjs:303`: `slug: 'shell', name: 'Bash', deckHeading: null, status: 'ships first', release: 'ships first',`
- `site/src/data/catalog.mjs:319`: `slug: 'python', name: 'Python', deckHeading: 'Python', status: 'planned', release: '0.1',`
- `site/src/data/catalog.mjs:330`: `slug: 'polars', name: 'Polars', deckHeading: 'Polars', status: 'planned', release: null,`
- `site/src/data/catalog.mjs:341`: `slug: 'typescript', name: 'TypeScript', deckHeading: 'TypeScript', status: 'planned', release: '0.1',`
- `site/src/data/catalog.mjs:352`: `slug: 'ruby', name: 'Ruby', deckHeading: 'Ruby', status: 'planned', release: '0.1',`
- `site/src/data/catalog.mjs:360`: `slug: 'r', name: 'R', deckHeading: 'R', status: 'planned', release: '0.1',`
- `site/src/data/catalog.mjs:371`: `slug: 'rust', name: 'Rust', deckHeading: 'Rust', status: 'planned', release: '0.1',`
- `site/src/data/catalog.mjs:382`: `slug: 'c', name: 'C', deckHeading: 'C', status: 'planned', release: '0.1',`
- `site/src/data/catalog.mjs:393`: `slug: 'duckdb', name: 'DuckDB', deckHeading: 'DuckDB', status: 'planned', release: '0.1',`
- `site/src/data/catalog.mjs:401`: `slug: 'sqlite', name: 'SQLite', deckHeading: 'SQLite', status: 'planned', release: '0.1',`
- `site/src/data/catalog.mjs:412`: `slug: 'postgresql', name: 'PostgreSQL', deckHeading: 'PostgreSQL', status: 'planned', release: '0.1',`

### `site/scripts/pull-examples.mjs`

- `site/scripts/pull-examples.mjs:7`: `// A surface sample the branch build ran carries what it printed, from the`
- `site/scripts/pull-examples.mjs:9`: `// Every function-and-surface cell gets a status: run, drawn, planned, or`
- `site/scripts/pull-examples.mjs:10`: `// preview. A missing cell of recognize or relate says preview.`
- `site/scripts/pull-examples.mjs:281`: `// What the sample printed when the branch build ran it: the command, the`
- `site/scripts/pull-examples.mjs:290`: `// The sample the branch build ran fills its functions first, whole and`
- `site/scripts/pull-examples.mjs:487`: `: { status: 'planned', source: 'the command has no such function yet' };`
- `site/scripts/pull-examples.mjs:498`: `cell = { status: 'planned', source: 'no example drawn for this surface yet' };`
- `site/scripts/pull-examples.mjs:500`: `// A preview function's missing cell says preview, the one word for it.`
- `site/scripts/pull-examples.mjs:501`: `if (fn.status === 'preview' && cell.status === 'planned') cell.status = 'preview';`

### `site/src/components/InstallLine.astro`

- `site/src/components/InstallLine.astro:4`: `// "comes with 0.1" when the release is named, "planned" when it is not.`
- `site/src/components/InstallLine.astro:5`: `const { line, status = 'planned', label = status, note = null, caption = 'Install' } = Astro.props;`

### `site/src/pages/functions/[name]/index.astro`

- `site/src/pages/functions/[name]/index.astro:19`: `const preview = fn.status === 'preview';`
- `site/src/pages/functions/[name]/index.astro:20`: `// A library with no release named is planned. Polars is the one today.`
- `site/src/pages/functions/[name]/index.astro:23`: `` ? `${unnamed.join(', ')} ${unnamed.length === 1 ? 'is' : 'are'} planned. The other libraries come with 0.1.` ``
- `site/src/pages/functions/[name]/index.astro:25`: `const shipped = !preview && !fn.notAFunction;`
- `site/src/pages/functions/[name]/index.astro:38`: `{preview && (`
- `site/src/pages/functions/[name]/index.astro:39`: `<div class="note"><span class="status preview">preview</span> The command has no <code>{fn.name}</code> yet. The samples below sho…`
- `site/src/pages/functions/[name]/index.astro:62`: `<p>{!preview && 'The Bash command works today. '}{libraries} Their samples show how the calls will read.</p>`
- `site/src/pages/functions/[name]/index.astro:67`: `<p>The specification does not carry <code>{fn.name}</code> yet, so its options are not settled.</p>`
- `site/src/pages/functions/[name]/index.astro:93`: `<p>No how-to uses <code>{title}</code> yet.</p>`

### `site/src/components/Code.astro`

- `site/src/components/Code.astro:2`: `// A drawn code sample. Its badge says "not run yet" wherever it appears,`
- `site/src/components/Code.astro:3`: `// because no library ships yet. "drawn" stays in the data and the class.`
- `site/src/components/Code.astro:5`: `const label = status === 'drawn' ? 'not run yet' : status;`

### `site/src/components/CrossView.astro`

- `site/src/components/CrossView.astro:3`: `// function yet shows its status in place of code.`
- `site/src/components/CrossView.astro:37`: `` …ta.code} caption={`${t.surface.name} ${t.surface.release ? `comes with ${t.surface.release}` : 'is planned'}. The call will read l… ``
- `site/src/components/CrossView.astro:41`: `` …ta.code} caption={`${t.surface.name} ${t.surface.release ? `comes with ${t.surface.release}` : 'is planned'}. The calls read like … ``
- `site/src/components/CrossView.astro:42`: `` …a.printed.output, exit: null }} caption={`Recorded from the branch build. ${t.data.printed.see}`} word={false} />… ``
- `site/src/components/CrossView.astro:45`: `{(t.data.status === 'planned' || t.data.status === 'preview') && (`
- `site/src/components/CrossView.astro:48`: `No <code>{fn}</code> example for {t.surface.name} yet.`

### `site/scripts/emit-md.mjs`

- `site/scripts/emit-md.mjs:210`: `'Bash ships first. Every library page shows how the calls will read. None of that code has run yet.',`

### Example cells in `site/src/data/examples/`

These 28 cells carry `"status": "planned"` or `"status": "preview"`. `site/scripts/pull-examples.mjs:487`, `:498`, and `:501` write them.

- `site/src/data/examples/choose__polars.json:2`: `"status": "planned"`
- `site/src/data/examples/filter__polars.json:2`: `"status": "planned"`
- `site/src/data/examples/find__duckdb.json:2`: `"status": "planned"`
- `site/src/data/examples/find__polars.json:2`: `"status": "planned"`
- `site/src/data/examples/find__postgresql.json:2`: `"status": "planned"`
- `site/src/data/examples/find__sqlite.json:2`: `"status": "planned"`
- `site/src/data/examples/_index.json:125`: `"status": "planned"`
- `site/src/data/examples/_index.json:235`: `"status": "planned"`
- `site/src/data/examples/_index.json:290`: `"status": "planned"`
- `site/src/data/examples/_index.json:325`: `"status": "planned"`
- `site/src/data/examples/_index.json:345`: `"status": "planned"`
- `site/src/data/examples/_index.json:375`: `"status": "planned"`
- `site/src/data/examples/_index.json:380`: `"status": "planned"`
- `site/src/data/examples/_index.json:385`: `"status": "planned"`
- `site/src/data/examples/_index.json:445`: `"status": "preview"`
- `site/src/data/examples/_index.json:455`: `"status": "preview"`
- `site/src/data/examples/_index.json:500`: `"status": "preview"`
- `site/src/data/examples/_index.json:510`: `"status": "preview"`
- `site/src/data/examples/_index.json:590`: `"status": "planned"`
- `site/src/data/examples/_index.json:70`: `"status": "planned"`
- `site/src/data/examples/question-file__c.json:2`: `"status": "planned"`
- `site/src/data/examples/rank__polars.json:2`: `"status": "planned"`
- `site/src/data/examples/rank__sqlite.json:2`: `"status": "planned"`
- `site/src/data/examples/recognize__polars.json:2`: `"status": "preview"`
- `site/src/data/examples/recognize__shell.json:2`: `"status": "preview"`
- `site/src/data/examples/relate__polars.json:2`: `"status": "preview"`
- `site/src/data/examples/relate__shell.json:2`: `"status": "preview"`
- `site/src/data/examples/tag__polars.json:2`: `"status": "planned"`

Another 99 example files carry `"status": "drawn"`. The word stays in the data. `site/src/components/Code.astro:5` turns it into the badge "not run yet", and `site/src/components/CrossView.astro:37` and `:41` add "comes with 0.1" or "is planned". Those three lines are the fix for the drawn cells.

## Lines a review added

A code review on the marketing side found six more lines. Each one gates a page on a status or colours a status badge.

- `site/src/styles/site.css:143`: `.status.shipped, .status.ships-first { color: var(--then); }`
- `site/src/pages/functions/[name]/index.astro:36`: `` <p class="lede">You give it {fn.takes}. You get back {fn.gives}.{shipped && <> Every answer carries a probability, and <code>--details</code> prints it.</>}</p> ``. The `shipped` gate hides the probability sentence on the recognize and relate pages.
- `site/src/pages/[surface]/index.astro:21`: `const headline = shipped`. The headline splits on the Bash status.
- `site/src/pages/[surface]/index.astro:24`: `const when = surface.release && !shipped`
- `site/src/pages/[surface]/index.astro:31`: `{!shipped && (`. It opens the note that says the code has not run.
- `site/src/pages/[surface]/index.astro:77`: `))}. {shipped`. It picks between the two "lands here" sentences at lines 78 and 79.

## Done when

- No page on the built site shows preview, planned, beta, ships first, not run yet, at launch, works today, comes with a release, or not shipped as a status.
- A version number, where a page names one, is the same plain number for every surface.
- The mktg repo checks its own copy with `scripts/check-no-staging.py`. Its patterns may serve as a site check.
