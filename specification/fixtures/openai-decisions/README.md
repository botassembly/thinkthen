# OpenAI Decisions text exchanges

These request and response files are the actual body pairs from the bounded 2026-10-06 probe, promoted unchanged. They contain no headers. The adapter tests correlate each response against its own request, including the expanded predicates and the complete 100-choice distribution.

The whole probe made 30 attempts: 28 successes and two HTTP 400 cardinality diagnostics. Successful replies reported `gpt-6-luna`, 11,728 input tokens across the probe and zero output tokens. Failed replies omitted usage; their charge remains unreported. Diagnostic maxima of 200 questions and 255 choices do not prove accepted exact boundaries. Batching changed probabilities, and repeated samples give no determinism guarantee. This folder establishes no image or full backend qualification.

Invalid replies, refusals, retry statuses, cancellation and storage isolation use owned fake-key loopback responses in the existing public tests. Those canned responses are test inputs, not live observations.
