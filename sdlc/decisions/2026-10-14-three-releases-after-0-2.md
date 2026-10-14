# Three releases after 0.2

Ian, 2026-10-13: split the work planned after 0.2 into three releases. On 2026-10-14 he asked the ThinkThen PM to create 0.3 and 0.4.

- 0.3 owns your answers: the answer database with the full future schema, the toolbox and `run`, one-tool MCP, spreadsheets if the writer prototype passes, and publishing.
- 0.4 grades and improves: curation, evals and the ranking setting.
- 0.5 serves and structures: the proxy and document structure.

Condition: the 0.3 database is designed for curation from the start. A routine test runs every planned eval use as SQL over the 0.3 tables.

Ian, 2026-10-14: libraries and MCP get only the ten functions and `run`. The toolbox, answers, curation, evals and admin stay in Rust and the command line.

Ian, 2026-10-14: curation is fixing data. With labels, find where models and labels disagree. Without labels, a model gives first answers and the queue puts the likely-wrong ones first: low confidence, then disagreement between models. Datasets are managed through the command line and, later, a UI.

Why: three small releases ship sooner, and each uses what the one before built.

Replaces: 0.3 as the proxy release. The proxy moves to 0.5.
