# ThinkThen 0.3

The product design for ThinkThen 0.3, kept by the marketing and product lead with Ian. These files say what 0.3 should be and why. They hold no status. When a design settles, the thinkthen PM turns it into issues, experiments and tickets in this repo's [`sdlc/`](../../), and those records own the work from then on.

## The release

0.3 makes the SDK as good as possible: new functions, plus a proxy that runs the functions centrally, a toolbox of named tools, and an MCP server an agent can learn in one tool call. 0.2 ships first. Marketing covers only 0.2 until 0.3 ships.

Ian split this design into three releases on 2026-10-14. 0.3, "own your answers": the answer database with its full schema, toolboxes and `run`, MCP as one tool, spreadsheets and publishing. 0.4, "grade and improve": curation, evals and the ranking setting. 0.5, "serve and structure": the proxy and document structure. The files below keep their 0.3 names; each PRD names its release.

## Why, and for whom (Ian, 2026-10-13)

- **Ian owns and drives ThinkThen.** The goals, in order: make the SDK as good as possible (0.2, then 0.3), then win adoption. The platform story comes after that.
- **Traction is the measure.** People using it, and the thought leadership that use triggers. The two indicators are GitHub stars and YouTube views.
- **Design everything first.** Ian is not ordering or timing 0.3 yet; how it gets built matters least.
- **Thought leadership lines.** "Rent your models, own your decisions" is right. Ian wants about twenty such lines, on control, ownership and data sovereignty.
- **The 0.3 customer** is the builder: a developer or power user, usually one person, who uses decision models and wants sovereignty and control over their models and their data. Teams are the long-term goal, not the 0.3 customer.
- **The strategy.** Become the SDK people use for System One models: start simple with the ten functions, then show that the same code runs on Jev today and on any other model tomorrow. Hosted and enterprise come later.
- **Every idea starts with what's in it for the customer.** Each 0.3 design file leads with it, and each claim gets the proof the 0.2 docs list has: a measured number or a demonstrated behavior.
- **The vision the docs may state.** Your answers are stored for you in a database, SQLite or Postgres, and you can harvest them to curate your own data and fine-tune your own models. A curation interface is part of the future product; send us your requests. 0.3 itself ships proxying and caching, not curation.
- **The proxy is not a new idea in the SDK.** The SDK already fronts any model. The proxy does the same on a server and adds one central choke point for controlling rollouts to production. Fronting any decision model's code is a front-and-center pitch, after the simplicity of the ten functions.
- **Excel is a must.** It is popular, so 0.3 does it. See "Spreadsheets" below.
- **Talks are not tied to releases.** 0.3 will get its own talk. Ian wants many blog articles, and a full article, doc and deck on SQL once 0.2's quality makes him confident to promote it; that needs better data on the SQL work.

| File | What it is | State of the design |
| --- | --- | --- |
| [proxy.md](proxy.md) | The proxy: two doors, the ThinkThen backend type, what runs where, streaming, SQLite, no access control yet | Settled enough for the PM |
| [toolbox.md](toolbox.md) | Tools replace questions. A self-describing toolbox folder. `run` as the eleventh function in every library | Folder layout and self-description still to design |
| [mcp.md](mcp.md) | MCP redesigned as one tool that takes a command, invoking the ten functions and `run` | Two small choices left |
| [data.md](data.md) | The cache grows into the answer database, in the SDK first, and the command line for managing it | Rulings recorded; the data model and commands to design with Ian |
| [functions.md](functions.md) | New functions: segment and chunk into a document graph, compact, and the ranking dial | Rulings recorded; experiments to design |
| [evals.md](evals.md) | The full eval design, paused, and a modest slice that an experiment can admit into 0.3 | The slice needs its experiment |
| [prd/](prd/) | Product requirement documents built on these files. PRD 00 sets the use cases, the jobs, and one shape for the command line and every library | PRD 00 drafted; five questions for Ian |

Out of 0.3: the user interface, the hosted service, the enterprise edition and access control.

## How ideas move

1. **Capture.** Ian's raw ideas land in his notes and get sorted by product or kept as ideas.
2. **Shape.** A product idea for a release becomes a file in this folder, `sdlc/planning/<version>/`, worked out with Ian.
3. **Hand off.** When a file settles, the lead sends the thinkthen PM a message. The PM designs the tickets and the prototypes or experiments that reduce each risk. Experiments run in a separate experiment repo.
4. **Build.** Tickets in [`sdlc/`](../../) own the work. This folder is not updated with progress.

## Risks to retire before tickets

- **Streaming through the function door.** A prototype pipes a large file through `filter` and `rank` on a local proxy and compares the time against a direct run.
- **The one-tool MCP.** A prototype checks that an agent can discover and call functions and tools through one tool and its help.
- **The toolbox self-description.** A fresh agent given only a toolbox folder should be able to use every tool correctly.
- **Evals in 0.3.** The experiment in `evals.md`.

## A trial-matching project is not tied to 0.3

A private trial-matching project comes first in Ian's priorities, and it may use ThinkThen tools for classifier-based retrieval: the third kind after keyword and semantic search, for deduplication, disambiguation and finding the right ontology concept. Ian, 2026-10-12: that work does not depend on 0.3, and 0.3 is not shaped around it. Its earlier asks are folded into `toolbox.md`, `proxy.md` and `evals.md`.

## Ian's notes, 2026-10-12

- **First, understand the whole product.** Before ordering 0.3 work, settle what ThinkThen is, what each part is for and why. The order shakes out later.
- **Toolboxes are local only.** See `toolbox.md`. The proxy serves functions, never tools.
- **Segment, compact and chunk all need prediction.** Segmentation exists because you do not know where to cut. Chunking may be a tool, but sentence boundaries can be a classification. Compaction keeps what is salient for a question, and salience is a classification, not code chopping text. If segmentation needs no classifier at all, that is itself a finding. Cleaning Markdown with Jev is an idea that does not work well yet.
- **Excel is a new tool.** Annotate for spreadsheets: name the input columns and the output columns, the function, the question and the choices, and ThinkThen writes each answer into its cell. It is grep for spreadsheets. Dry run first, then write in place, after a backup, or to a new file.
- **Small models on the proxy.** The docs may promise them (2026-10-13). Can the proxy serve small non-chat models, such as Typed Decision on EmbeddingGemma or GLiNER? Open: whether they can handle System One decisions at all.
- **Awesome ThinkThen.** A list of tested model guides. For each model, such as Liquid d1, Microsoft Decision-1 or OpenAI's Decisions API, a guide shows how to use it with ThinkThen locally or hosted, with the test date and the timings found. The list's README points to every guide. Ian, 2026-10-13: a separate initiative on 0.2, started now in parallel with 0.3. Each model gets its own folder with scripts, materials and a README. Bugs it finds go into 0.3.
- **Choose whether the cache follows the model.** Today the model is part of every saved answer's key, so a new model asks everything again. Make it a setting: change models and start fresh, or change models and keep reusing the old answers. Verified on 2026-10-12 that answers are saved per question: the same four questions replayed from one recording whether packed together or sent one, two or three per request, and a fifth new record was the only miss.
- **Evals need a whole strategy.** audit and diff are useful small utilities today. Evals, benchmarks, curation and GEPA need one design that covers them all. The cases file in `evals.md` is the seed of curation and benchmarking.

## Spreadsheets (Ian, 2026-10-13)

- **The user is an agent working on a local file.** Excel files are local, so an agent pipes them through the ThinkThen command line and gets a file back. That is the 0.3 shape.
- **Google Sheets only if it stays simple.** Ian does not yet know how Sheets add-ons work. The lead's reading: a Sheets add-on is an Apps Script that calls a web address, so it needs a proxy reachable from Google's servers and a key stored in the sheet's script settings. That is a hosted-proxy feature, not a 0.3 one. In 0.3 a Sheets user downloads the sheet as Excel or CSV, runs the tool, and uploads the result.
- **Results go to a new file by default** (2026-10-13), for Excel and CSV alike, as the least surprise. Writing into the existing file must be asked for, and makes a backup first.
