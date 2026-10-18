# Draft PR description for approval

ThinkThen is ten functions on top of Jev, TypeSafe's System One model, or any model like it. System One models return probabilities. Your code decides what to do with them. ThinkThen makes these models simple to call from any language or database. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

The extension is free and MIT licensed; hosted models charge per token, and local models are free to use on your own computer. This submission targets DuckDB 1.5.6 and 2.0.

A live judgment sends the question and text to the selected backend. The default is TypeSafe. Set `THINKTHEN_API_KEY` before starting DuckDB for a hosted call. [Install and SQL examples](https://thinkthen.dev/databases/duckdb/) show how to read typed values, probabilities and failures.
